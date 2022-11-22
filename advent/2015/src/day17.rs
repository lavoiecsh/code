use std::fs;
use itertools::Itertools;
use crate::problem_solver::ProblemSolver;

pub struct Problem17Solver {
    containers: Vec<usize>
}
impl Problem17Solver {
    pub fn new() -> Self {
        Self {
            containers: fs::read_to_string("inputs/day17.txt")
                .expect("error reading")
                .trim()
                .lines()
                .map(|l| l.parse().unwrap())
                .collect()
        }
    }
    
    fn compute_total(&self, index: usize) -> usize {
        let mut total: usize = 0;
        for i in 0..self.containers.len() {
            if index & usize::pow(2, i as u32) != 0 {
                total += self.containers[i];
            }
        }
        total
    }
}

impl ProblemSolver for Problem17Solver {
    fn solve_part1(&self) -> usize {
        const LIMIT: usize = 150;
        let mut count: usize = 0;
        let max: usize = usize::pow(2, self.containers.len() as u32);
        for i in 0..max {
            let total = self.compute_total(i);
            if total == LIMIT {
                count += 1;
            }
        }
        count
    }

    fn solve_part2(&self) -> usize {
        const LIMIT: usize = 150;
        let mut min: usize = self.containers.len();
        let mut count: usize = 0;
        let max: usize = usize::pow(2, self.containers.len() as u32);
        for i in 0..max {
            let total = self.compute_total(i);
            if total != LIMIT {
                continue;
            }
            let container_count = compute_count(i);
            if container_count > min {
                continue;
            }
            if container_count == min {
                count += 1;
                continue;
            }
            if container_count < min {
                count = 1;
                min = container_count;
            }
        }
        count
    }
}

fn compute_count(index: usize) -> usize {
    if index == 0 {
        return 0;
    }
    compute_count(index / 2) + index % 2
}
