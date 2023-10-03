use md5::compute;

use crate::solver::AdventSolver;

const START_PART1: &str = "00000";
const START_PART2: &str = "000000";

pub struct Advent2015Day04Solver {
    input: String,
}

impl Advent2015Day04Solver {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl AdventSolver for Advent2015Day04Solver {
    fn solve_part1(&self) -> usize {
        find_number(self.input.as_str(), START_PART1)
    }

    fn solve_part2(&self) -> usize {
        find_number(self.input.as_str(), START_PART2)
    }
}

fn find_number(input: &str, start: &str) -> usize {
    let mut number = 0;
    loop {
        number += 1;
        let digest = compute(input.to_owned() + number.to_string().as_str());
        if format!("{:x}", digest).starts_with(start) {
            return number;
        }
    }
}
