use common::read_file_as_string;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static INPUT: Lazy<String> =
    Lazy::new(|| read_file_as_string("data/day11.txt").expect("Failed to load input"));

struct Rack {
    outputs: HashMap<String, Vec<String>>,
}

impl Rack {
    fn new(input: &str) -> Self {
        let mut outputs = HashMap::new();
        for line in input.lines() {
            let parts1 = line.split(':').collect::<Vec<&str>>();
            let parts2 = parts1[1]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            outputs.insert(parts1[0].to_string(), parts2.to_vec());
        }
        Self { outputs }
    }

    fn count_paths(&self, from: &str, to: &str) -> usize {
        let mut path = Vec::new();
        Self::count_paths_inner(self, from, to, &mut path)
    }

    fn count_paths_inner(&self, from: &str, to: &str, path: &mut Vec<String>) -> usize {
        if from == to {
            1
        } else {
            path.push(from.to_string());
            let sum = self
                .outputs
                .get(from)
                .unwrap_or(&Vec::new())
                .iter()
                .filter_map(|s| (!path.contains(s)).then_some(self.count_paths_inner(s, to, path)))
                .sum();
            path.pop();
            sum
        }
    }
}

pub fn part1(input: &str) -> usize {
    let rack = Rack::new(input);
    rack.count_paths("you", "out")
}

pub fn part2(_input: &str) -> usize {
    todo!("day11 - part1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(part1(input), 5);
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 534);
    }

    #[test]
    fn test_part2() {
        todo!("day11 - test - part2")
    }
}
