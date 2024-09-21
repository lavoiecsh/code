use std::collections::{HashMap, HashSet};

use crate::solver::AdventSolver;

type Pos = (usize, usize, usize);

fn surrounding_pos(pos: &Pos) -> Vec<Pos> {
    vec!(
        (pos.0 - 1, pos.1, pos.2),
        (pos.0 + 1, pos.1, pos.2),
        (pos.0, pos.1 - 1, pos.2),
        (pos.0, pos.1 + 1, pos.2),
        (pos.0, pos.1, pos.2 - 1),
        (pos.0, pos.1, pos.2 + 1),
    )
}

struct Cube {
    pos: Pos,
    l: bool,
    r: bool,
    u: bool,
    d: bool,
    f: bool,
    b: bool,
}

impl Cube {
    fn new(pos: &Pos) -> Self {
        Self {
            pos: *pos,
            l: true,
            r: true,
            u: true,
            d: true,
            f: true,
            b: true,
        }
    }

    fn open_sides(&self) -> Vec<bool> {
        vec!(self.l, self.r, self.u, self.d, self.f, self.b)
    }

    fn count_open(&self) -> usize {
        self.open_sides()
            .iter()
            .filter(|s| **s)
            .count()
    }

    fn open_adjacents(&self) -> Vec<Pos> {
        self.open_sides()
            .iter()
            .zip(surrounding_pos(&self.pos))
            .filter(|(o, _)| **o)
            .map(|(_, p)| p)
            .collect()
    }
}

pub struct Advent2022Day18Solver {
    cubes: Vec<Pos>,
}

impl Advent2022Day18Solver {
    pub fn new(input: String) -> Self {
        Self {
            cubes: input
                .lines()
                .map(|l| {
                    let mut s = l.split(",");
                    (s.next().unwrap().parse().unwrap(),
                     s.next().unwrap().parse().unwrap(),
                     s.next().unwrap().parse().unwrap())
                })
                .collect()
        }
    }

    fn build_cube_map(&self) -> HashMap<Pos, Cube> {
        let mut cubes: HashMap<Pos, Cube> = HashMap::new();
        for p in &self.cubes {
            let mut cube = Cube::new(p);
            if let Some(c) = cubes.get_mut(&(p.0 - 1, p.1, p.2)) {
                c.r = false;
                cube.l = false;
            }
            if let Some(c) = cubes.get_mut(&(p.0 + 1, p.1, p.2)) {
                c.l = false;
                cube.r = false;
            }
            if let Some(c) = cubes.get_mut(&(p.0, p.1 - 1, p.2)) {
                c.d = false;
                cube.u = false;
            };
            if let Some(c) = cubes.get_mut(&(p.0, p.1 + 1, p.2)) {
                c.u = false;
                cube.d = false;
            };
            if let Some(c) = cubes.get_mut(&(p.0, p.1, p.2 - 1)) {
                c.b = false;
                cube.f = false;
            };
            if let Some(c) = cubes.get_mut(&(p.0, p.1, p.2 + 1)) {
                c.f = false;
                cube.b = false;
            };
            cubes.insert(*p, cube);
        }
        cubes
    }
}

impl AdventSolver for Advent2022Day18Solver {
    fn solve_part1(&self) -> usize {
        let cubes = self.build_cube_map();
        cubes.values()
            .map(Cube::count_open)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let cubes = self.build_cube_map();
        let min0 = cubes.values().map(|c| c.pos.0).min().unwrap() - 1;
        let max0 = cubes.values().map(|c| c.pos.0).max().unwrap() + 1;
        let min1 = cubes.values().map(|c| c.pos.1).min().unwrap() - 1;
        let max1 = cubes.values().map(|c| c.pos.1).max().unwrap() + 1;
        let min2 = cubes.values().map(|c| c.pos.2).min().unwrap() - 1;
        let max2 = cubes.values().map(|c| c.pos.2).min().unwrap() + 1;
        let mut open_cubes: HashSet<Pos> = cubes
            .values()
            .flat_map(Cube::open_adjacents)
            .filter(|p| p.0 != min0 && p.0 != max0 && p.1 != min1 && p.1 != max1 && p.2 != min2 && p.2 != max2)
            .collect();
        let mut prev_open_cube_count = usize::MAX;
        while open_cubes.len() != prev_open_cube_count {
            prev_open_cube_count = open_cubes.len();
            open_cubes = open_cubes
                .iter()
                .filter(|p| open_cubes.iter().any(|p2| p2.0 < p.0 && p2.1 == p.1 && p2.2 == p.2) || cubes.contains_key(&(p.0 - 1, p.1, p.2)))
                .filter(|p| open_cubes.iter().any(|p2| p2.0 > p.0 && p2.1 == p.1 && p2.2 == p.2) || cubes.contains_key(&(p.0 + 1, p.1, p.2)))
                .filter(|p| open_cubes.iter().any(|p2| p2.0 == p.0 && p2.1 < p.1 && p2.2 == p.2) || cubes.contains_key(&(p.0, p.1 - 1, p.2)))
                .filter(|p| open_cubes.iter().any(|p2| p2.0 == p.0 && p2.1 > p.1 && p2.2 == p.2) || cubes.contains_key(&(p.0, p.1 + 1, p.2)))
                .filter(|p| open_cubes.iter().any(|p2| p2.0 == p.0 && p2.1 == p.1 && p2.2 < p.2) || cubes.contains_key(&(p.0, p.1, p.2 - 1)))
                .filter(|p| open_cubes.iter().any(|p2| p2.0 == p.0 && p2.1 == p.1 && p2.2 > p.2) || cubes.contains_key(&(p.0, p.1, p.2 + 1)))
                .copied()
                .collect();
        }
        cubes.values().map(Cube::count_open).sum::<usize>() -
            open_cubes.iter().map(|p| surrounding_pos(p).iter().filter(|p2| cubes.contains_key(p2)).count()).sum::<usize>()
    }
}
