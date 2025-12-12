use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::HashSet;

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
        let mut path = self.path.clone();
        path.largest_green_rectangle()
    }
}

#[derive(Clone)]
struct Path {
    red_tiles: Vec<Pos>,
    path_tiles: HashSet<Pos>,
    green_tiles: HashSet<Pos>,
    black_tiles: HashSet<Pos>,
    rectangles: Vec<(usize, usize, usize)>,
}

impl Path {
    fn new(red_tiles: Vec<Pos>) -> Self {
        let rectangles = (0..red_tiles.len())
            .flat_map(|i| (i + 1..red_tiles.len()).map(move |j| (i, j)))
            .map(|(i, j)| (i, j, rectangle_area(&red_tiles[i], &red_tiles[j])))
            .sorted_by_key(|&(_, _, area)| area)
            .rev()
            .collect();
        let mut path_tiles = HashSet::new();
        let mut last = &red_tiles[red_tiles.len() - 1];
        for next in &red_tiles {
            if next.0 == last.0 {
                path_tiles.extend((next.1.min(last.1)..=next.1.max(last.1)).map(|y| (next.0, y)));
            } else {
                path_tiles.extend((next.0.min(last.0)..=next.0.max(last.0)).map(|x| (x, next.1)));
            }
            last = next;
        }
        Self {
            red_tiles,
            path_tiles,
            green_tiles: HashSet::new(),
            black_tiles: HashSet::new(),
            rectangles,
        }
    }

    fn largest_red_rectangle(&self) -> usize {
        self.rectangles[0].2
    }

    fn largest_green_rectangle(&mut self) -> usize {
        for i in 0..self.rectangles.len() {
            dbg!(&self.green_tiles.len());
            dbg!(&self.black_tiles.len());
            let (a, b, d) = self.rectangles[i];
            if self.is_valid_green(a, b) {
                return d;
            }
        }
        unreachable!("no solution found")
    }

    fn is_valid_green(&mut self, a: usize, b: usize) -> bool {
        let left = self.red_tiles[a].0.min(self.red_tiles[b].0);
        let right = self.red_tiles[a].0.max(self.red_tiles[b].0);
        let top = self.red_tiles[a].1.min(self.red_tiles[b].1);
        let bottom = self.red_tiles[a].1.max(self.red_tiles[b].1);
        dbg!(&a, &b);
        dbg!(&self.red_tiles[a], &self.red_tiles[b]);
        for y in top..=bottom {
            for x in left..=right {
                let mut pos = (x, y);
                if self.black_tiles.contains(&pos) {
                    return false;
                }
                if self.path_tiles.contains(&pos) {
                    continue;
                }
                if self.green_tiles.contains(&pos) {
                    continue;
                }
                let mut count = 0;
                while pos.0 != 0 && pos.1 != 0 {
                    if self.path_tiles.contains(&pos) {
                        count += 1;
                    }
                    pos.0 -= 1;
                    pos.1 -= 1;
                }
                if count % 2 == 1 {
                    self.green_tiles.insert((x, y));
                    continue;
                }
                self.black_tiles.insert((x, y));
                return false;
            }
        }
        true
    }
}

fn rectangle_area(a: &Pos, b: &Pos) -> usize {
    (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)
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
