use md5::compute;
use crate::solver::AdventSolver;

const START_PART1: &str = "00000";
const START_PART2: &str = "000000";

const INPUT: &str = "iwrupvqb";

pub struct Advent2015Day04Solver {
}

impl AdventSolver for Advent2015Day04Solver {
    fn day(&self) -> usize { 04 }
    fn year(&self) -> usize { 2015 }

    fn solve_part1(&self) -> usize {
        find_number(INPUT, START_PART1)
    }

    fn solve_part2(&self) -> usize {
        find_number(INPUT, START_PART2)
    }
}

fn find_number(input: &str, start: &str) -> usize {
    let mut number = 0;
    loop {
        number += 1;
        let digest = compute(input.to_owned() + number.to_string().as_str());
        if format!("{:x}", digest).starts_with(start) {
            return number
        }
    }
}

pub fn advent2015_day04_solver() -> Box<dyn AdventSolver> {
    Box::new(Advent2015Day04Solver {})
}
