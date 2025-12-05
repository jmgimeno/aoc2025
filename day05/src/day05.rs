use common::read_file_as_string;
use once_cell::sync::Lazy;
use std::str::FromStr;

pub static INPUT: Lazy<DB> = Lazy::new(|| {
    let input = read_file_as_string("data/day05.txt").expect("Failed to load input");
    input.parse().expect("Failed to parse input")
});

#[derive(Debug)]
pub struct DB {
    // ranges are non-overlapping and sorted
    ranges: Vec<(u64, u64)>,
    // available is sorted
    available: Vec<u64>,
}

impl FromStr for DB {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut empty_line = false;
        let mut ranges = Vec::new();
        let mut available = Vec::new();
        for line in s.lines() {
            if line.is_empty() {
                empty_line = true;
                continue;
            }
            if empty_line {
                available.push(line.parse::<u64>().map_err(|e| e.to_string())?);
            } else {
                let split = line.split("-");
                let begin = split
                    .clone()
                    .nth(0)
                    .unwrap()
                    .parse::<u64>()
                    .map_err(|e| e.to_string())?;
                let end = split
                    .clone()
                    .nth(1)
                    .unwrap()
                    .parse::<u64>()
                    .map_err(|e| e.to_string())?;
                ranges.push((begin, end));
            }
        }
        available.sort_unstable();
        Ok(DB {
            ranges: non_overlapping_and_sorted(ranges),
            available,
        })
    }
}

fn non_overlapping_and_sorted(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_unstable();
    let mut current = sorted_ranges[0];
    let mut non_averlapping_ranges = Vec::new();
    for next in sorted_ranges.iter().skip(1) {
        // current.0 <= next.0 because sorted
        if next.0 <= current.1 {
            // current overlaps with range
            current.1 = u64::max(current.1, next.1);
        } else {
            non_averlapping_ranges.push(current);
            current = *next;
        }
    }
    non_averlapping_ranges.push(current);
    non_averlapping_ranges
}

impl DB {
    fn fresh_ingredients(&self) -> usize {
        let mut i = 0;
        let mut j = 0;
        let mut fresh = 0;
        while i < self.available.len() && j < self.ranges.len() {
            let at = self.available[i];
            let (begin, end) = self.ranges[j];
            if at < begin {
                // before, so we can discard at
                i += 1;
            } else if end < at {
                // after, we can discard the range
                j += 1;
            } else {
                // inside, we count it
                fresh += 1;
                i += 1;
            }
        }
        fresh
    }

    fn total_fresh(&self) -> u64 {
        self.ranges.iter().map(|(begin, end)| end - begin + 1).sum()
    }
}

// 548 too low
pub fn part1(input: &DB) -> usize {
    input.fresh_ingredients()
}

pub fn part2(input: &DB) -> u64 {
    input.total_fresh()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exaple_part1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(part1(&DB::from_str(input).unwrap()), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 563);
    }

    #[test]
    fn test_exaple_part2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(part2(&DB::from_str(input).unwrap()), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 338693411431456);
    }
}
