use json::JsonValue;

use crate::solver::AdventSolver;

pub struct Advent2015Day12Solver {
    object: JsonValue,
}

impl Advent2015Day12Solver {
    pub fn new(input: &str) -> Self {
        Self {
            object: json::parse(input).unwrap(),
        }
    }
}

impl AdventSolver for Advent2015Day12Solver {
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
        value.members().map(compute_sum).sum()
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
        value.members().map(compute_sum_without_red).sum()
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
