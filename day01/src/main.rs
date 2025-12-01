use common::read_file_as_string;
use once_cell::sync::Lazy;

static INPUT: Lazy<String> =
    Lazy::new(|| read_file_as_string("data/day01.txt").expect("Failed to load input"));

fn parse_moves(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|l| {
            let (d, s) = l.split_at(1);
            (d.chars().next().unwrap(), s.parse::<i32>().unwrap())
        })
        .collect()
}

fn main() {
    println!("Part1: {}", part1(&INPUT));
    println!("Part2: {}", part2(&INPUT));
}

fn part1(input: &str) -> i32 {
    let mut position: i32 = 50;
    let mut password = 0;
    for (dir, step) in parse_moves(input) {
        match dir {
            'R' => position += step,
            'L' => position -= step,
            _ => panic!("Invalid direction: {}", dir),
        }
        if position % 100 == 0 {
            password += 1;
        }
    }
    password
}

fn part2(input: &str) -> i32 {
    let mut position: i32 = 50;
    let mut password = 0;
    for (dir, step) in parse_moves(input) {
        let whole = step / 100;
        let small = step % 100;
        password += whole;
        let old = position;

        match dir {
            'R' => {
                if old + small >= 100 {
                    password += 1;
                }
                position = (old + small) % 100;
            }
            'L' => {
                if old != 0 && old - small <= 0 {
                    password += 1;
                }
                position = ((old - small) % 100 + 100) % 100;
            }
            _ => panic!("Invalid direction: {}", dir),
        }
    }
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();
        assert_eq!(part1(&input), 3);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 964);
    }

    #[test]
    fn test_part2_example() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();
        assert_eq!(part2(&input), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 5872);
    }
}
