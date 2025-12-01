use common::read_file_as_string;
use once_cell::sync::Lazy;

static INPUT: Lazy<String> =
    Lazy::new(|| read_file_as_string("data/day01.txt").expect("Failed to load input"));

fn main() {
    println!("Part1: {}", part1(&INPUT));
    println!("Part2: {}", part2(&INPUT));
}

fn part1(input: &str) -> usize {
    input.len()
}

fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 20);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 20);
    }
}