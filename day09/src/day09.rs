use common::read_file_as_elements;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Unbounded};
use std::ops::RangeInclusive;
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
            max_area = std::cmp::max(max_area, tile1.area(tile2));
        }
    }
    max_area
}

#[derive(Debug)]
struct IsInside {
    // for each x it gives the vertical edges at that x
    vertical_edges: BTreeMap<i32, Vec<RangeInclusive<i32>>>,
    // for each y it gives the horizontal edhes at that y
    horizontal_edges: BTreeMap<i32, Vec<RangeInclusive<i32>>>,
}

impl IsInside {
    fn new(points: &[Point]) -> Self {
        Self {
            vertical_edges: points
                .iter()
                .circular_tuple_windows()
                .filter_map(|(p1, p2)| {
                    (p1.x == p2.x)
                        .then_some((p1.x, std::cmp::min(p1.y, p2.y)..=std::cmp::max(p1.y, p2.y)))
                })
                .into_group_map_by(|(x, _)| *x)
                .into_iter()
                .map(|(x, pairs)| (x, pairs.into_iter().map(|(_, r)| r).collect()))
                .collect::<BTreeMap<_, _>>(),
            horizontal_edges: points
                .iter()
                .circular_tuple_windows()
                .filter_map(|(p1, p2)| {
                    (p1.y == p2.y)
                        .then_some((p1.y, std::cmp::min(p1.x, p2.x)..=std::cmp::max(p1.x, p2.x)))
                })
                .into_group_map_by(|(y, _)| *y)
                .into_iter()
                .map(|(x, pairs)| (x, pairs.into_iter().map(|(_, r)| r).collect()))
                .collect::<BTreeMap<_, _>>(),
        }
    }

    fn is_inside(&self, point: &Point) -> bool {
        let Point { x, y } = point;
        // If it's on any vertical edge is inside (this accounts for vertices as well)
        if self
            .vertical_edges
            .get(x)
            .map(|ranges| ranges.iter().any(|range| range.contains(y)))
            .unwrap_or(false)
        {
            return true;
        }
        // If it's on any horizontal edge is inside
        if self
            .horizontal_edges
            .get(y)
            .map(|ranges| ranges.iter().any(|range| range.contains(x)))
            .unwrap_or(false)
        {
            return true;
        }
        // If not, we use ray-casting (to the infinite x)
        let mut crosses = 0;
        for (_xx, ranges) in self.vertical_edges.range((Excluded(x), Unbounded)) {
            if ranges.iter().any(|r| r.contains(y)) { // we cross a vertical edge at (_xx, y)
                crosses += 1;
            }
        }
        crosses % 2 == 1
    }
}

pub fn part2(tiles: &[Point]) -> u64 {
    let is_inside = IsInside::new(&tiles);
    let mut max_area = 0;
    for (i, tile1) in tiles.iter().enumerate() {
        for tile2 in &tiles[i + 1..] {
            let other1 = Point {
                x: tile1.x,
                y: tile2.y,
            };
            let other2 = Point {
                x: tile2.x,
                y: tile1.y,
            };
            // Is this condition enough to determine if all the square is inside?
            if is_inside.is_inside(&other1) && is_inside.is_inside(&other2) {
                max_area = std::cmp::max(max_area, tile1.area(tile2));
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
        todo!("day09 - test - part2")
    }

    #[test]
    fn test_example_is_inside() {
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
        let vertical_lines = IsInside::new(&tiles);
        assert!(!vertical_lines.is_inside(&Point { x: 2, y: 1 }));
        assert!(vertical_lines.is_inside(&Point { x: 9, y: 3 }));
    }
}
