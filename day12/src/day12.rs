use common::read_file_as_string;
use once_cell::sync::Lazy;
use std::collections::HashSet;

pub static INPUT: Lazy<String> =
    Lazy::new(|| read_file_as_string("data/day12.txt").expect("Failed to load input"));

const SHAPE_SIDE: usize = 3;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
// matrix of filled (true) and empty (false)
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
struct Problem {
    width: usize,               // width of the region to fill
    height: usize,              // heigh of the region
    quantities: Vec<u8>,        // quantity of each shape needed
    sorted_indexes: Vec<usize>, // indices sorted by decreasing quantity
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

    fn is_solution(&self) -> bool {
        self.shape_counter.iter().all(|&i| i == 0)
    }

    fn admits(&self, x: usize, y: usize, s: &Shape) -> bool {
        todo!()
    }

    fn mark(&mut self, x: usize, y: usize, s: &Shape) {
        todo!()
    }

    fn unmark(&mut self, x: usize, y: usize, s: &Shape) {
        todo!()
    }
}

impl Problem {
    fn new(width: usize, height: usize, quantities: Vec<u8>) -> Self {
        let mut sorted_indexes = (0..quantities.len()).collect::<Vec<_>>();
        sorted_indexes.sort_by_key(|&i| -(quantities[i] as i8));
        Self {
            width,
            height,
            quantities,
            sorted_indexes,
        }
    }

    fn possible_coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.width - SHAPE_SIDE)
            .flat_map(|x| (0..self.height - SHAPE_SIDE).map(move |y| (y, x)))
    }

    fn can_fit(&self, permutations: &Vec<HashSet<Shape>>) -> bool {
        let mut state = State::new(self.width, self.height, permutations.len());
        self.can_fit_rec(&mut state, &permutations)
    }

    fn can_fit_rec(&self, state: &mut State, permutations: &Vec<HashSet<Shape>>) -> bool {
        if state.is_solution() {
            true
        } else {
            for &i in self.sorted_indexes.iter() {
                if (state.shape_counter[i] > 0) {
                    for shape in permutations[i].iter() {
                        for (x, y) in self.possible_coordinates() {
                            if state.admits(x, y, shape) {
                                state.mark(x, y, shape);
                                state.shape_counter[i] -= 1;
                                if self.can_fit_rec(state, permutations) {
                                    return true;
                                }
                                state.shape_counter[i] += 1;
                                state.unmark(x, y, shape);
                            }
                        }
                    }
                }
            }
            false
        }
    }
}

#[derive(Debug)]
struct ParsedProblems {
    shapes: Vec<Shape>,     // vec with all the shapes (shape i at position i)
    problems: Vec<Problem>, // problems to solve
}

impl ParsedProblems {
    fn new(shapes: Vec<Shape>, regions: Vec<Problem>) -> Self {
        Self {
            shapes,
            problems: regions,
        }
    }
}

#[derive(Debug)]
struct Problems {
    shape_permutations: Vec<HashSet<Shape>>, // for shape i, all its possible rotations and flips
    problems: Vec<Problem>,                  // vec of problems to test
}

impl From<ParsedProblems> for Problems {
    fn from(parsed: ParsedProblems) -> Self {
        Self {
            shape_permutations: parsed
                .shapes
                .into_iter()
                .map(|shape| shape.different_permutations())
                .collect(),
            problems: parsed.problems,
        }
    }
}

impl Problems {
    fn count_solvable(&self) -> usize {
        self.problems
            .iter()
            .filter(|problem| problem.can_fit(&self.shape_permutations))
            .count()
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

        let region0 = Problem::new(4, 4, vec![0, 0, 0, 0, 2, 0]);
        let region1 = Problem::new(12, 5, vec![1, 0, 1, 0, 2, 2]);
        let region2 = Problem::new(12, 5, vec![1, 0, 1, 0, 3, 2]);
        let regions = vec![region0, region1, region2];

        let parsed = ParsedProblems::new(shapes, regions);

        println!("Parsed: {:?}", parsed);

        let problem = Problems::from(parsed);

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
            vec![shape0, p1, p2, p3, p4, p5, p6, p7]
                .into_iter()
                .collect()
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
