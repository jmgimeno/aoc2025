use common::read_file_as_lines;
use once_cell::sync::Lazy;

static INPUT: Lazy<Diagram> = Lazy::new(|| Diagram {
    lines: read_file_as_lines("data/day04.txt").expect("Failed to load input"),
});

#[derive(Debug, Clone)]
struct Diagram {
    lines: Vec<String>,
}

impl Diagram {
    fn is_paper(&self, x: isize, y: isize) -> bool {
        if y < 0 || x < 0 {
            false
        } else if y as usize >= self.lines.len()
            || x as usize >= self.lines[y as usize].chars().count()
        {
            false
        } else {
            self.lines[y as usize].chars().nth(x as usize).unwrap() == '@'
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
        let mut accesible = Vec::new();
        for y in 0..self.lines.len() {
            for x in 0..self.lines[y].chars().count() {
                if self.is_paper(x as isize, y as isize) && self.count_neighbors(x, y) < 4 {
                    accesible.push((x, y));
                }
            }
        }
        accesible
    }

    fn remove_accesible_papers(&mut self, to_remove: &[(usize, usize)]) {
        for (x, y) in to_remove {
            let line = std::mem::take(&mut self.lines[*y]); // take ownership
            let mut bytes = line.into_bytes();
            bytes[*x] = b'.';
            self.lines[*y] = String::from_utf8(bytes).unwrap();
        }
    }
}

fn main() {
    println!("Part1: {}", part1(&INPUT));
    println!("Part2: {}", part2(&INPUT));
}

fn part1(diagram: &Diagram) -> usize {
    diagram.accessible_papers().len()
}

fn part2(diagram: &Diagram) -> usize {
    let mut diagram = diagram.clone();
    let mut total_removed = 0;
    loop {
        let to_remove = diagram.accessible_papers();
        if to_remove.is_empty() {
            break total_removed;
        }
        total_removed += to_remove.len();
        diagram.remove_accesible_papers(&to_remove);
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
        let diagram = Diagram { lines };
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
        let diagram = Diagram { lines };
        assert_eq!(part2(&diagram), 43);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 9280);
    }
}
