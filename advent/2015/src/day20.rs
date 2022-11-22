use std::fs;
use crate::problem_solver::ProblemSolver;

const INPUT: usize = 36000000;

pub struct Problem20Solver;
impl Problem20Solver {
    pub fn new() -> Self {
        Self
    }
}

impl ProblemSolver for Problem20Solver {
    fn solve_part1(&self) -> usize {
        const MAX: usize = 1000000;
        let mut sieve: [usize; MAX] = [0; MAX];
        for i in 1..MAX {
            let mut j = i;
            let presents = i * 10;
            while j < MAX {
                sieve[j] += presents;
                j += i;
            }
            if sieve[i] > INPUT {
                return i;
            }
        }
        0
    }

    fn solve_part2(&self) -> usize {
        const MAX: usize = 1000000;
        let mut sieve: [usize; MAX] = [0; MAX];
        for i in 1..MAX {
            let presents = i * 11;
            for j in 0..50 {
                let house = i + j*i;
                if house >= MAX {
                    break;
                }
                sieve[house] += presents;
            }
            if sieve[i] > INPUT {
                return i;
            }
        }
        0
    }
}
