use bit_set::BitSet;
use common::read_file_as_lines;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static INPUT: Lazy<Vec<String>> =
    Lazy::new(|| read_file_as_lines("data/day07.txt").expect("Failed to load input"));

pub fn part1(input: &[String]) -> usize {
    let first_ray = &input[0].find("S").expect("Missing first ray");
    let mut rays = BitSet::with_capacity(input[0].len());
    rays.insert(*first_ray);
    let mut total_splits = 0;
    for line in input.iter().skip(1) {
        let splitters = line
            .chars()
            .enumerate()
            .filter_map(|(i, c)| (c == '^').then_some(i))
            .collect::<BitSet<_>>();
        if splitters.is_empty() {
            continue;
        }
        let split_rays = rays.intersection(&splitters).collect::<BitSet<_>>();
        total_splits += split_rays.len();
        let new_rays = split_rays
            .iter()
            .flat_map(|ray| [ray - 1, ray + 1])
            .collect::<BitSet<usize>>();
        let old_rays = rays.difference(&split_rays).collect::<BitSet<_>>();
        rays = new_rays.union(&old_rays).collect::<BitSet<_>>();
    }
    total_splits
}

#[allow(dead_code)]
pub fn part1_slower(input: &[String]) -> usize {
    let first_ray = &input[0].find("S").expect("Missing first ray");
    let mut rays = BitSet::with_capacity(input[0].len());
    rays.insert(*first_ray);
    let mut total_splits = 0;
    for line in input.iter().skip(1) {
        let snapshot = rays.clone();
        for ray in snapshot.iter() {
            if line.chars().nth(ray) == Some('^') {
                rays.remove(ray);
                rays.insert(ray - 1);
                rays.insert(ray + 1);
                total_splits += 1;
            }
        }
    }
    total_splits
}

#[allow(dead_code)]
pub fn part2_slower(input: &[String]) -> usize {
    let first_ray = input[0].find("S").expect("Missing first ray");
    let mut timelines = HashMap::new();
    timelines.insert(first_ray, 1);

    for line in input.iter().skip(1) {
        let mut next: HashMap<usize, usize> = HashMap::new();

        for (&ray, &count) in timelines.iter() {
            if line.chars().nth(ray) == Some('.') {
                *next.entry(ray).or_insert(0) += count;
            } else {
                *next.entry(ray - 1).or_insert(0) += count;
                *next.entry(ray + 1).or_insert(0) += count;
            }
        }
        timelines = next;
    }
    timelines.values().sum()
}

#[allow(dead_code)]
pub fn part1_slowest(input: &[String]) -> usize {
    let first_ray = input[0].find("S").expect("Missing first ray");
    let mut timelines = vec![false; input[0].len()];
    timelines[first_ray] = true;
    let mut total_splits = 0;
    for line in input.iter().skip(1) {
        for i in 0..timelines.len() {
            if !timelines[i] {
                continue;
            }
            if line.chars().nth(i) == Some('^') {
                timelines[i - 1] = true;
                timelines[i + 1] = true;
                timelines[i] = false;
                total_splits += 1;
            }
        }
    }
    total_splits
}

pub fn part2(input: &[String]) -> usize {
    let first_ray = input[0].find("S").expect("Missing first ray");
    let mut timelines = vec![0; input[0].len()];
    timelines[first_ray] = 1;

    for line in input.iter().skip(1) {
        for i in 0..timelines.len() {
            if timelines[i] == 0 {
                continue;
            }
            if line.chars().nth(i) == Some('^') {
                timelines[i - 1] += timelines[i];
                timelines[i + 1] += timelines[i];
                timelines[i] = 0;
            }
        }
    }
    timelines.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 1703);
    }

    #[test]
    fn test_part1_slowest() {
        assert_eq!(part1_slowest(&INPUT), 1703);
    }

    #[test]
    fn test_part1_slower() {
        assert_eq!(part1_slower(&INPUT), 1703);
    }

    #[test]
    fn test_example_part2() {
        let input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();
        assert_eq!(part2(&input), 40);
    }

    #[test]
    fn test_part2_slower() {
        assert_eq!(part2_slower(&INPUT), 171692855075500);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 171692855075500);
    }
}
