use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Advent2017Day13Solver {
    layers: HashMap<usize, usize>,
}

impl Advent2017Day13Solver {
    pub fn new(input: &str) -> Self {
        Self {
            layers: input
                .lines()
                .map(|l| {
                    l.split(": ")
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2017Day13Solver {
    fn solve_part1(&self) -> usize {
        self.layers
            .iter()
            .filter(|(d, r)| *d % ((*r - 1) * 2) == 0)
            .map(|(d, r)| d * r)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        (0..)
            .find(|i| {
                self.layers
                    .iter()
                    .all(|(d, r)| (i + d) % ((r - 1) * 2) != 0)
            })
            .unwrap()
    }
}
