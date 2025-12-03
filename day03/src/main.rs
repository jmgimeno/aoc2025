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
    fn maximum_joltage(&self) -> u32 {
        let mut bf = self.batteries[0].joltage;
        let mut max = 0;
        for s in 1..self.batteries.len() {
            let bs = self.batteries[s].joltage;
            max = max.max(bf * 10 + bs);
            if bf < bs {
                bf = bs;
            }
        }
        max
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

fn part1(input: &[Bank]) -> u32 {
    input.iter().map(|b| b.maximum_joltage()).sum()
}

fn part2(input: &[Bank]) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert_eq!("987654321111111".parse::<Bank>().unwrap().maximum_joltage(), 98);
        assert_eq!("811111111111119".parse::<Bank>().unwrap().maximum_joltage(), 89);
        assert_eq!("234234234234278".parse::<Bank>().unwrap().maximum_joltage(), 78);
        assert_eq!("818181911112111".parse::<Bank>().unwrap().maximum_joltage(), 92);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 17332);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 20);
    }
}
