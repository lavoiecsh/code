use crate::solver::AdventSolver;

pub struct Advent2017Day01Solver {
    digits: Vec<u8>,
}

impl Advent2017Day01Solver {
    pub fn new(input: &str) -> Self {
        Self {
            digits: input.chars().map(|c| c as u8 - 48).collect(),
        }
    }
}

impl AdventSolver for Advent2017Day01Solver {
    fn solve_part1(&self) -> usize {
        let mut sum: usize = 0;
        for i in 1..self.digits.len() {
            if self.digits[i - 1] == self.digits[i] {
                sum += self.digits[i - 1] as usize;
            }
        }
        if self.digits[self.digits.len() - 1] == self.digits[0] {
            sum += self.digits[0] as usize;
        }
        sum
    }

    fn solve_part2(&self) -> usize {
        let mut sum: usize = 0;
        let half = self.digits.len() / 2;
        for i in 0..self.digits.len() {
            if self.digits[i] == self.digits[(i + half) % self.digits.len()] {
                sum += self.digits[i] as usize;
            }
        }
        sum
    }
}
