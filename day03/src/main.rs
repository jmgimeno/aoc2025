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
    fn maximum_joltage_old(&self) -> u32 {
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

    fn maximum_joltage_step1(&self) -> u32 {
        let mut idxs = [0, 1];
        for i in 2..self.batteries.len() {
            let bf = self.batteries[idxs[0]].joltage;
            let bs = self.batteries[idxs[1]].joltage;
            let bi = self.batteries[i].joltage;
            if bs > bf {
                idxs[0] = idxs[1];
                idxs[1] = i;
            } else if bi > bs {
                idxs[1] = i;
            }
        }
        self.batteries[idxs[0]].joltage * 10 + self.batteries[idxs[1]].joltage
    }

    fn maximum_joltage_step2(&self) -> u32 {
        let mut max = [0, 1].map(|i| self.batteries[i].joltage);
        for b in &self.batteries[2..] {
            let bj = b.joltage;
            if max[1] > max[0] {
                max[0] = max[1];
                max[1] = bj;
            } else if bj > max[1] {
                max[1] = bj;
            }
        }
        max[0] * 10 + max[1]
    }

    fn maximum_joltage_step3(&self) -> u32 {
        let mut max = [0, 1]
            .iter()
            .map(|i| self.batteries[*i].joltage)
            .collect::<Vec<_>>();
        for b in &self.batteries[2..] {
            max.push(b.joltage);
            let mut removed = false;
            for i in 1..max.len() {
                // I need to find if there is a position in the prefix 0..i that
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
        max[0] * 10 + max[1]
    }

    fn maximum_joltage(&self) -> u32 {
        self.maximum_joltage_step4(2)
    }

    fn maximum_joltage_step4(&self, len: usize) -> u32 {
        let mut max =
            (0..len)
            .map(|i| self.batteries[i].joltage)
            .collect::<Vec<_>>();
        for b in &self.batteries[len..] {
            max.push(b.joltage);
            let mut removed = false;
            for i in 1..max.len() {
                // I need to find if there is a position in the prefix 0..i that
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

        max.iter().fold(0, |acc, &x| 10 * acc + x)
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
