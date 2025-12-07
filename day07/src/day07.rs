use bit_set::BitSet;
use common::read_file_as_lines;
use once_cell::sync::Lazy;

pub static INPUT: Lazy<Vec<String>> =
    Lazy::new(|| read_file_as_lines("data/day07.txt").expect("Failed to load input"));

pub fn part1(input: &[String]) -> usize {
    let first_ray = &input[0].find("S").expect("Missing first ray");
    let mut rays = BitSet::with_capacity(input[0].len());
    rays.insert(*first_ray);
    let mut total_splits = 0;
    for line in input.iter().skip(1) {
        let splitters = line.chars().enumerate().filter_map(|(i, c)| (c == '^').then_some(i)).collect::<BitSet<_>>();
        if splitters.is_empty() { continue; }
        let splitted_rays = rays.intersection(&splitters).collect::<BitSet<_>>();
        total_splits += splitted_rays.len();
        let new_rays = splitted_rays.iter().flat_map(|ray| [ray - 1, ray + 1]).collect::<BitSet<usize>>();
        let old_rays = rays.difference(&splitted_rays).collect::<BitSet<_>>();
        rays = new_rays.union(&old_rays).collect::<BitSet<_>>();
    }
    total_splits
}

pub fn part2(_input: &[String]) -> usize {
    todo!("day07 - part1")
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
...............".lines().map(|l| l.to_string()).collect::<Vec<_>>();
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 1703);
    }

    #[test]
    fn test_part2() {
        todo!("day07 - test - part2")
    }
}