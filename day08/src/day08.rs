use common::read_file_as_lines;
use once_cell::sync::Lazy;
use partitions::PartitionVec;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub static INPUT: Lazy<Vec<String>> =
    Lazy::new(|| read_file_as_lines("data/day08.txt").expect("Failed to load input"));

#[derive(Clone, Copy, Debug)]
pub struct Box {
    id: usize,
    x: u32,
    y: u32,
    z: u32,
}

impl Box {
    fn distance(&self, other: Box) -> f64 {
        let x_diff = self.x as i64 - other.x as i64;
        let y_diff = self.y as i64 - other.y as i64;
        let z_diff = self.z as i64 - other.z as i64;
        let sq = x_diff * x_diff + y_diff * y_diff + z_diff * z_diff;
        (sq as f64).sqrt()
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

#[derive(Debug)]
struct DistanceTriplet {
    distance: f64,
    b1: Box,
    b2: Box,
}

impl DistanceTriplet {
    fn new(distance: f64, b1: Box, b2: Box) -> Self {
        Self { distance, b1, b2 }
    }
}

impl Eq for DistanceTriplet {}

impl PartialEq<Self> for DistanceTriplet {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl PartialOrd<Self> for DistanceTriplet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DistanceTriplet {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance < other.distance {
            Ordering::Greater
        } else if self.distance > other.distance {
            Ordering::Less
        } else {
            panic!("We assume no two distances are equal")
        }
    }
}

fn sorted_distances_heap(boxes: &[Box]) -> BinaryHeap<DistanceTriplet> {
    let mut distances = BinaryHeap::with_capacity(boxes.len() * boxes.len());
    for (i, b_i) in boxes.iter().enumerate() {
        for b_j in boxes[i + 1..].iter() {
            distances.push(DistanceTriplet::new(b_i.distance(*b_j), *b_i, *b_j));
        }
    }
    distances
}

#[allow(dead_code)]
fn sorted_distances_sorted_vec(boxes: &[Box]) -> Vec<DistanceTriplet> {
    let mut distances = Vec::with_capacity(boxes.len() * boxes.len());
    for (i, b_i) in boxes.iter().enumerate() {
        for b_j in boxes[i + 1..].iter() {
            distances.push(DistanceTriplet::new(b_i.distance(*b_j), *b_i, *b_j));
        }
    }
    distances.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
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
    let mut distances = sorted_distances_heap(&boxes);
    let mut circuits = circuits(&boxes);
    for _ in 0..connections {
        let DistanceTriplet { distance: _, b1, b2 } =
            distances.pop().expect("Distances empty");
        if !circuits.same_set(b1.id, b2.id) {
            circuits.union(b1.id, b2.id);
        }
    }
    let mut sizes = circuits.all_sets().map(|s| s.count()).collect::<Vec<_>>();
    sizes.sort_by_key(|a| usize::MAX -a);
    sizes.iter().take(3).product()
}

pub fn part2(_input: &[String]) -> usize {
    todo!("day08 - part1")
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
    fn test_part2() {
        todo!("day08 - test - part2")
    }
}