use common::read_file_as_lines;
use once_cell::sync::Lazy;

const ACCESSIBLE_LIMIT: usize = 4;
const PAPER: char = '@';

pub static INPUT: Lazy<Diagram> = Lazy::new(|| {
    let lines = read_file_as_lines("data/day04.txt").expect("Failed to load input");
    Diagram::from_strings(lines)
});

#[derive(Debug, Clone)]
pub struct Diagram {
    // true == '@', false == '.'
    grid: Vec<Vec<bool>>,
}

impl Diagram {
    pub fn from_strings(lines: Vec<String>) -> Self {
        Diagram {
            grid: lines
                .into_iter()
                .map(|s| s.chars().map(|c| c == PAPER).collect::<Vec<bool>>())
                .collect::<Vec<Vec<bool>>>(),
        }
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn width(&self) -> usize {
        if self.grid.is_empty() {
            0
        } else {
            self.grid[0].len()
        }
    }

    fn is_paper(&self, x: isize, y: isize) -> bool {
        if y < 0 || x < 0 {
            false
        } else if y as usize >= self.height() || x as usize >= self.width() {
            false
        } else {
            self.grid[y as usize][x as usize]
        }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut counter = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.is_paper(x as isize + dx, y as isize + dy) {
                    counter += 1;
                }
            }
        }
        counter
    }

    fn accessible_papers(&self) -> Vec<(usize, usize)> {
        let mut accessible = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.is_paper(x as isize, y as isize)
                    && self.count_neighbors(x, y) < ACCESSIBLE_LIMIT
                {
                    accessible.push((x, y));
                }
            }
        }
        accessible
    }

    fn remove_accessible_papers(&mut self, to_remove: &[(usize, usize)]) {
        for (x, y) in to_remove {
            self.grid[*y][*x] = false;
        }
    }
}

pub fn part1(diagram: &Diagram) -> usize {
    diagram.accessible_papers().len()
}

pub fn part2(diagram: &Diagram) -> usize {
    let mut diagram = diagram.clone();
    let mut total_removed = 0;
    loop {
        let to_remove = diagram.accessible_papers();
        if to_remove.is_empty() {
            break total_removed;
        }
        total_removed += to_remove.len();
        diagram.remove_accessible_papers(&to_remove);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let lines = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let diagram = Diagram::from_strings(lines);
        assert_eq!(part1(&diagram), 13);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 1569);
    }

    #[test]
    fn test_example_part2() {
        let lines = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let diagram = Diagram::from_strings(lines);
        assert_eq!(part2(&diagram), 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 9280);
    }
}
