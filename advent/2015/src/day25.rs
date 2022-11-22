use crate::problem_solver::ProblemSolver;

const FIRST: usize = 20151125;
const MULTIPLIER: usize = 252533;
const MODULO: usize = 33554393;

const ROW: usize = 2947;
const COLUMN: usize = 3029;

pub struct Problem25Solver;
impl Problem25Solver {
    pub fn new() -> Self {
        Self
    }
}

impl ProblemSolver for Problem25Solver {
    fn solve_part1(&self) -> usize {
        let mut number = FIRST;
        for row in 2..(ROW + COLUMN) {
            for col in 0..row {
                number = iterate(number);
                if (row - col) == ROW && (col + 1) == COLUMN {
                    return number;
                }
            }
        }
        0
    }

    fn solve_part2(&self) -> usize {
        0
    }
}

fn iterate(number: usize) -> usize {
    (number * MULTIPLIER) % MODULO
}
