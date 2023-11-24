use std::collections::HashSet;

use itertools::Itertools;
use num_traits::abs;
use regex::Regex;

use crate::solver::AdventSolver;

pub struct Advent2018Day06Solver {
    grid: Grid,
}

impl Advent2018Day06Solver {
    pub fn new(input: String) -> Self {
        let re = Regex::new(r"(\d+), (\d+)").unwrap();
        let coordinates = input.lines()
            .filter_map(|l| re.captures(l))
            .map(|c| (c.get(1).unwrap().as_str().parse().unwrap(),
                      c.get(2).unwrap().as_str().parse().unwrap()))
            .collect();
        Self { grid: Grid::new(&coordinates) }
    }
}

impl AdventSolver for Advent2018Day06Solver {
    fn solve_part1(&self) -> usize {
        self.grid.largest_finite_region()
    }

    fn solve_part2(&self) -> usize {
        self.grid.within_distance
    }
}

type Coord = (i64, i64);

struct Grid {
    counts: Vec<usize>,
    infinite: HashSet<usize>,
    within_distance: usize,
}

impl Grid {
    fn new(coordinates: &Vec<Coord>) -> Self {
        let mut counts = Vec::new();
        counts.resize(coordinates.len(), 0);
        let mut infinite: HashSet<usize> = HashSet::new();
        let mut within_distance = 0;

        let (mut min_x, mut max_x) = coordinates.iter().map(|c| c.0).minmax().into_option().unwrap();
        let (mut min_y, mut max_y) = coordinates.iter().map(|c| c.1).minmax().into_option().unwrap();

        macro_rules! calculate {
            ($x: expr, $y: expr) => {
                let distances = coordinates.iter().map(|c| abs(c.0 - $x) + abs(c.1 - $y)).collect_vec();
                let closest = distances.iter().enumerate().min_by(|l, r| l.1.cmp(&r.1)).unwrap();
                if distances.iter().filter(|d| *d == closest.1).count() == 1 {
                    counts[closest.0] += 1;
                    infinite.insert(closest.0);
                }
                if distances.iter().sum::<i64>() < 10000 {
                    within_distance += 1;
                }
            }
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                calculate!(x, y);
            }
        }

        let mut prev_infinite_len: usize = 0;
        let mut prev_within_distance = 0;
        while prev_within_distance != within_distance || prev_infinite_len != infinite.len() {
            prev_within_distance = within_distance;
            prev_infinite_len = infinite.len();
            infinite.clear();
            min_x -= 1;
            min_y -= 1;
            max_x += 1;
            max_y += 1;
            for y in min_y..=max_y {
                calculate!(min_x, y);
                calculate!(max_x, y);
            }
            for x in min_x + 1..=max_x - 1 {
                calculate!(x, min_y);
                calculate!(x, max_y);
            }
        }
        Self { counts, infinite, within_distance }
    }

    fn largest_finite_region(&self) -> usize {
        (0..self.counts.len())
            .filter(|i| !self.infinite.contains(i))
            .map(|i| self.counts[i])
            .max()
            .unwrap()
    }
}
