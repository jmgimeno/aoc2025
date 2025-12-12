use common::read_file_as_string;
use once_cell::sync::Lazy;
use std::collections::HashSet;

pub static INPUT: Lazy<String> =
    Lazy::new(|| read_file_as_string("data/day12.txt").expect("Failed to load input"));

const SHAPE_SIDE: usize = 3;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Shape([[bool; SHAPE_SIDE]; SHAPE_SIDE]);

impl Shape {
    fn rotate_right(&self) -> Self {
        let mut out = [[false; SHAPE_SIDE]; SHAPE_SIDE];
        for r in 0..SHAPE_SIDE {
            for c in 0..SHAPE_SIDE {
                out[c][SHAPE_SIDE - 1 - r] = self.0[r][c];
            }
        }
        Shape(out)
    }

    fn flip_horizontally(&self) -> Self {
        // horizontal mirror (flip left-right)
        let mut out = [[false; SHAPE_SIDE]; SHAPE_SIDE];
        for r in 0..SHAPE_SIDE {
            for c in 0..SHAPE_SIDE {
                out[r][SHAPE_SIDE - 1 - c] = self.0[r][c];
            }
        }
        Shape(out)
    }

    fn different_permutations(&self) -> HashSet<Self> {
        let mut set = HashSet::new();
        let mut cur = self.clone();
        for _ in 0..4 {
            set.insert(cur.clone());
            set.insert(cur.flip_horizontally());
            cur = cur.rotate_right();
        }
        set.into_iter().collect()
    }
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    quantities: Vec<u8>,
}

impl Region {
    fn new(width: usize, height: usize, quantities: Vec<u8>) -> Self {
        Self {
            width,
            height,
            quantities,
        }
    }

    fn sorted_indexes(&self) -> Vec<usize> {
        let mut indices = (0..self.quantities.len()).collect::<Vec<_>>();
        indices.sort_by_key(|&i| -(self.quantities[i] as i8));
        indices
    }
}

#[derive(Debug)]
struct ParsedProblem {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

impl ParsedProblem {
    fn new(shapes: Vec<Shape>, regions: Vec<Region>) -> Self {
        Self { shapes, regions }
    }
}

#[derive(Debug)]
struct Problem {
    shape_permutations: Vec<HashSet<Shape>>,
    regions: Vec<Region>,
}

impl From<ParsedProblem> for Problem {
    fn from(parsed: ParsedProblem) -> Self {
        let shape_permutations = parsed
            .shapes
            .into_iter()
            .map(|shape| shape.different_permutations())
            .collect();
        let regions = parsed.regions;
        Self {
            shape_permutations,
            regions,
        }
    }
}

struct State {
    shape_counter: Vec<u8>,
    region_state: Vec<Vec<bool>>,
}

impl State {
    fn new(width: usize, height: usize, shapes: usize) -> Self {
        Self {
            shape_counter: vec![0; shapes],
            region_state: vec![vec![false; width]; height],
        }
    }
}

pub fn part1(_input: &str) -> usize {
    todo!("day12 - part1")
}

pub fn part2(_input: &str) -> usize {
    todo!("day12 - part1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_manual_example() {
        let shape0 = Shape([[true, true, true], [true, true, false], [true, true, false]]);
        let shape1 = Shape([[true, true, true], [true, true, false], [false, true, true]]);
        let shape2 = Shape([[false, true, true], [true, true, true], [true, true, false]]);
        let shape3 = Shape([[true, true, false], [true, true, true], [true, true, false]]);
        let shape4 = Shape([[true, true, true], [true, false, false], [true, true, true]]);
        let shape5 = Shape([[true, true, true], [true, false, true], [true, true, true]]);
        let shapes = vec![shape0, shape1, shape2, shape3, shape4, shape5];

        let region0 = Region::new(4, 4, vec![0, 0, 0, 0, 2, 0]);
        let region1 = Region::new(12, 5, vec![1, 0, 1, 0, 2, 2]);
        let region2 = Region::new(12, 5, vec![1, 0, 1, 0, 3, 2]);
        let regions = vec![region0, region1, region2];

        let parsed = ParsedProblem::new(shapes, regions);

        println!("Parsed: {:?}", parsed);

        let problem = Problem::from(parsed);

        println!("Problem: {:?}", problem);
    }

    #[test]
    fn test_permutations1() {
        let all = Shape([[true, true, true], [true, true, true], [true, true, true]]);
        assert_eq!(
            all.different_permutations(),
            vec![all].into_iter().collect()
        )
    }

    #[test]
    fn test_permutations2() {
        let up = Shape([[true, true, true], [true, false, true], [true, true, true]]);
        let down = Shape([[true, true, true], [true, false, true], [true, true, true]]);
        let left = Shape([[true, true, true], [true, false, true], [true, true, true]]);
        let right = Shape([[true, true, true], [true, false, true], [true, true, true]]);

        assert_eq!(
            up.different_permutations(),
            vec![up, down, left, right].into_iter().collect()
        )
    }

    #[test]
    fn test_permutations3() {
        let tl = Shape([[false, true, true], [true, true, true], [true, true, true]]);
        let tr = Shape([[true, true, false], [true, true, true], [true, true, true]]);
        let bl = Shape([[true, true, true], [true, true, true], [false, true, true]]);
        let br = Shape([[true, true, true], [true, true, true], [true, true, false]]);

        assert_eq!(
            tl.different_permutations(),
            vec![tl, tr, bl, br].into_iter().collect()
        )
    }

    #[test]
    fn test_permutations_shape0() {
        let shape0 = Shape([[true, true, true], [true, true, false], [true, true, false]]);
        let p1 = Shape([[true, true, true], [false, true, true], [false, true, true]]);

        let p2 = Shape([[true, true, true], [true, true, true], [false, false, true]]);
        let p3 = Shape([[true, true, true], [true, true, true], [true, false, false]]);

        let p4 = Shape([[false, true, true], [false, true, true], [true, true, true]]);
        let p5 = Shape([[true, true, false], [true, true, false], [true, true, true]]);

        let p6 = Shape([[true, false, false], [true, true, true], [true, true, true]]);
        let p7 = Shape([[false, false, true], [true, true, true], [true, true, true]]);

        assert_eq!(
            shape0.different_permutations(),
            vec![shape0, p1, p2, p3, p4, p5, p6, p7].into_iter().collect()
        )
    }

    #[test]
    fn permutations_shape2() {
        let shape2 = Shape([[false, true, true], [true, true, true], [true, true, false]]);
        let p1 = Shape([[true, true, false], [true, true, true], [false, true, true]]);

        assert_eq!(
            shape2.different_permutations(),
            vec![shape2, p1].into_iter().collect()
        )
    }

    #[test]
    fn test_part1() {
        todo!("day12 - test - part1")
    }

    #[test]
    fn test_part2() {
        todo!("day12 - test - part2")
    }
}
