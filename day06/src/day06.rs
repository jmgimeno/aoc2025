use common::read_file_as_lines;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::str::FromStr;

pub static INPUT: Lazy<Vec<String>> =
    Lazy::new(|| read_file_as_lines("data/day06.txt").expect("Failed to load input"));

enum Operation {
    Sum,
    Mult,
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

pub fn part1_iterative(input: &[String]) -> u64 {
    let numbers = input[..input.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    let ops = input[input.len() - 1]
        .split_whitespace()
        .map(|x| x.parse::<Operation>().unwrap())
        .collect::<Vec<_>>();
    let mut total = 0;
    for i in 0..ops.len() {
        let args = numbers.iter().map(|row| row[i]).collect::<Vec<_>>();
        let result = match ops[i] {
            Operation::Sum => args.iter().sum::<u64>(),
            Operation::Mult => args.iter().product::<u64>(),
        };
        total += result;
    }
    total
}

pub fn part2_iterative(input: &[String]) -> u64 {
    let numbers = &input[..input.len() - 1];
    assert!(numbers.iter().all(|row| row.len() == numbers[0].len()));
    let len = numbers[0].len();
    let transposed_numbers = (0..len)
        .map(|i| {
            numbers
                .iter()
                .map(|row| row.chars().nth(i).unwrap())
                .collect::<String>()
                .trim()
                .to_string()
        })
        .collect::<Vec<_>>();
    let mut total = 0;
    let mut it = transposed_numbers.iter();
    let mut args = Vec::new();
    for op in input[input.len() - 1]
        .split_whitespace()
        .map(|x| x.parse::<Operation>().unwrap())
    {
        loop {
            let empty = "".to_string();
            let s = it.next().unwrap_or(&empty);
            if s.is_empty() {
                break;
            }
            args.push(s.parse::<u64>().unwrap());
        }
        total += match op {
            Operation::Sum => args.iter().sum::<u64>(),
            Operation::Mult => args.iter().product::<u64>(),
        };
        args.clear();
    }
    total
}

pub fn part2_strings(input: &[String]) -> u64 {
    let transposed_numbers = (0..input[0].len())
        .map(|i| {
            input
                .iter()
                .take(input.len() - 1)
                .map(|row| row.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .map(|num_str| num_str.trim().parse::<u64>().unwrap_or(0))
        .chunk_by(|n| *n != 0); // no column has value zero, so we use it to mark separators
    let ops = input[input.len() - 1]
        .split_whitespace()
        .map(|x| x.parse::<Operation>().unwrap());
    transposed_numbers
        .into_iter()
        .filter_map(|(has_numbers, numbers)| has_numbers.then_some(numbers))
        .zip(ops)
        .map(|(numbers, op)| match op {
            Operation::Sum => numbers.sum::<u64>(),
            Operation::Mult => numbers.product::<u64>(),
        })
        .sum::<u64>()
}

pub fn part1(input: &[String]) -> u64 {
    let mut numbers = input[..input.len() - 1]
        .iter()
        .map(|line| line.split_whitespace().map(|x| x.parse::<u64>().unwrap()))
        .collect::<Vec<_>>();
    let ops = input[input.len() - 1]
        .split_whitespace()
        .map(|x| x.parse::<Operation>().unwrap());
    ops.map(|op| {
        let args = numbers.iter_mut().map(|row| row.next().unwrap());
        match op {
            Operation::Sum => args.sum::<u64>(),
            Operation::Mult => args.product::<u64>(),
        }
    })
    .sum::<u64>()
}

pub fn part2(input: &[String]) -> u64 {
    let transposed_numbers = (0..input[0].len())
        .map(|i| {
            input
                .iter()
                .take(input.len() - 1)
                .map(|row| row.chars().nth(i).unwrap())
                .fold(0, |acc, c| {
                    if c.is_digit(10) {
                        acc * 10 + (c as u64 - '0' as u64)
                    } else {
                        acc
                    }
                })
        })
        .chunk_by(|n| *n != 0); // no column has value zero, so we use it to mark separators
    let ops = input[input.len() - 1]
        .split_whitespace()
        .map(|x| x.parse::<Operation>().unwrap());
    transposed_numbers
        .into_iter()
        .filter_map(|(has_numbers, numbers)| has_numbers.then_some(numbers))
        .zip(ops)
        .map(|(numbers, op)| match op {
            Operation::Sum => numbers.sum::<u64>(),
            Operation::Mult => numbers.product::<u64>(),
        })
        .sum::<u64>()
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
    fn test_example_part2() {
        let input = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
        assert_eq!(part2(&input), 3263827);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 7450962489289);
    }

    #[test]
    fn test_example_part2_iterative() {
        let input = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +  ",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
        assert_eq!(part2_iterative(&input), 3263827);
    }

    #[test]
    fn test_part1_iterative() {
        assert_eq!(part1_iterative(&INPUT), 4405895212738);
    }

    #[test]
    fn test_part2_iterative() {
        assert_eq!(part2_iterative(&INPUT), 7450962489289);
    }

    #[test]
    fn test_part2_strings() {
        assert_eq!(part2_strings(&INPUT), 7450962489289);
    }
}
