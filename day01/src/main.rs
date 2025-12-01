use common::read_file_as_string;
use once_cell::sync::Lazy;

static INPUT: Lazy<String> =
    Lazy::new(|| read_file_as_string("data/day01.txt").expect("Failed to load input"));

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

fn main() {
    println!("Part1: {}", part1(&INPUT));
    println!("Part2: {}", part2(&INPUT));
}

fn part1(input: &str) -> i32 {
    let mut position = 50;
    let mut password = 0;
    let input = parse_input(input);
    for movement in input {
        let direction = &movement[0..1];
        let step = &movement[1..].parse::<i32>().unwrap();
        match direction {
            "R" => position += step,
            "L" => position -= step,
            _ => panic!("Invalid direction: {}", direction),
        }
        if position % 100 == 0 {
            password += 1;
        }
    }
    password
}

fn part2(input: &str) -> i32 {
    let mut position = 50;
    let mut password = 0;
    let input = parse_input(input);
    for movement in input {
        let direction = &movement[0..1];
        let step = &movement[1..].parse::<i32>().unwrap();
        let old_position = position;
        password += step / 100;
        match direction {
            "R" => {
                position += step % 100;
                if position >= 100 {
                    password += 1;
                }
            },
            "L" => {
                position -= step % 100;
                if old_position != 0 && position <= 0 {
                    password += 1;
                }
            },
            _ => panic!("Invalid direction: {}", direction),
        }
        position = (position % 100 + 100) % 100;
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
