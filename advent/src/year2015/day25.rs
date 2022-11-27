use crate::solver::AdventSolver;

const FIRST: usize = 20151125;
const MULTIPLIER: usize = 252533;
const MODULO: usize = 33554393;

const ROW: usize = 2947;
const COLUMN: usize = 3029;

pub struct Advent2015Day25Solver {}

impl AdventSolver for Advent2015Day25Solver {
    fn day(&self) -> usize { 25 }
    fn year(&self) -> usize { 2015 }

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

pub fn advent2015_day25_solver() -> Box<dyn AdventSolver> {
    Box::new(Advent2015Day25Solver {})
}
