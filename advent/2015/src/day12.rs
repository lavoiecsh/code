use std::fs;
use json::JsonValue;
use regex::Regex;
use crate::problem_solver::ProblemSolver;

pub struct Problem12Solver {
    object: JsonValue
}

impl Problem12Solver {
    pub fn new() -> Self {
        Self {
            object: json::parse(fs::read_to_string("inputs/day12.txt")
                .expect("error reading").as_str())
                .unwrap()
        }
    }
}

impl ProblemSolver for Problem12Solver {
    fn solve_part1(&self) -> usize {
        compute_sum(&self.object) as usize
    }

    fn solve_part2(&self) -> usize {
        compute_sum_without_red(&self.object) as usize
    }
}

fn compute_sum(value: &JsonValue) -> isize {
    if value.is_number() {
        value.as_isize().unwrap()
    } else if value.is_array() {
        value.members().map(|m| compute_sum(m)).sum()
    } else if value.is_object() {
        value.entries().map(|e| compute_sum(e.1)).sum()
    } else {
        0
    }
}

fn compute_sum_without_red(value: &JsonValue) -> isize {
    if value.is_number() {
        value.as_isize().unwrap()
    } else if value.is_array() {
        value.members().map(|m| compute_sum_without_red(m)).sum()
    } else if value.is_object() {
        if value.entries().any(|e| e.1 == "red") {
            0
        } else {
            value.entries().map(|e| compute_sum_without_red(e.1)).sum()
        }
    } else {
        0
    }
}
