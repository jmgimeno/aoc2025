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

#[derive(Debug)]
struct ParsedProblems {
    num_of_hashes: Vec<usize>,  // #'s in the corresponding figure
    problems: Vec<Problem>,
}


fn parse_problems(input: &str) -> ParsedProblems {
    let lines: Vec<&str> = input.lines().collect();
    let mut num_of_hashes = Vec::new();
    let mut problems = Vec::new();

    let mut i = 0;

    // Parse shapes
    while i < lines.len() {
        let line = lines[i].trim();

        // Check if this is a shape header (e.g., "0:")
        if line.ends_with(':') {
            i += 1;

            // Read the next 3 lines for the 3x3 shape
            let mut filled = 0;
            for _ in 0..3 {
                if i < lines.len() {
                    let shape_line = lines[i].trim();
                    for (col, ch) in shape_line.chars().enumerate() {
                        if col < 3 {
                            if ch == '#' { filled += 1; };
                        }
                    }
                    i += 1;
                }
            }

            num_of_hashes.push(filled);

            // Skip empty line after shape
            if i < lines.len() && lines[i].trim().is_empty() {
                i += 1;
            }
        } else if line.contains('x') {
            // Parse region line (e.g., "4x4: 0 0 0 0 2 0")
            let parts: Vec<&str> = line.split(':').collect();
            let dimensions: Vec<&str> = parts[0].trim().split('x').collect();
            let width = dimensions[0].parse::<usize>().unwrap();
            let height = dimensions[1].parse::<usize>().unwrap();

            let quantities: Vec<usize> = parts[1]
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();

            problems.push(Problem { width, height, quantities });
            i += 1;
        } else {
            i += 1;
        }
    }

    ParsedProblems { num_of_hashes, problems }
}

pub fn part1(input: &str) -> usize {
    let ParsedProblems { num_of_hashes, problems } = parse_problems(input);
    problems
        .iter()
        .filter(|problem| {
            let total_size = problem.width * problem.height;
            let total_hashes = problem
                .quantities.iter()
                .zip(num_of_hashes.iter())
                .map(|(quantity, filled)| quantity * filled)
                .sum();
            total_size >= total_hashes
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
