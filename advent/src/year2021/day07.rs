use crate::solver::AdventSolver;

pub struct Advent2021Day07Solver {
    crabs: Vec<usize>
}

impl Advent2021Day07Solver {
    pub fn new(input: String) -> Self {
        Self {
            crabs: input
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect()
        }
    }
}

impl AdventSolver for Advent2021Day07Solver {
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
