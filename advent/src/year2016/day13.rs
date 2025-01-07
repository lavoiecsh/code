use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};

pub struct Advent2016Day13Solver {
    number: usize,
}

impl Advent2016Day13Solver {
    pub fn new(input: &str) -> Self {
        Self {
            number: input.parse().unwrap(),
        }
    }
}

impl AdventSolver for Advent2016Day13Solver {
    fn solve_part1(&self) -> usize {
        let map = Map::new(self.number, 80);
        let mut path = BreadthFirstSearch::new(&map);
        path.run_to_point((39, 31))
    }

    fn solve_part2(&self) -> usize {
        let map = Map::new(self.number, 30);
        let mut path = BreadthFirstSearch::new(&map);
        path.run_to_limit(50)
    }
}

struct Map {
    map: Vec<Vec<bool>>,
}

impl Map {
    fn new(number: usize, size: usize) -> Self {
        Self {
            map: (0..size)
                .map(|y| (0..size).map(|x| Map::is_open_init(x, y, number)).collect())
                .collect(),
        }
    }

    fn is_open_init(x: usize, y: usize, number: usize) -> bool {
        let mut n = x * x + 3 * x + 2 * x * y + y + y * y + number;
        let mut open = true;
        while n > 0 {
            if n % 2 == 1 {
                open = !open;
            }
            n >>= 1;
        }
        open
    }

    fn is_open(&self, point: &Point) -> bool {
        self.map[point.0][point.1]
    }
}

type Point = (usize, usize);

struct BreadthFirstSearch<'a> {
    distances: HashMap<Point, usize>,
    map: &'a Map,
}

impl Debug for BreadthFirstSearch<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\n{}",
            &(0..self.map.map.len())
                .map(|y| (0..self.map.map.len())
                    .map(|x| self.distances.get(&(y, x)).map_or_else(
                        || String::from(if self.map.is_open(&(y, x)) {
                            "   "
                        } else {
                            "###"
                        }),
                        |d| format!("{d:3}")
                    ))
                    .join(""))
                .join("\n")
        ))
    }
}

impl<'a> BreadthFirstSearch<'a> {
    fn new(map: &'a Map) -> Self {
        Self {
            map,
            distances: HashMap::new(),
        }
    }

    fn run_to_point(&mut self, p: Point) -> usize {
        let start = (1, 1);
        let mut queue: VecDeque<Point> = VecDeque::new();
        self.distances.insert(start, 0);
        queue.push_back(start);
        while !self.distances.contains_key(&p) {
            let current = queue.pop_front().unwrap();
            let next_distance = self.distances.get(&current).unwrap() + 1;
            queue.extend(
                [
                    self.above(&current),
                    self.below(&current),
                    self.left(&current),
                    self.right(&current),
                ]
                .iter()
                .flatten()
                .inspect(|&&p| {
                    self.distances.insert(p, next_distance);
                }),
            );
        }
        *self.distances.get(&p).unwrap()
    }

    fn run_to_limit(&mut self, limit: usize) -> usize {
        let start = (1, 1);
        let mut queue: VecDeque<Point> = VecDeque::new();
        self.distances.insert(start, 0);
        queue.push_back(start);
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            let next_distance = self.distances.get(&current).unwrap() + 1;
            if next_distance > limit {
                continue;
            }
            queue.extend(
                [
                    self.above(&current),
                    self.below(&current),
                    self.left(&current),
                    self.right(&current),
                ]
                .iter()
                .flatten()
                .inspect(|&&p| {
                    self.distances.insert(p, next_distance);
                }),
            );
        }
        self.distances.keys().count()
    }

    fn above(&self, point: &Point) -> Option<Point> {
        if point.0 == 0 {
            None
        } else {
            self.skip((point.0 - 1, point.1))
        }
    }

    fn below(&self, point: &Point) -> Option<Point> {
        self.skip((point.0 + 1, point.1))
    }

    fn left(&self, point: &Point) -> Option<Point> {
        if point.1 == 0 {
            None
        } else {
            self.skip((point.0, point.1 - 1))
        }
    }

    fn right(&self, point: &Point) -> Option<Point> {
        self.skip((point.0, point.1 + 1))
    }

    fn skip(&self, point: Point) -> Option<Point> {
        if self.map.is_open(&point) && !self.distances.contains_key(&point) {
            Some(point)
        } else {
            None
        }
    }
}
