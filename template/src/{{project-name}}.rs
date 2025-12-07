use common::read_file_as_string;
use once_cell::sync::Lazy;

pub static INPUT: Lazy<String> =
    Lazy::new(|| read_file_as_string("data/{{project-name}}.txt").expect("Failed to load input"));

pub fn part1(_input: &str) -> usize {
    todo!("{{project-name}} - part1")
}

pub fn part2(_input: &str) -> usize {
    todo!("{{project-name}} - part1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        todo!("{{project-name}} - test - part1")
    }

    #[test]
    fn test_part2() {
        todo!("{{project-name}} - test - part2")
    }
}