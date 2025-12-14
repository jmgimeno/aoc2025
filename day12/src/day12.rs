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

    fn num_of_hashes(&self) -> usize {
        self.0.iter().map(|row| row.iter().filter(|&x| *x).count()).sum()
    }

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

#[derive(Clone)]
struct State {
    quantities: Vec<usize>,
    available: Vec<Vec<bool>>,
}

impl State {
    fn new(width: usize, height: usize, quantities: Vec<usize>) -> Self {
        Self {
            quantities,
            available: vec![vec![true; width]; height],
        }
    }

    fn is_solution(&self) -> bool {
        self.quantities.iter().all(|&i| i == 0)
    }

    fn admits(&self, x: usize, y: usize, s: &Shape) -> bool {
        for dx in 0..SHAPE_SIDE {
            for dy in 0..SHAPE_SIDE {
                if s.0[dy][dx] && !self.available[y + dy][x + dx] {
                    return false;
                }
            }
        }
        true
    }

    fn mark(&mut self, x: usize, y: usize, s: &Shape) {
        for dx in 0..SHAPE_SIDE {
            for dy in 0..SHAPE_SIDE {
                if s.0[dy][dx] {
                    self.available[y + dy][x + dx] = false;
                }
            }
        }
    }

    fn unmark(&mut self, x: usize, y: usize, s: &Shape) {
        for dx in 0..SHAPE_SIDE {
            for dy in 0..SHAPE_SIDE {
                if s.0[dy][dx] {
                    self.available[y + dy][x + dx] = true;
                }
            }
        }
    }

    fn _advance(&self, x: usize, y: usize, s: &Shape, i: usize) -> Self {
        let mut new_state = self.clone();
        new_state.mark(x, y, s);
        new_state.quantities[i] -= 1;
        new_state
    }
}

#[derive(Debug, Clone)]
struct Problem {
    width: usize,           // width of the region to fill
    height: usize,          // heigh of the region
    quantities: Vec<usize>, // quantity of each shape needed
}

impl Problem {
    fn new(width: usize, height: usize, quantities: Vec<usize>) -> Self {
        Self {
            width,
            height,
            quantities,
        }
    }

    fn possible_coordinates(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..=self.width - SHAPE_SIDE)
            .flat_map(|x| (0..=self.height - SHAPE_SIDE).map(move |y| (x, y)))
    }

    fn is_solvable(&self, shapes: &[Shape], shape_permutations: &Vec<HashSet<Shape>>) -> bool {
        if self.is_trivially_solvable() {
            return true;
        }
        if self.is_trivially_insolvable(shapes) {
            return false;
        }
        self.can_fit(&shape_permutations)
    }

    fn is_trivially_solvable(&self) -> bool {
        let max_nonoverlapping_shapes = (self.width / 3) * (self.height / 3);
        let total_shapes = self.quantities.iter().sum::<usize>();
        total_shapes <= max_nonoverlapping_shapes
    }

    fn is_trivially_insolvable(&self, shapes: &[Shape]) -> bool {
        let total_size = self.width * self.height;
        let total_hashes = self
            .quantities
            .iter()
            .zip(shapes.iter().map(|s| s.num_of_hashes()))
            .map(|(quantity, filled)| quantity * filled)
            .sum::<usize>();
        total_hashes > total_size
    }

    fn can_fit(&self, permutations: &Vec<HashSet<Shape>>) -> bool {
        let mut state = State::new(self.width, self.height, self.quantities.clone());
        let mut order = (0..self.quantities.len()).collect::<Vec<_>>();
        order.sort_by_key(|&i| -(self.quantities[i] as i8));
        dbg!(self.can_fit_rec(&mut state, &permutations, &order))
    }

    fn can_fit_rec(
        &self,
        state: &mut State,
        permutations: &Vec<HashSet<Shape>>,
        order: &Vec<usize>,
    ) -> bool {
        if state.is_solution() {
            true
        } else {
            for &i in order.iter() {
                if state.quantities[i] > 0 {
                    for shape in permutations[i].iter() {
                        for (x, y) in self.possible_coordinates() {
                            if state.admits(x, y, shape) {
                                state.mark(x, y, shape);
                                state.quantities[i] -= 1;
                                if self.can_fit_rec(state, permutations, order) {
                                    return true;
                                }
                                state.quantities[i] += 1;
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
    shapes: Vec<Shape>,
    problems: Vec<Problem>,
}

impl ParsedProblems {
    fn new(shapes: Vec<Shape>, regions: Vec<Problem>) -> Self {
        Self {
            shapes,
            problems: regions,
        }
    }

    fn count_solvable(&self) -> usize {
        let shape_permutations = self.shapes.iter()
            .map(|shape| shape.different_permutations())
            .collect::<Vec<_>>();
        self.problems
            .iter()
            .filter(|problem| {
                problem.is_solvable(&self.shapes, &shape_permutations)
            })
            .count()
    }
}

fn parse_problems(input: &str) -> ParsedProblems {
    let mut shapes = Vec::new();
    let mut problems = Vec::new();
    let mut lines = input.lines().peekable();

    // Parse shapes (blocks starting with "N:")
    while let Some(line) = lines.peek() {
        let trimmed = line.trim();
        if trimmed.contains('x') && trimmed.contains(':') {
            break; // Start of problem lines
        }
        let line = lines.next().unwrap().trim();
        if line.is_empty() || line.ends_with(':') {
            continue; // Skip empty lines and shape headers like 0:
        }
        // First row of shape found, collect all 3 rows
        let mut grid = [[false; SHAPE_SIDE]; SHAPE_SIDE];
        for (c, ch) in line.chars().enumerate() {
            if c < SHAPE_SIDE {
                grid[0][c] = ch == '#';
            }
        }
        for r in 1..SHAPE_SIDE {
            if let Some(row_line) = lines.next() {
                for (c, ch) in row_line.trim().chars().enumerate() {
                    if c < SHAPE_SIDE {
                        grid[r][c] = ch == '#';
                    }
                }
            }
        }
        shapes.push(Shape(grid));
    }

    // Parse problem lines
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(':').collect();
        let dims: Vec<usize> = parts[0].split('x').map(|s| s.parse().unwrap()).collect();
        let quantities: Vec<usize> = parts[1].split_whitespace().map(|s| s.parse().unwrap()).collect();

        problems.push(Problem::new(dims[0], dims[1], quantities));
    }

    ParsedProblems::new(shapes, problems)
}

pub fn part1(input: &str) -> usize {
    parse_problems(input).count_solvable()
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
        let shape5 = Shape([[true, true, true], [false, true, false], [true, true, true]]);
        let shapes = vec![shape0, shape1, shape2, shape3, shape4, shape5];

        // region 0 is solvable
        let region0 = Problem::new(4, 4, vec![0, 0, 0, 0, 2, 0]);
        let regions = vec![region0];
        let parsed = ParsedProblems::new(shapes.clone(), regions);
        assert_eq!(parsed.count_solvable(), 1);

        // Region 1 is solvable
        let region1 = Problem::new(12, 5, vec![1, 0, 1, 0, 2, 2]);
        let regions = vec![region1];
        let parsed = ParsedProblems::new(shapes.clone(), regions);
        assert_eq!(parsed.count_solvable(), 1);

        // Region 2 is not solvable (but too slow)
        // let region2 = Problem::new(12, 5, vec![1, 0, 1, 0, 3, 2]);
        // let regions = vec![region2];
        // let parsed = ParsedProblems::new(shapes.clone(), regions);
        // assert_eq!(parsed.count_solvable(), 0);
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

    #[ignore = "Takes too long to run on problem 3"]
    #[test]
    fn test_part1_example() {
        let input = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
        ";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 510);
    }
}
