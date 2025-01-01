use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Advent2024Day10Solver {
    map: TopographicMap,
}

impl Advent2024Day10Solver {
    pub fn new(input: &str) -> Self {
        Self {
            map: TopographicMap::new(
                input
                    .lines()
                    .map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect())
                    .collect(),
            ),
        }
    }
}

impl AdventSolver for Advent2024Day10Solver {
    fn solve_part1(&self) -> usize {
        self.map.trailhead_scores().iter().sum()
    }

    fn solve_part2(&self) -> usize {
        self.map.trailhead_ratings().iter().sum()
    }
}

type Pos = (usize, usize);

struct TopographicMap {
    grid: Vec<Vec<u8>>,
    height: usize,
    width: usize,
    trailheads: Vec<Pos>,
    nodes: HashMap<Pos, TrailNode>,
}

impl TopographicMap {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        let mut trailheads = Vec::new();
        for y in 0..height {
            for x in 0..width {
                match grid[y][x] {
                    0 => trailheads.push((y, x)),
                    _ => {}
                }
            }
        }
        let mut s = Self {
            grid,
            height,
            width,
            trailheads,
            nodes: HashMap::new(),
        };
        s.build();
        s
    }

    fn build(&mut self) {
        let mut queue = VecDeque::new();
        queue.extend(self.trailheads.iter().map(|&h| TrailNode::new(h, 0)));
        while let Some(mut current) = queue.pop_front() {
            let nexts = self
                .around(current.pos)
                .into_iter()
                .filter(|&p| self.value(p) == current.value + 1)
                .collect_vec();
            for next in nexts {
                if let Some(n) = self.nodes.get_mut(&next) {
                    current.children.push(n.pos);
                    n.parents.push(current.pos);
                } else {
                    let mut n = TrailNode::new(next, current.value + 1);
                    n.parents.push(current.pos);
                    current.children.push(n.pos);
                    queue.push_back(n);
                }
            }
            self.nodes.insert(current.pos, current);
        }
    }

    fn trailhead_scores(&self) -> Vec<usize> {
        self.trailheads
            .iter()
            .map(|&h| self.trailhead_score(h))
            .collect()
    }

    fn trailhead_ratings(&self) -> Vec<usize> {
        self.trailheads
            .iter()
            .map(|p| self.nodes.get(p).unwrap().rating(&self))
            .collect()
    }

    fn trailhead_score(&self, trailhead: Pos) -> usize {
        let mut positions = HashSet::new();
        positions.insert(trailhead);
        for v in 1..=9 {
            positions = positions
                .iter()
                .flat_map(|&p| self.around(p))
                .filter(|&p| self.value(p) == v)
                .collect();
        }
        positions.len()
    }

    fn around(&self, pos: Pos) -> Vec<Pos> {
        let mut a = Vec::new();
        if pos.0 > 0 {
            a.push((pos.0 - 1, pos.1));
        }
        if pos.0 < self.height - 1 {
            a.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            a.push((pos.0, pos.1 - 1));
        }
        if pos.1 < self.width - 1 {
            a.push((pos.0, pos.1 + 1));
        }
        a
    }

    fn value(&self, pos: Pos) -> u8 {
        self.grid[pos.0][pos.1]
    }

    fn node(&self, pos: &Pos) -> &TrailNode {
        self.nodes.get(pos).unwrap()
    }
}

#[derive(Debug)]
struct TrailNode {
    pos: Pos,
    parents: Vec<Pos>,
    children: Vec<Pos>,
    value: u8,
}

impl TrailNode {
    fn new(pos: Pos, value: u8) -> Self {
        Self {
            pos,
            value,
            parents: Vec::new(),
            children: Vec::new(),
        }
    }

    fn rating(&self, map: &TopographicMap) -> usize {
        if self.children.is_empty() {
            if self.value == 9 { 1 } else { 0 }
        } else {
            self.children.iter()
                .map(|p| map.node(p).rating(map))
                .sum()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn finds_trailhead_scores() {
        let solver = Advent2024Day10Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 36);
    }

    #[test]
    fn finds_trailhead_ratings() {
        let solver = Advent2024Day10Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 81);
    }
}
