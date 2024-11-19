use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

use crate::solver::AdventSolver;

pub struct Advent2015Day09Solver {
    distances: HashMap<(String, String), usize>,
}

impl Advent2015Day09Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
        Self {
            distances: input
                .lines()
                .map(|l| {
                    let m = re.captures(l).unwrap();
                    ((m[1].to_string(), m[2].to_string()), m[3].parse().unwrap())
                })
                .collect(),
        }
    }

    fn compute_distance(&self, path: Vec<&String>) -> usize {
        let mut distance: usize = 0;
        for i in 1..path.len() {
            let destination1 = path[i];
            let destination2 = path[i - 1];
            let leg_distance = self
                .distances
                .get(&(destination1.clone(), destination2.clone()))
                .or(self
                    .distances
                    .get(&(destination2.clone(), destination1.clone())))
                .unwrap();
            distance += *leg_distance;
        }
        distance
    }
}

impl AdventSolver for Advent2015Day09Solver {
    fn solve_part1(&self) -> usize {
        let destinations: HashSet<String> = self
            .distances
            .iter()
            .flat_map(|e| [e.0 .0.clone(), e.0 .1.clone()])
            .collect();
        destinations
            .iter()
            .permutations(destinations.len())
            .unique()
            .map(|p| self.compute_distance(p))
            .min()
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        let destinations: HashSet<String> = self
            .distances
            .iter()
            .flat_map(|e| [e.0 .0.clone(), e.0 .1.clone()])
            .collect();
        destinations
            .iter()
            .permutations(destinations.len())
            .unique()
            .map(|p| self.compute_distance(p))
            .max()
            .unwrap()
    }
}
