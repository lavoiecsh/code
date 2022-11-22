use std::fs;
use md5;
use crate::problem_solver::ProblemSolver;

const START_PART1: &str = "00000";
const START_PART2: &str = "000000";

const INPUT1: &str = "abcdef";
const INPUT2: &str = "pqrstuv";
const INPUT3: &str = "iwrupvqb";

pub struct Problem04Solver;

impl Problem04Solver {
    pub fn new() -> Self {
        Self {}
    }
}

impl ProblemSolver for Problem04Solver {
    fn solve_part1(&self) -> usize {
        find_number(INPUT3, START_PART1)
    }

    fn solve_part2(&self) -> usize {
        find_number(INPUT3, START_PART2)
    }
}

fn find_number(input: &str, start: &str) -> usize {
    let mut number: usize = 0;
    while true {
        number += 1;
        let digest = md5::compute(input.to_owned() + number.to_string().as_str());
        if format!("{:x}", digest).starts_with(start) {
            return number
        }
    }
    0
}
