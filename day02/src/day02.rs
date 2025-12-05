use std::collections::HashSet;
use common::read_file_as_string;
use once_cell::sync::Lazy;
use std::str::FromStr;

pub static INPUT: Lazy<String> =
    Lazy::new(|| read_file_as_string("data/day02.txt").expect("Failed to load input"));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct InvalidIP {
    root: u32,
}

impl InvalidIP {
    fn new(root: u32) -> Self {
        Self { root }
    }

    fn to_ip(&self) -> u64 {
        let root64 = self.root as u64;
        let digits = num_digits_u64(root64);
        let pow = 10u64.pow(digits as u32);
        root64 * pow + root64
    }

    fn next_root(ip: u64) -> u32 {
        let len = num_digits_u64(ip);
        if len == 1 {
            1
        } else if len % 2 == 0 {
            let half = len / 2;
            let pow = 10u64.pow(half as u32);
            let high = (ip / pow) as u32;
            let low = (ip % pow) as u32;
            if high >= low {
                high
            } else {
                high + 1
            }
        } else {
            10_u32.pow((len / 2) as u32)
        }
    }

    fn previous_root(ip: u64) -> u32 {
        let len = num_digits_u64(ip);
        if len == 1 {
            0 // not quite but seems to work for tests
        } else if len % 2 == 0 {
            let half = len / 2;
            let pow = 10u64.pow(half as u32);
            let high = (ip / pow) as u32;
            let low = (ip % pow) as u32;
            if high > low {
                high - 1
            } else {
                high
            }
        } else {
            10_u32.pow((len / 2) as u32) - 1
        }
    }
}

fn num_digits_u64(mut n: u64) -> usize {
    if n == 0 {
        return 1;
    }
    let mut d = 0;
    while n > 0 {
        d += 1;
        n /= 10;
    }
    d
}

#[derive(Debug)]
struct Range {
    min: u64,
    max: u64,
}

impl FromStr for Range {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let min_str = parts.next().ok_or_else(|| "Missing min".to_string())?;
        let max_str = parts.next().ok_or_else(|| "Missing max".to_string())?;
        let min = min_str.trim().parse::<u64>().map_err(|e| e.to_string())?;
        let max = max_str.trim().parse::<u64>().map_err(|e| e.to_string())?;
        if min <= max {
            Ok(Range { min, max })
        } else {
            Err(format!("Invalid range: {}-{}", min, max))
        }
    }
}

impl Range {
    fn invalids_part1(&self) -> Vec<InvalidIP> {
        let mut result = Vec::new();
        let first = InvalidIP::next_root(self.min);
        let last = InvalidIP::previous_root(self.max);
        if first > last {
            return result;
        }
        for root in first..=last {
            let invalid_ip = InvalidIP::new(root);
            let ip = invalid_ip.to_ip();
            if self.min <= ip && ip <= self.max {
                result.push(invalid_ip);
            }
        }
        result
    }

    fn invalids_part2(&self) -> Vec<u64> {
        let mut set = HashSet::new();
        let min_len = num_digits_u64(self.min);
        let max_len = num_digits_u64(self.max);

        for len in min_len..=max_len {
            for chunk_size in 1..=(len / 2) {
                if len % chunk_size != 0 { continue; }
                let num_chunks = len / chunk_size;

                let chunk_start = 10u128.pow((chunk_size - 1) as u32);
                let chunk_end = 10u128.pow(chunk_size as u32);

                // geometric multiplier: 1 + chunk_end + chunk_end^2 + ... + chunk_end^(num_chunks-1)
                // geom sets to 1 digits 0, chunk_size, chunk_size*2, chunk_size*3, ... chunk_size*(num_chunks-1)
                let geom = (chunk_end.pow(num_chunks as u32) - 1) / (chunk_end - 1);

                for base in chunk_start..chunk_end {
                    let val128 = base * geom;
                    if val128 > u64::MAX as u128 { break; }
                    let val = val128 as u64;
                    if val >= self.min && val <= self.max {
                        set.insert(val);
                    }
                }
            }
        }
        set.into_iter().collect()
    }
}

fn parse_ranges(input: &str) -> Vec<Range> {
    input
        .split(',')
        .map(|l| l.trim().parse().unwrap())
        .collect()
}

pub fn part1(input: &str) -> u64 {
    parse_ranges(input)
        .iter()
        .flat_map(|r| r.invalids_part1())
        .map(|i| i.to_ip())
        .sum()
}

pub fn part2(input: &str) -> u64 {
    parse_ranges(input)
        .iter()
        .flat_map(|r| r.invalids_part2())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_root() {
        assert_eq!(InvalidIP::next_root(1), 1);
        assert_eq!(InvalidIP::next_root(21), 2);
        assert_eq!(InvalidIP::next_root(22), 2);
        assert_eq!(InvalidIP::next_root(23), 3);
        assert_eq!(InvalidIP::next_root(1211), 12);
        assert_eq!(InvalidIP::next_root(1234), 13);
        assert_eq!(InvalidIP::next_root(123), 10);
        assert_eq!(InvalidIP::next_root(12345), 100);
    }

    #[test]
    fn test_previous_root() {
        assert_eq!(InvalidIP::previous_root(1), 0);
        assert_eq!(InvalidIP::previous_root(21), 1);
        assert_eq!(InvalidIP::previous_root(22), 2);
        assert_eq!(InvalidIP::previous_root(23), 2);
        assert_eq!(InvalidIP::previous_root(1211), 11);
        assert_eq!(InvalidIP::previous_root(1234), 12);
        assert_eq!(InvalidIP::previous_root(123), 9);
        assert_eq!(InvalidIP::previous_root(12345), 99);
    }

    #[test]
    fn test_example_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                          1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                          824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(input), 1227775554);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 30599400849);
    }

    #[test]
    fn test_example_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                          1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                          824824821-824824827,2121212118-2121212124";
        assert_eq!(part2(input), 4174379265);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 46270373595);
    }
}