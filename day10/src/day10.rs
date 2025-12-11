use bit_set::BitSet;
use common::read_file_as_elements;
use microlp::{ComparisonOp, OptimizationDirection, Problem};
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
        let initial_lights = BitSet::with_capacity(self.target.len());
        let target = Machine::target_as_bitset(self.target.clone());
        let mut queue = VecDeque::new();
        let mut explored = HashSet::new();
        explored.insert(initial_lights.clone());
        queue.push_back((0, initial_lights));
        while let Some((steps, lights)) = queue.pop_front() {
            if lights == target {
                return steps;
            }
            self.button_wiring
                .iter()
                .filter_map(|wiring| {
                    let mut neighbor = lights.clone();
                    wiring.iter().for_each(|&i| {
                        if neighbor.contains(i) {
                            neighbor.remove(i);
                        } else {
                            neighbor.insert(i);
                        }
                    });
                    if !explored.contains(&neighbor) {
                        explored.insert(neighbor.clone());
                        Some(neighbor)
                    } else {
                        None
                    }
                })
                .for_each(|neighbor| queue.push_back((steps + 1, neighbor)));
        }
        unreachable!("No solution found")
    }

    fn target_as_bitset(value: Vec<bool>) -> BitSet {
        let mut bitset = BitSet::with_capacity(value.len());
        for (i, value) in value.into_iter().enumerate() {
            if value {
                bitset.insert(i);
            }
        }
        bitset
    }

    fn min_steps_to_joltage(&self) -> u32 {
        let mut problem = Problem::new(OptimizationDirection::Minimize);
        let max_presses = self.joltage_requirements.iter().max().unwrap();
        let buttons = self
            .button_wiring
            .iter()
            .map(|_| problem.add_integer_var(1.0, (0, *max_presses as i32)))
            .collect::<Vec<_>>();
        for (i, &target) in self.joltage_requirements.iter().enumerate() {
            let in_buttons = self
                .button_wiring
                .iter()
                .enumerate()
                .filter_map(|(j, wiring)| wiring.contains(&i).then_some((buttons[j], 1.0)))
                .collect::<Vec<_>>();
            problem.add_constraint(in_buttons, ComparisonOp::Eq, target as f64)
        }
        let solution = problem.solve().unwrap();
        solution.objective().round() as u32
    }
}

pub fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(Machine::min_steps_to_target).sum()
}

pub fn part2(machines: &[Machine]) -> u32 {
    machines.iter().map(Machine::min_steps_to_joltage).sum()
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
    fn test_part2_machine1_manual() {
        let mut problem = Problem::new(OptimizationDirection::Minimize);
        let b0 = problem.add_integer_var(1.0, (0, 7));
        let b1 = problem.add_integer_var(1.0, (0, 7));
        let b2 = problem.add_integer_var(1.0, (0, 7));
        let b3 = problem.add_integer_var(1.0, (0, 7));
        let b4 = problem.add_integer_var(1.0, (0, 7));
        let b5 = problem.add_integer_var(1.0, (0, 7));

        problem.add_constraint(&[(b4, 1.0), (b5, 1.0)], ComparisonOp::Eq, 3.0);
        problem.add_constraint(&[(b1, 1.0), (b5, 1.0)], ComparisonOp::Eq, 5.0);
        problem.add_constraint(&[(b2, 1.0), (b3, 1.0), (b4, 1.0)], ComparisonOp::Eq, 4.0);
        problem.add_constraint(&[(b0, 1.0), (b1, 1.0), (b3, 1.0)], ComparisonOp::Eq, 7.0);

        let solution = problem.solve().unwrap();
        println!("Solution = {}", solution.objective());
        println!("b0 = {}", solution[b0]);
        println!("b1 = {}", solution[b1]);
        println!("b2 = {}", solution[b2]);
        println!("b3 = {}", solution[b3]);
        println!("b4 = {}", solution[b4]);
        println!("b5 = {}", solution[b5]);

        assert_eq!(solution.objective() as u32, 10);
    }

    #[test]
    fn test_part2_machine1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = input.parse::<Machine>().unwrap();
        assert_eq!(machine.min_steps_to_joltage(), 10);
    }

    #[test]
    fn test_part2_machine2() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = input.parse::<Machine>().unwrap();
        assert_eq!(machine.min_steps_to_joltage(), 12);
    }

    #[test]
    fn test_part2_machine3() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = input.parse::<Machine>().unwrap();
        assert_eq!(machine.min_steps_to_joltage(), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 17214);
    }
}
