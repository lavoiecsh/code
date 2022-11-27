use std::fs::read_to_string;
use crate::solver::AdventSolver;

pub struct Advent2021Day07Solver {
    crabs: Vec<usize>
}

impl AdventSolver for Advent2021Day07Solver {
    fn day(&self) -> usize { 07 }
    fn year(&self) -> usize { 2021 }

    fn solve_part1(&self) -> usize {
        let max: usize = *self.crabs.iter().max().unwrap();
        let mut best = usize::MAX;
        for n in 0..=max {
            let sum = self.crabs
                .iter()
                .fold(0, |acc, cur| acc + if cur > &n { cur - n } else { n - cur });
            if sum < best {
                best = sum;
            }
        }
        best
    }

    fn solve_part2(&self) -> usize {
        let max = *self.crabs.iter().max().unwrap();
        let mut best = usize::MAX;
        for n in 0..=max {
            let sum = self.crabs
                .iter()
                .fold(0, |acc, cur| acc + cost(*cur, n));
            if sum < best {
                best = sum;
            }
        }
        best
    }
}

fn cost(crab: usize, pos: usize) -> usize {
    let dist = if crab > pos { crab - pos } else { pos - crab };
    (dist * dist + dist) / 2
}

pub fn advent2021_day07_solver() -> Box<dyn AdventSolver> {
    Box::new(Advent2021Day07Solver {
        crabs: read_to_string("src/year2021/day07.txt")
            .unwrap()
            .trim()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect()
    })
}
