use common::read_file_as_string;
use once_cell::sync::Lazy;
use std::str::FromStr;

static INPUT: Lazy<String> =
    Lazy::new(|| read_file_as_string("data/day02.txt").expect("Failed to load input"));

#[derive(Debug, PartialEq)]
struct InvalidIP {
    root: u32,
}

impl InvalidIP {
    fn new(root: u32) -> Self {
        Self { root }
    }

    fn to_ip(&self) -> u64 {
        let s = self.root.to_string();
        let ss = format!("{}{}", s, s);
        ss.parse().unwrap()
    }

    fn next_root(ip: u64) -> u32 {
        let s = ip.to_string();
        if s.len() == 1 {
            1
        } else if s.len() % 2 == 0 {
            let hight = s[..s.len() / 2].parse().unwrap();
            let low = s[s.len() / 2..].parse().unwrap();
            if hight >= low {
                hight
            } else {
                hight + 1
            }
        } else {
            10_u32.pow(s.len() as u32 / 2)
        }
    }

    fn previous_root(ip: u64) -> u32 {
        let s = ip.to_string();
        if s.len() == 1 {
            0 // not quite but seems to work
        } else if s.len() % 2 == 0 {
            let hight = s[..s.len() / 2].parse().unwrap();
            let low = s[s.len() / 2..].parse().unwrap();
            if hight > low {
                hight - 1
            } else {
                hight
            }
        } else {
            10_u32.pow(s.len() as u32 / 2) - 1
        }
    }
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
        let min = parts.next().unwrap().parse().unwrap();
        let max = parts.next().unwrap().parse().unwrap();
        assert!(min <= max, "Invalid range: {}-{}", min, max);
        Ok(Range { min, max })
    }
}

impl Range {
    fn invalids(&self) -> Vec<InvalidIP> {
        let mut result = Vec::new();
        let first = InvalidIP::next_root(self.min);
        let last = InvalidIP::previous_root(self.max);
        for root in first..=last {
            let invalid_ip = InvalidIP::new(root);
            assert!(self.min <= invalid_ip.to_ip() && invalid_ip.to_ip() <= self.max, "{} is not in range {:?}", invalid_ip.to_ip(), self);
            result.push(invalid_ip);
        }
        result
    }
}

fn parse_ranges(input: &str) -> Vec<Range> {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

fn main() {
    println!("Part1: {}", part1(&INPUT));
    println!("Part2: {}", part2(&INPUT));
}

fn part1(input: &str) -> u64 {
    parse_ranges(input).iter().flat_map(|r| r.invalids()).map(|i| i.to_ip()).sum()
}

fn part2(_input: &str) -> usize {
    todo!("Not yet implemented")
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
    fn test_invalids() {
        assert_eq!(Range { min: 11, max: 22 }.invalids(), vec![InvalidIP::new(1), InvalidIP::new(2)]);
        assert_eq!(Range { min: 95, max: 115 }.invalids(), vec![InvalidIP::new(9)]);
        assert_eq!(Range { min: 998, max: 1012 }.invalids(), vec![InvalidIP::new(10)]);
        assert_eq!(Range { min: 1188511880, max: 1188511890 }.invalids(), vec![InvalidIP::new(11885)]);
        assert_eq!(Range { min: 222220, max: 222224 }.invalids(), vec![InvalidIP::new(222)]);
        assert_eq!(Range { min: 1698522, max: 1698528 }.invalids(), vec![]);
        assert_eq!(Range { min: 446443, max: 446449 }.invalids(), vec![InvalidIP::new(446)]);
        assert_eq!(Range { min: 38593856, max: 38593862 }.invalids(), vec![InvalidIP::new(3859)]);
        assert_eq!(Range { min: 565653, max: 565659 }.invalids(), vec![]);
        assert_eq!(Range { min: 824824821, max: 824824827 }.invalids(), vec![]);
        assert_eq!(Range { min: 2121212118, max: 2121212124 }.invalids(), vec![]);
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
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 20);
    }
}
