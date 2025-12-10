use common::read_file_as_elements;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::str::FromStr;

pub static INPUT: Lazy<Vec<Point>> =
    Lazy::new(|| read_file_as_elements("data/day09.txt").expect("Failed to load input"));

#[derive(Clone, Copy, Debug, Ord, Eq, PartialOrd, PartialEq)]
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
    // < 0: a -> b is a clockwise (left) turn around o
    // = 0: o, a, b are co-linear
    // > 0: a -> b is a counter-clockwise (right) turn around o
    let ax = (a.x - o.x) as i64;
    let ay = (a.y - o.y) as i64;
    let bx = (b.x - o.x) as i64;
    let by = (b.y - o.y) as i64;
    ax * by - ay * bx
}

fn half_border<'a, I>(iter: I) -> Vec<Point>
where
    I: Iterator<Item = &'a Point>,
{
    let mut border: Vec<Point> = Vec::new();
    for &p in iter {
        // only allow advance if clockwise
        while border.len() >= 2
            && cross(&border[border.len() - 2], &border[border.len() - 1], &p) >= 0
        {
            border.pop();
        }
        border.push(p);
    }
    border
}

pub fn convex_hull(points: &[Point]) -> Vec<Point> {
    // Monotone chain algorithm
    let mut pts = points.to_vec();
    if pts.len() <= 1 {
        return pts;
    }
    pts.sort_unstable(); // lexicographically: first x, then y
    let mut lower = half_border(pts.iter());
    let mut upper = half_border(pts.iter().rev());
    lower.pop();
    upper.pop();
    lower.extend(upper);
    lower
}

pub fn part1(tiles: &[Point]) -> u64 {
    let external = convex_hull(tiles);
    let mut max_area = 0;
    for (i, tile1) in external.iter().enumerate() {
        for tile2 in &external[i + 1..] {
            max_area = max_area.max(tile1.area(tile2));
        }
    }
    max_area
}

#[derive(Debug)]
struct InsideDetector<'a> {
    lines: Vec<(&'a Point, &'a Point)>,
}

impl<'a> InsideDetector<'a> {
    fn new(points: &'a [Point]) -> Self {
        Self {
            lines: points
                .iter()
                .circular_tuple_windows()
                .collect::<Vec<(_, _)>>(),
        }
    }

    // Chris Biscardi's idea
    fn is_valid(&self, p1: &Point, p2: &Point) -> bool {
        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_y = p1.y.max(p2.y);
        self.lines.iter().all(|(l1, l2)| {
            let min_l_x = l1.x.min(l2.x);
            let max_l_x = l1.x.max(l2.x);
            let min_l_y = l1.y.min(l2.y);
            let max_l_y = l1.y.max(l2.y);
            max_x <= min_l_x // line on the right
                || min_x >= max_l_x // line on the left
                || max_y <= min_l_y // line below
                || min_y >= max_l_y // line above
        })
    }
}

pub fn part2(tiles: &[Point]) -> u64 {
    let detector = InsideDetector::new(tiles);
    let mut max_area = 0;
    for (i, p1) in tiles.iter().enumerate() {
        for p2 in &tiles[i + 1..] {
            if detector.is_valid(p1, p2) {
                max_area = max_area.max(p1.area(&p2));
            }
        }
    }
    max_area
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
        let tiles = input
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<Point>>();
        assert_eq!(part1(&tiles), 50);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 4750176210);
    }

    #[test]
    fn test_example_part2() {
        let input = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let tiles = input
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<Point>>();
        assert_eq!(part2(&tiles), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 1574684850);
    }
}
