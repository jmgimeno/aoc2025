use common::read_file_as_elements;
use itertools::Itertools;
use once_cell::sync::Lazy;
use std::cmp::PartialEq;
use std::collections::BTreeMap;
use std::ops::Bound::Included;
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

    fn up_left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    fn up_right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y - 1,
        }
    }

    fn down_left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn down_right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
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

#[derive(PartialEq, Debug)]
enum SegmentOrientation {
    Left,
    Right,
    Up,
    Down,
}

impl SegmentOrientation {
    fn from_points(p1: &Point, p2: &Point) -> Self {
        assert_ne!(p1, p2, "Cannot create orientation from identical points");
        assert!(p1.x == p2.x || p1.y == p2.y, "Points share a common axis");
        if p1.x == p2.x {
            if p1.y < p2.y {
                SegmentOrientation::Down
            } else {
                SegmentOrientation::Up
            }
        } else {
            if p1.x < p2.x {
                SegmentOrientation::Right
            } else {
                SegmentOrientation::Left
            }
        }
    }
}

fn boundary_vertices(tiles: &[Point]) -> Vec<Point> {
    let mut exterior = Vec::with_capacity(tiles.len());
    let tiles = guarantee_exterior_on_the_left(tiles);
    exterior.push(tiles[0].up_left());
    for (p1, p2, p3) in tiles.iter().tuple_windows() {
        // Invariant: Segment p1 p2 has its exterior on its left
        let in_step = SegmentOrientation::from_points(p1, p2);
        let out_step = SegmentOrientation::from_points(p2, p3);
        let outside_point = match (in_step, out_step) {
            (SegmentOrientation::Right, SegmentOrientation::Up) => &p2.up_left(),
            (SegmentOrientation::Right, SegmentOrientation::Down) => &p2.up_right(),
            (SegmentOrientation::Left, SegmentOrientation::Up) => &p2.down_left(),
            (SegmentOrientation::Left, SegmentOrientation::Down) => &p2.down_right(),
            (SegmentOrientation::Up, SegmentOrientation::Right) => &p2.up_left(),
            (SegmentOrientation::Up, SegmentOrientation::Left) => &p2.down_left(),
            (SegmentOrientation::Down, SegmentOrientation::Right) => &p2.up_right(),
            (SegmentOrientation::Down, SegmentOrientation::Left) => &p2.down_right(),
            _ => panic!("Line does not a zig-zag: {:?} {:?} {:?}", p1, p2, p3),
        };
        exterior.push(*outside_point);
    }
    exterior
}

fn find_lexicographic_first_tile_index(tiles: &[Point]) -> usize {
    assert!(!tiles.is_empty(), "tiles must not be empty to find start");
    let mut idx = 0;
    for next in 1..tiles.len() {
        if tiles[next].x < tiles[idx].x {
            idx = next;
        } else if tiles[next].x == tiles[idx].x && tiles[next].y < tiles[idx].y {
            idx = next;
        }
    }
    idx
}

fn guarantee_exterior_on_the_left(tiles: &[Point]) -> Vec<Point> {
    let mut tiles = tiles.to_vec();
    // We start with a tile we know where its exterior is
    let start_id = find_lexicographic_first_tile_index(&tiles);
    // We ensure that it will be the starting point of the tour
    tiles.rotate_left(start_id);
    // It will also be the end point of the tour
    tiles.push(tiles[0]);
    // We ensure the first movement will be on the left
    let first_step = SegmentOrientation::from_points(&tiles[0], &tiles[1]);
    if first_step == SegmentOrientation::Down {
        tiles.reverse();
    }
    tiles
}


struct InsideDetector {
    // for each x it gives the vertical edges at that x
    vertical_bounds: BTreeMap<i32, Vec<RangeInclusive<i32>>>,
    // for each y it gives the horizontal edges at that y
    horizontal_bounds: BTreeMap<i32, Vec<RangeInclusive<i32>>>,
}

impl InsideDetector {
    fn new(tiles: &[Point]) -> Self {
        let boundary_vertices = boundary_vertices(tiles);
        Self {
            vertical_bounds: boundary_vertices
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
            horizontal_bounds: boundary_vertices
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

    fn is_valid(&self, p1: &Point, p2: &Point) -> bool {
        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_y = p1.y.max(p2.y);
        //no collision with any vertical bound
        self
            .vertical_bounds
            .range((Included(min_x), Included(max_x)))
            .flat_map(|(_, ranges)| ranges.iter())
            .all(|r| max_y <= *r.start() || min_y >= *r.end())
            // and no collision with any horizontal bound
            && self
            .horizontal_bounds
            .range((Included(min_y), Included(max_y)))
            .flat_map(|(_, ranges)| ranges.iter())
            .all(|r| max_x <= *r.start() || min_x >= *r.end())
    }
}

pub fn part2(tiles: &[Point]) -> u64 {
    let inside_detector = InsideDetector::new(&tiles);
    let mut max_area = 0;
    for (p1, p2) in tiles.iter().tuple_combinations() {
        if inside_detector.is_valid(p1, p2) {
            max_area = max_area.max(p1.area(p2));
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
