use crate::solver::AdventSolver;

pub struct Advent2021Day01Solver {
    depths: Vec<usize>
}

impl Advent2021Day01Solver {
    pub fn new(input: String) -> Self {
        Self {
            depths: input
                .trim()
                .lines()
                .map(|s| s.parse().unwrap())
                .collect()
        }
    }
}

impl AdventSolver for Advent2021Day01Solver {
    fn solve_part1(&self) -> usize {
        let mut prev: usize = 1000000;
        let mut count: usize = 0;
        for d in &self.depths {
            if d > &prev {
                count += 1;
            }
            prev = *d;
        }
        count
    }

    fn solve_part2(&self) -> usize {
        let mut prev: usize = 1000000;
        let mut count: usize = 0;
        for i in 2..self.depths.len() {
            let sum = self.depths[i-2] + self.depths[i-1] + self.depths[i];
            if sum > prev {
                count += 1;
            }
            prev = sum;
        }
        count
    }
}
