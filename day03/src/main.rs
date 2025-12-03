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
        let mut max = (0..window_len)
            .map(|i| self.batteries[i].joltage)
            .collect::<Vec<_>>();
        for b in &self.batteries[window_len..] {
            max.push(b.joltage);
            let mut removed = false;
            for i in 1..max.len() {
                // I need to find if there is a position in the prefix before i that
                // has a value that is lower than max[i]
                match (0..i).rev().find(|j| max[*j] < max[i]) {
                    Some(j) => {
                        removed = true;
                        max.remove(j);
                        break;
                    }
                    None => (),
                }
            }
            if !removed {
                max.pop();
            }
        }

        max.iter().fold(0, |acc, &x| 10 * acc + x as u64)
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

    #[test]
    fn test_examples_part1() {
        assert_eq!(
            "987654321111111".parse::<Bank>().unwrap().maximum_joltage(2),
            98
        );
        assert_eq!(
            "811111111111119".parse::<Bank>().unwrap().maximum_joltage(2),
            89
        );
        assert_eq!(
            "234234234234278".parse::<Bank>().unwrap().maximum_joltage(2),
            78
        );
        assert_eq!(
            "818181911112111".parse::<Bank>().unwrap().maximum_joltage(2),
            92
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 17332);
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(
            "987654321111111".parse::<Bank>().unwrap().maximum_joltage(12),
            987654321111
        );
        assert_eq!(
            "811111111111119".parse::<Bank>().unwrap().maximum_joltage(12),
            811111111119
        );
        assert_eq!(
            "234234234234278".parse::<Bank>().unwrap().maximum_joltage(12),
            434234234278
        );
        assert_eq!(
            "818181911112111".parse::<Bank>().unwrap().maximum_joltage(12),
            888911112111
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 172516781546707);
    }
}
