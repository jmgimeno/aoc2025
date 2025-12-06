use std::str::FromStr;
use common::read_file_as_lines;
use once_cell::sync::Lazy;

pub static INPUT: Lazy<Vec<String>> =
    Lazy::new(|| read_file_as_lines("data/day06.txt").expect("Failed to load input"));

enum Operation {
    Sum, Mult
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "+" {
            Ok(Operation::Sum)
        } else if s == "*" {
            Ok(Operation::Mult)
        } else {
            Err(format!("Unknown operation {}", s))
        }
    }
}

pub fn part1(input: &[String]) -> u64 {
    let numbers =
        input[..input.len()-1].iter()
            .map(|line| line.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect())
            .collect::<Vec<Vec<_>>>();
    let ops =
        input[input.len()-1]
            .split_whitespace()
            .map(|x| x.parse::<Operation>().unwrap())
            .collect::<Vec<_>>();
    let mut total = 0;
    for i in 0..ops.len() {
        let args =
            numbers.iter()
                .map(|row| row[i])
                .collect::<Vec<_>>();
        let result = match ops[i] {
            Operation::Sum => args.iter().sum::<u64>(),
            Operation::Mult => args.iter().product::<u64>(),
        };
        total += result;
    }
    total
}

pub fn part2(_input: &[String]) -> usize {
    todo!("day06 - part1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";
        let input = input.lines().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(part1(&input), 4277556);
    }
    
    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 4405895212738);
    }

    #[test]
    fn test_part2() {
        todo!("day06 - part2");
    }
}