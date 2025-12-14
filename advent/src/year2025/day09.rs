use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub struct Advent2025Day09Solver {
    path: Path,
}

impl Advent2025Day09Solver {
    pub fn new(input: &str) -> Self {
        Self {
            path: Path::new(
                input
                    .lines()
                    .filter_map(|l| l.split_once(','))
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .collect(),
            ),
        }
    }
}

impl AdventSolver for Advent2025Day09Solver {
    fn solve_part1(&self) -> usize {
        self.path.largest_red_rectangle()
    }

    fn solve_part2(&self) -> usize {
        self.path.largest_green_rectangle()
    }
}

struct Path {
    horizontal_walls: Vec<HorizontalWall>,
    vertical_walls: Vec<VerticalWall>,
    rectangles: Vec<Rectangle>,
}

struct HorizontalWall {
    y: usize,
    xs: RangeInclusive<usize>,
}

impl HorizontalWall {
    fn new(y: usize, xs: RangeInclusive<usize>) -> Self {
        Self { y, xs }
    }

    fn overlaps(&self, rectangle: &Rectangle) -> bool {
        if rectangle.ys.start() == &self.y
            || rectangle.ys.end() == &self.y
            || !rectangle.ys.contains(&self.y)
            || rectangle.xs.end() == self.xs.start()
            || rectangle.xs.start() == self.xs.end()
        {
            return false;
        }
        rectangle.xs.contains(self.xs.start())
            || rectangle.xs.contains(self.xs.end())
            || self.xs.contains(rectangle.xs.start())
            || self.xs.contains(rectangle.xs.end())
    }
}

struct VerticalWall {
    x: usize,
    ys: RangeInclusive<usize>,
}

impl VerticalWall {
    fn new(x: usize, ys: RangeInclusive<usize>) -> Self {
        Self { x, ys }
    }

    fn overlaps(&self, rectangle: &Rectangle) -> bool {
        if rectangle.xs.start() == &self.x
            || rectangle.xs.end() == &self.x
            || !rectangle.xs.contains(&self.x)
            || rectangle.ys.start() == self.ys.end()
            || rectangle.ys.end() == self.ys.start()
        {
            return false;
        }
        rectangle.ys.contains(self.ys.start())
            || rectangle.ys.contains(self.ys.end())
            || self.ys.contains(rectangle.ys.start())
            || self.ys.contains(rectangle.ys.end())
    }
}

#[derive(Debug)]
struct Rectangle {
    xs: RangeInclusive<usize>,
    ys: RangeInclusive<usize>,
}

impl Rectangle {
    fn new(a: &Pos, b: &Pos) -> Self {
        Self {
            xs: a.0.min(b.0)..=a.0.max(b.0),
            ys: a.1.min(b.1)..=a.1.max(b.1),
        }
    }

    fn area(&self) -> usize {
        (self.xs.end() - self.xs.start() + 1) * (self.ys.end() - self.ys.start() + 1)
    }
}

impl Path {
    fn new(red_tiles: Vec<Pos>) -> Self {
        let rectangles = (0..red_tiles.len())
            .flat_map(|i| (i + 1..red_tiles.len()).map(move |j| (i, j)))
            .map(|(i, j)| Rectangle::new(&red_tiles[i], &red_tiles[j]))
            .sorted_by_key(Rectangle::area)
            .rev()
            .collect();
        let mut path_tiles = HashSet::new();
        let mut horizontal_walls = Vec::new();
        let mut vertical_walls = Vec::new();
        let mut last = &red_tiles[red_tiles.len() - 1];
        for next in &red_tiles {
            if next.0 == last.0 {
                let x = next.0;
                let min_y = next.1.min(last.1);
                let max_y = next.1.max(last.1);
                path_tiles.extend((min_y..=max_y).map(|y| (x, y)));
                vertical_walls.push(VerticalWall::new(x, min_y..=max_y));
            } else {
                let y = next.1;
                let min_x = next.0.min(last.0);
                let max_x = next.0.max(last.0);
                path_tiles.extend((min_x..=max_x).map(|x| (x, y)));
                horizontal_walls.push(HorizontalWall::new(y, min_x..=max_x));
            }
            last = next;
        }
        Self {
            horizontal_walls,
            vertical_walls,
            rectangles,
        }
    }

    fn largest_red_rectangle(&self) -> usize {
        self.rectangles[0].area()
    }

    fn largest_green_rectangle(&self) -> usize {
        self.rectangles
            .iter()
            .find(|r| self.is_valid(r))
            .unwrap()
            .area()
    }

    fn is_valid(&self, rectangle: &Rectangle) -> bool {
        self.vertical_walls.iter().all(|w| !w.overlaps(rectangle))
            && self.horizontal_walls.iter().all(|w| !w.overlaps(rectangle))
    }
}

type Pos = (usize, usize);

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    #[test]
    fn finds_largest_rectangle() {
        let solver = Advent2025Day09Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 50);
    }

    #[test]
    fn finds_largest_green_rectangle() {
        let solver = Advent2025Day09Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 24);
    }
}
