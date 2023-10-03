use crate::solver::AdventSolver;

const FIRST: usize = 20151125;
const MULTIPLIER: usize = 252533;
const MODULO: usize = 33554393;

const ROW: usize = 2947;
const COLUMN: usize = 3029;

pub struct Advent2015Day25Solver {}

impl Advent2015Day25Solver {
    pub fn new(_input: String) -> Self {
        // todo read actual input instead of hard-coding
        Self {}
    }
}

impl AdventSolver for Advent2015Day25Solver {
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
}

fn iterate(number: usize) -> usize {
    (number * MULTIPLIER) % MODULO
}
