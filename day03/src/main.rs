use common::read_file_as_elements;
use once_cell::sync::Lazy;
use std::str::FromStr;

static INPUT: Lazy<Vec<Bank>> =
    Lazy::new(|| read_file_as_elements("data/day03.txt").expect("Failed to load input"));

#[derive(Debug)]
struct Bank {
    batteries: Vec<Battery>,
}

impl FromStr for Bank {
    type Err = <Battery as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let batteries = s
            .chars()
            .map(|c| c.to_string().parse::<Battery>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { batteries })
    }
}

impl Bank {
    fn maximum_joltage(&self, window_len: usize) -> u64 {
        // Number of digits we may remove to end up with `window_len` digits.
        let mut removals_remaining = self.batteries.len() - window_len;
        // initial_removals = self.batteries.len() - window_len
        // Loop invariant (after processing first `i` items):
        //  - `removals_used = initial_removals - removals_remaining`
        //  - `stack.len() == i - removals_used`
        //  - `stack` is the lexicographically largest subsequence obtainable from the
        //    processed prefix consistent with those `removals_used`.
        let mut stack: Vec<u32> = Vec::with_capacity(window_len);

        for battery in &self.batteries {
            let value = battery.joltage;
            // While we can still remove digits and the last digit in `stack` is smaller than
            // the incoming `value`, pop it to make a larger number (greedy choice).
            while removals_remaining > 0 && !stack.is_empty() && *stack.last().unwrap() < value {
                stack.pop();
                removals_remaining -= 1;
            }
            stack.push(value);
        }
        // If we didn't remove enough (or window_len < n), truncate to the desired length.
        if stack.len() > window_len {
            stack.truncate(window_len);
        }

        stack.iter().fold(0u64, |acc, &x| acc * 10 + x as u64)
    }
}

#[derive(Debug)]
struct Battery {
    joltage: u32,
}

impl FromStr for Battery {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            joltage: s.parse()?,
        })
    }
}

fn main() {
    println!("Part1: {}", part1(&INPUT));
    println!("Part2: {}", part2(&INPUT));
}

fn part1(input: &[Bank]) -> u64 {
    input.iter().map(|b| b.maximum_joltage(2)).sum()
}

fn part2(input: &[Bank]) -> u64 {
    input.iter().map(|b| b.maximum_joltage(12)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    fn test_examples_part1(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(input.parse::<Bank>().unwrap().maximum_joltage(2), expected);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 17332);
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn test_examples_part2(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(input.parse::<Bank>().unwrap().maximum_joltage(12), expected);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 172516781546707);
    }
}
