use crate::solver::AdventSolver;

pub struct Advent2015Day17Solver {
    containers: Vec<usize>,
}

impl Advent2015Day17Solver {
    pub fn new(input: &str) -> Self {
        Self {
            containers: input.lines().map(|l| l.parse().unwrap()).collect(),
        }
    }

    fn compute_total(&self, index: usize) -> usize {
        let mut total: usize = 0;
        let mut power: usize = 1;
        for i in 0..self.containers.len() {
            if index & power != 0 {
                total += self.containers[i];
            }
            power *= 2;
        }
        total
    }
}

impl AdventSolver for Advent2015Day17Solver {
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
        0
    } else {
        compute_count(index / 2) + index % 2
    }
}
