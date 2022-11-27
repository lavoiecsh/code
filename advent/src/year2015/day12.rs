use std::fs::read_to_string;
use json::JsonValue;
use crate::solver::AdventSolver;

pub struct Advent2015Day12Solver {
    object: JsonValue
}

impl AdventSolver for Advent2015Day12Solver {
    fn day(&self) -> usize { 12 }
    fn year(&self) -> usize { 2015 }

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

pub fn advent2015_day12_solver() -> Box<dyn AdventSolver> {
    Box::new(Advent2015Day12Solver {
        object: read_to_string("src/year2015/day12.txt")
            .map(|s| json::parse(s.as_str()))
            .unwrap()
            .unwrap()
    })
}
