use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};

use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2016Day24Solver {
    map: Vec<Vec<bool>>,
    robots: HashMap<u8, Pos>,
    max_x: usize,
    max_y: usize,
}

impl Advent2016Day24Solver {
    pub fn new(input: String) -> Self {
        let mut robots = HashMap::new();
        let map: Vec<Vec<bool>> = input.lines()
            .enumerate()
            .map(|(y, l)| l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => false,
                    '.' => true,
                    _ => {
                        robots.insert(c as u8 - 48, Pos { x, y });
                        true
                    }
                })
                .collect())
            .collect();
        let max_x = map[0].len();
        let max_y = map.len();
        Self { map, robots, max_x, max_y }
    }

    fn build_distance_map(&self) -> HashMap<(u8, u8), usize> {
        let mut distance_map = HashMap::new();
        let robots: Vec<u8> = self.robots.keys().cloned().sorted().collect();
        for i in 0..robots.len() {
            for j in i+1..robots.len() {
                let ri = robots[i];
                let rj = robots[j];
                let distance = self.distance_between(ri, rj);
                distance_map.insert((ri, rj), distance);
                distance_map.insert((rj, ri), distance);
            }
        }
        distance_map
    }

    fn distance_between(&self, from: u8, to: u8) -> usize {
        let mut distances: HashMap<Pos, usize> = HashMap::new();
        let mut queue: VecDeque<Pos> = VecDeque::new();
        let start = self.robots.get(&from).unwrap();
        distances.insert(*start, 0);
        queue.push_back(*start);
        let end = self.robots.get(&to).unwrap();
        while !distances.contains_key(end) {
            let current = queue.pop_front().unwrap();
            let next_distance = distances.get(&current).unwrap() + 1;
            current.around(self.max_x, self.max_y)
                .filter(|p| self.is_open(p))
                .for_each(|p| {
                    if distances.contains_key(&p) { return }
                    distances.insert(p, next_distance);
                    queue.push_back(p);
                });
        }
        *distances.get(end).unwrap()
    }

    fn is_open(&self, pos: &Pos) -> bool {
        self.map[pos.y][pos.x]
    }
}

impl AdventSolver for Advent2016Day24Solver {
    fn solve_part1(&self) -> usize {
        let distance_map = DistanceMap { distances: self.build_distance_map() };
        let robot_list: Vec<u8> = self.robots.keys().cloned().filter(|r| *r != 0).collect();
        robot_list.iter()
            .cloned()
            .permutations(robot_list.len())
            .map(|p| distance_map.compute_simple(&p))
            .min()
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        let distance_map = DistanceMap { distances: self.build_distance_map() };
        let robot_list: Vec<u8> = self.robots.keys().cloned().filter(|r| *r != 0).collect();
        robot_list.iter()
            .cloned()
            .permutations(robot_list.len())
            .map(|p| distance_map.compute_round(&p))
            .min()
            .unwrap()
    }
}

struct DistanceMap {
    distances: HashMap<(u8,u8), usize>,
}

impl DistanceMap {
    fn compute_simple(&self, route: &[u8]) -> usize {
        let mut distance = *self.distances.get(&(0, route[0])).unwrap();
        for i in 1..route.len() {
            distance += self.distances.get(&(route[i-1], route[i])).unwrap();
        }
        distance
    }

    fn compute_round(&self, route: &[u8]) -> usize {
        self.compute_simple(route) + self.distances.get(&(route[route.len() - 1], 0)).unwrap()
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn around(&self, max_x: usize, max_y: usize) -> impl Iterator<Item=Pos> {
        vec!(
            if self.x > 0 { Some(Pos { x: self.x - 1, y: self.y }) } else { None },
            if self.x < max_x - 1 { Some(Pos { x: self.x + 1, y: self.y }) } else { None },
            if self.y > 0 { Some(Pos { x: self.x, y: self.y - 1 }) } else { None },
            if self.y < max_y - 1 { Some(Pos { x: self.x, y: self.y + 1 }) } else { None },
        )
            .into_iter()
            .flatten()
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}
