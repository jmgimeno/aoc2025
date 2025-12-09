use common::read_file_as_elements;
use once_cell::sync::Lazy;
use std::cmp::Ordering;
use std::str::FromStr;

pub static INPUT: Lazy<Vec<Point>> =
    Lazy::new(|| read_file_as_elements("data/day09.txt").expect("Failed to load input"));

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn area(&self, other: &Point) -> u64 {
        let width = (self.x - other.x).abs() as u64 + 1;
        let height = (self.y - other.y).abs() as u64 + 1;
        width * height
    }
}

impl FromStr for Point {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(',');
        let x = split.next().ok_or(())?.parse().map_err(|_| ())?;
        let y = split.next().ok_or(())?.parse().map_err(|_| ())?;
        Ok(Point { x, y })
    }
}

fn cross(o: &Point, a: &Point, b: &Point) -> i64 {
    let ax = (a.x - o.x) as i64;
    let ay = (a.y - o.y) as i64;
    let bx = (b.x - o.x) as i64;
    let by = (b.y - o.y) as i64;
    ax * by - ay * bx
}

pub fn convex_hull(points: &[Point]) -> Vec<Point> {
    // Monotone chain algorithm (Andrew)
    let mut pts = points.to_vec();
    if pts.len() <= 1 {
        return pts;
    }

    pts.sort_by(|a, b| match a.x.cmp(&b.x) {
        Ordering::Equal => a.y.cmp(&b.y),
        other => other,
    });

    pts.dedup_by(|a, b| a.x == b.x && a.y == b.y);

    let mut lower: Vec<Point> = Vec::new();
    for &p in &pts {
        while lower.len() >= 2 && cross(&lower[lower.len() - 2], &lower[lower.len() - 1], &p) <= 0 {
            lower.pop();
        }
        lower.push(p);
    }

    let mut upper: Vec<Point> = Vec::new();
    for &p in pts.iter().rev() {
        while upper.len() >= 2 && cross(&upper[upper.len() - 2], &upper[upper.len() - 1], &p) <= 0 {
            upper.pop();
        }
        upper.push(p);
    }

    // quitar el último elemento de cada mitad porque se repite (punto de unión)
    lower.pop();
    upper.pop();
    lower.extend(upper);
    if lower.is_empty() && !pts.is_empty() {
        lower.push(pts[0]);
    }
    lower
}

pub fn part1(tiles: &[Point]) -> u64 {
    let external = convex_hull(tiles);
    let mut max_area = 0;
    for (i, tile1) in external.iter().enumerate() {
        for tile2 in &external[i + 1..] {
            max_area = std::cmp::max(max_area, tile1.area(tile2));
        }
    }
    max_area
}

pub fn part2(_tiles: &[Point]) -> usize {
    todo!("day09 - part1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let tiles = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<Point>>();
        assert_eq!(part1(&tiles), 50);
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 4750176210);
    }

    #[test]
    fn test_part2() {
        todo!("day09 - test - part2")
    }
}