use common::read_file_as_lines;
use once_cell::sync::Lazy;
use partitions::PartitionVec;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub static INPUT: Lazy<Vec<String>> =
    Lazy::new(|| read_file_as_lines("data/day08.txt").expect("Failed to load input"));

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Box {
    id: usize,
    x: u32,
    y: u32,
    z: u32,
}

impl Box {
    fn distance(&self, other: Box) -> u64 {
        let x_diff = self.x as i64 - other.x as i64;
        let y_diff = self.y as i64 - other.y as i64;
        let z_diff = self.z as i64 - other.z as i64;
        (x_diff * x_diff + y_diff * y_diff + z_diff * z_diff) as u64
    }
}

fn parse_boxes(input: &[String]) -> Result<Vec<Box>, String> {
    input.iter().enumerate().map(|(id, line)| {
        let mut parts = line.split(',');
        let x = parts.next().ok_or("Missing x")?.parse().map_err(|_| "Invalid x")?;
        let y = parts.next().ok_or("Missing y")?.parse().map_err(|_| "Invalid y")?;
        let z = parts.next().ok_or("Missing z")?.parse().map_err(|_| "Invalid z")?;
        Ok(Box { id, x, y, z })
    }).collect()
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct DistanceTriplet {
    distance: u64,
    b1: Box,
    b2: Box,
}

impl DistanceTriplet {
    fn new(distance: u64, b1: Box, b2: Box) -> Self {
        Self { distance, b1, b2 }
    }
}

fn sorted_distances(boxes: &[Box]) -> BinaryHeap<Reverse<DistanceTriplet>> {
    let mut distances = BinaryHeap::with_capacity(boxes.len() * boxes.len());
    for (i, b_i) in boxes.iter().enumerate() {
        for b_j in boxes[i + 1..].iter() {
            distances.push(Reverse(DistanceTriplet::new(b_i.distance(*b_j), *b_i, *b_j)));
        }
    }
     distances
}

fn circuits(boxes: &[Box]) -> PartitionVec<Box> {
    let mut partitions = PartitionVec::with_capacity(boxes.len());
    for b in boxes {
        partitions.insert(b.id, *b);
    }
    partitions
}

pub fn part1(input: &[String], connections: usize) -> usize {
    let boxes = parse_boxes(input).unwrap();
    let mut distances = sorted_distances(&boxes);
    let mut circuits = circuits(&boxes); // union-find via partitions crate
    for _ in 0..connections {
        let Reverse(DistanceTriplet { distance: _, b1, b2 }) =
            distances.pop().expect("Distances empty");
        if !circuits.same_set(b1.id, b2.id) {
            circuits.union(b1.id, b2.id);
        }
    }
    circuits.all_sets()
        .map(|s| s.count()).collect::<BinaryHeap<_>>().iter()
        .take(3).product()
}

pub fn part2(input: &[String]) -> u64 {
    let boxes = parse_boxes(input).unwrap();
    let mut distances = sorted_distances(&boxes);
    let mut circuits = circuits(&boxes); // union-find via partitions crate
    let mut last_two = None;
    let mut connections = 0;
    while connections != boxes.len() - 1 {
        let Reverse(DistanceTriplet { distance: _, b1, b2 }) =
            distances.pop().expect("Distances empty");
        if !circuits.same_set(b1.id, b2.id) {
            last_two = Some(b1.x as u64 * b2.x as u64);
            circuits.union(b1.id, b2.id);
            connections += 1;
        }
    }
    last_two.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let input = input.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(part1(&input, 10), 40);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT, 1000), 57564);
    }

    #[test]
    fn test_example_part2() {
        let input = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let input = input.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(part2(&input), 25272);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 133296744);
    }
}