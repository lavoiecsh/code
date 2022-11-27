use crate::solver::AdventSolver;

const INPUT: usize = 36000000;
pub struct Advent2015Day20Solver {}

impl AdventSolver for Advent2015Day20Solver {
    fn day(&self) -> usize { 20 }
    fn year(&self) -> usize { 2015 }

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

pub fn advent2015_day20_solver() -> Box<dyn AdventSolver> {
    Box::new(Advent2015Day20Solver {})
}
