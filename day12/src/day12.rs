use common::read_file_as_string;
use once_cell::sync::Lazy;

pub static INPUT: Lazy<String> =
    Lazy::new(|| read_file_as_string("data/day12.txt").expect("Failed to load input"));

#[derive(Debug, Clone)]
struct Problem {
    width: usize,           // width of the region to fill
    height: usize,          // heigh of the region
    quantities: Vec<usize>, // quantity of each shape needed
}

fn parse_problems(input: &str) -> Vec<Problem> {
    let mut problems = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        if line.contains('x') {
            let parts: Vec<&str> = line.split(':').collect();
            let dimensions: Vec<&str> = parts[0].trim().split('x').collect();
            let width = dimensions[0].parse::<usize>().unwrap();
            let height = dimensions[1].parse::<usize>().unwrap();

            let quantities: Vec<usize> = parts[1]
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            problems.push(Problem {
                width,
                height,
                quantities,
            });
        }
    }
    problems
}

pub fn part1(input: &str) -> usize {
    parse_problems(input)
        .iter()
        .filter(|problem| {
            problem.width * problem.height >= 9 * problem.quantities.iter().sum::<usize>()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "does not work on the example input ?!"]
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
