use common::read_file_as_lines;
use once_cell::sync::Lazy;

static INPUT: Lazy<Diagram> =
    Lazy::new(|| Diagram {
        lines: read_file_as_lines("data/day04.txt").expect("Failed to load input"),
    });

#[derive(Debug)]
struct Diagram {
    lines: Vec<String>,
}

impl Diagram {
    fn is_paper(&self, x: isize, y: isize) -> bool {
        if y < 0 || x < 0 { false }
        else if y as usize >= self.lines.len() || x as usize >= self.lines[y as usize].chars().count() { false }
        else { self.lines[y as usize].chars().nth(x as usize).unwrap() == '@' }
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut counter = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 { continue; }
                if self.is_paper(x as isize + dx, y as isize + dy) {
                    counter += 1;
                }
            }
        }
        counter
    }

    fn count_accessible_papers(&self) -> usize {
        let mut counter = 0;
        for y in 0..self.lines.len() {
            for x in 0..self.lines[y].chars().count() {
                if self.is_paper(x as isize , y as isize) && self.count_neighbors(x, y) < 4 {
                    counter += 1;
                }
            }
        }
        counter
    }
}

fn main() {
    println!("Part1: {}", part1(&INPUT));
    println!("Part2: {}", part2(&INPUT));
}

fn part1(diagram: &Diagram) -> usize {
    diagram.count_accessible_papers()
}

fn part2(_input: &Diagram) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let lines =
            vec!["..@@.@@@@.",
                 "@@@.@.@.@@",
                 "@@@@@.@.@@",
                 "@.@@@@..@.",
                 "@@.@@@@.@@",
                 ".@@@@@@@.@",
                 ".@.@.@.@@@",
                 "@.@@@.@@@@",
                 ".@@@@@@@@.",
                 "@.@.@@@.@."].iter().map(|s| s.to_string()).collect();
        let diagram = Diagram { lines };
        assert_eq!(part1(&diagram), 13);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 1569);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 20);
    }
}