use crate::solver::AdventSolver;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, VecDeque};

pub struct Advent2018Day22Solver {
    depth: usize,
    target: (usize, usize),
}

impl Advent2018Day22Solver {
    pub fn new(input: String) -> Self {
        let depth_re = Regex::new(r"depth: (\d+)").unwrap();
        let target_re = Regex::new(r"target: (\d+),(\d+)").unwrap();
        let lines = input.lines().collect_vec();
        let depth = depth_re.captures(lines[0]).unwrap().get(1).unwrap().as_str().parse().unwrap();
        let target_cap = target_re.captures(lines[1]).unwrap();
        let target_x = target_cap.get(1).unwrap().as_str().parse().unwrap();
        let target_y = target_cap.get(2).unwrap().as_str().parse().unwrap();
        Self { depth, target: (target_x, target_y) }
    }
}

impl AdventSolver for Advent2018Day22Solver {
    fn solve_part1(&self) -> usize {
        let mut map = Map::new(self.depth, self.target);
        map.risk_level()
    }

    fn solve_part2(&self) -> usize {
        // todo slow (90s)
        let mut map = Map::new(self.depth, self.target);
        map.fastest_path()
    }
}

type Pos = (usize, usize);
const ROCKY: usize = 0;
const WET: usize = 1;
const NARROW: usize = 2;

struct Map {
    depth: usize,
    target: Pos,
    erosion_levels: HashMap<Pos, usize>,
}

impl Map {
    fn new(depth: usize, target: Pos) -> Self {
        let mut erosion_levels = HashMap::new();
        erosion_levels.insert((0, 0), depth % 20183);
        erosion_levels.insert(target, depth % 20183);
        Self { depth, target, erosion_levels }
    }

    fn region(&mut self, pos: Pos) -> usize {
        if self.erosion_levels.contains_key(&pos) {
            return *self.erosion_levels.get(&pos).unwrap();
        }

        let geologic_index = match pos {
            (0, 0) => 0,
            (x, 0) => x * 16807,
            (0, y) => y * 48271,
            (x, y) => self.region((x - 1, y)) * self.region((x, y - 1)),
        };
        let erosion_level = (geologic_index + self.depth) % 20183;
        self.erosion_levels.insert(pos, erosion_level);
        *self.erosion_levels.get(&pos).unwrap()
    }

    fn risk_level(&mut self) -> usize {
        let mut sum = 0;
        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                if let Some(r) = self.erosion_levels.get(&(x, y)) {
                    sum += r % 3;
                } else {
                    sum += self.region((x, y)) % 3;
                }
            }
        }
        sum
    }

    fn fastest_path(&mut self) -> usize {
        let mut queue = VecDeque::new();
        let mut gear_distances = DistanceMap::new();
        gear_distances.set((0, 0), 7);
        let mut torch_distances = DistanceMap::new();
        torch_distances.set((0, 0), 0);
        let mut neither_distances = DistanceMap::new();
        queue.push_back((1, 0));
        queue.push_back((0, 1));
        let mut best_target = usize::MAX;
        while let Some(pos) = queue.pop_front() {
            let region = self.region(pos);
            let mut positions_around = vec![(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)];
            if pos.0 > 0 { positions_around.push((pos.0 - 1, pos.1)); }
            if pos.1 > 0 { positions_around.push((pos.0, pos.1 - 1)); }
            let (changed_1, changed_2) = match region % 3 {
                ROCKY => (
                    gear_distances.insert_if_better(pos, &positions_around, &mut torch_distances),
                    torch_distances.insert_if_better(pos, &positions_around, &mut gear_distances)
                ),
                WET => (
                    gear_distances.insert_if_better(pos, &positions_around, &mut neither_distances),
                    neither_distances.insert_if_better(pos, &positions_around, &mut gear_distances)
                ),
                NARROW => (
                    torch_distances.insert_if_better(pos, &positions_around, &mut neither_distances),
                    neither_distances.insert_if_better(pos, &positions_around, &mut torch_distances)
                ),
                r => unreachable!("unknown region type {r}"),
            };
            if pos == self.target {
                if let Some(&g) = gear_distances.get(pos) {
                    if g < best_target {
                        best_target = g;
                    }
                }
                if let Some(&t) = torch_distances.get(pos) {
                    if t < best_target {
                        best_target = t;
                    }
                }
            }
            let distance_to_target = usize::max(self.target.0, pos.0) - usize::min(self.target.0, pos.0) +
                usize::max(self.target.1, pos.1) - usize::min(self.target.1, pos.1);
            if changed_1.is_some_and(|c| distance_to_target + c < best_target) ||
                changed_2.is_some_and(|c| distance_to_target + c < best_target) {
                queue.extend(positions_around);
            }
        }
        *torch_distances.get(self.target).unwrap()
    }
}

struct DistanceMap {
    distances: HashMap<Pos, usize>,
}

impl DistanceMap {
    fn new() -> Self {
        Self { distances: HashMap::new() }
    }

    fn get(&self, pos: Pos) -> Option<&usize> {
        self.distances.get(&pos)
    }

    fn set(&mut self, pos: Pos, distance: usize) -> Option<usize> {
        self.distances.insert(pos, distance);
        Some(distance)
    }

    fn set_if_better(&mut self, pos: Pos, distance: usize) -> Option<usize> {
        if let Some(&current) = self.distances.get(&pos) {
            if distance < current {
                self.set(pos, distance)
            } else {
                None
            }
        } else {
            self.set(pos, distance)
        }
    }

    fn insert_if_better(&mut self, pos: Pos, around: &[Pos], other: &mut Self) -> Option<usize> {
        if let Some(&best_around) = around.iter().filter_map(|p| self.distances.get(p)).min() {
            if let Some(&current) = self.distances.get(&pos) {
                if best_around + 1 < current {
                    other.set_if_better(pos, best_around + 8);
                    return self.set(pos, best_around + 1);
                }
            } else {
                other.set_if_better(pos, best_around + 8);
                return self.set(pos, best_around + 1);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const DEPTH: usize = 510;
    const TARGET: (usize, usize) = (10, 10);

    #[test]
    fn calculates_risk_level() {
        let solver = Advent2018Day22Solver { depth: DEPTH, target: TARGET };
        assert_eq!(solver.solve_part1(), 114);
    }

    #[test]
    fn finds_fastest_route() {
        let solver = Advent2018Day22Solver { depth: DEPTH, target: TARGET };
        assert_eq!(solver.solve_part2(), 45);
    }
}
