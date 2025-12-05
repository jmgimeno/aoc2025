use common::read_file_as_string;
use once_cell::sync::Lazy;

static INPUT: Lazy<String> =
    Lazy::new(|| read_file_as_string("data/{{project-name}}.txt").expect("Failed to load input"));

fn main() {
    println!("Part1: {}", part1(&INPUT));
    println!("Part2: {}", part2(&INPUT));
}

fn part1(_input: &str) -> usize {
    todo!("{{project-name}} - part1")
}

fn part2(_input: &str) -> usize {
    todo!("{{project-name}} - part1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 0);
    }
}