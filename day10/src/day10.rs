use common::read_file_as_elements;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

pub static INPUT: Lazy<Vec<Machine>> =
    Lazy::new(|| read_file_as_elements("data/day10.txt").expect("Failed to load input"));

#[derive(Debug)]
pub struct Machine {
    target: Vec<bool>,
    button_wiring: Vec<Vec<usize>>,
    joltage_requirements: Vec<u32>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        // [.##.] is tha target (. is false, # is true)
        // (3) (1,3) (2) (2,3) (0,2) (0,1) are the button wiring
        // {3,5,4,7} are the joltage_requirements

        // Parse target: [.##.]
        let target_re = Regex::new(r"\[([.#]+)]").unwrap();
        let target_cap = target_re.captures(s).ok_or("Failed to parse target")?;
        let target: Vec<bool> = target_cap[1].chars().map(|c| c == '#').collect();

        // Parse button wiring: (3) (1,3) etc.
        let wiring_re = Regex::new(r"\((\d+(?:,\d+)*)\)").unwrap();
        let mut button_wiring: Vec<Vec<usize>> = Vec::new();

        // Find where joltage requirements start to avoid parsing them as wiring
        let joltage_start = s.find('{').unwrap_or(s.len());
        let wiring_section = &s[..joltage_start];

        for cap in wiring_re.captures_iter(wiring_section) {
            let indices: Vec<usize> = cap[1].split(',').map(|n| n.parse().unwrap()).collect();
            button_wiring.push(indices);
        }

        // Parse joltage requirements: {3,5,4,7}
        let joltage_re = Regex::new(r"\{(\d+(?:,\d+)*)}").unwrap();
        let joltage_cap = joltage_re
            .captures(s)
            .ok_or("Failed to parse joltage requirements")?;
        let joltage_requirements: Vec<u32> = joltage_cap[1]
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Machine {
            target,
            button_wiring,
            joltage_requirements,
        })
    }
}

impl Machine {
    fn min_steps_to_target(&self) -> usize {
        let initial_lights = vec![false; self.target.len()];
        let mut queue = VecDeque::new();
        let mut explored = HashSet::new();
        explored.insert(initial_lights.clone());
        queue.push_back((0, initial_lights));
        while let Some((steps, lights)) = queue.pop_front() {
            if lights == self.target {
                return steps;
            }
            self.button_wiring.iter()
                .filter_map(|wiring| {
                    let mut neighbor = lights.clone();
                    wiring.iter().for_each(|i| neighbor[*i] = !neighbor[*i]);
                    if !explored.contains(&neighbor) {
                        Some(neighbor)
                    } else {
                        None
                    }
                })
                .for_each(|neighbor| queue.push_back((steps + 1, neighbor)));
        }
        unreachable!("No solution found")
    }
}

pub fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(Machine::min_steps_to_target).sum()
}

pub fn part2(_machines: &[Machine]) -> usize {
    todo!("day10 - part1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_machine1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = input.parse::<Machine>().unwrap();
        assert_eq!(machine.min_steps_to_target(), 2);
    }

    #[test]
    fn test_part1_machine2() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = input.parse::<Machine>().unwrap();
        assert_eq!(machine.min_steps_to_target(), 3);
    }

    #[test]
    fn test_part1_machine3() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = input.parse::<Machine>().unwrap();
        assert_eq!(machine.min_steps_to_target(), 2);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 466);
    }

    #[test]
    fn test_part2() {
        todo!("day10 - test - part2")
    }
}
