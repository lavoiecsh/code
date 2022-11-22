use std::fs;
use crate::problem_solver::ProblemSolver;

pub struct Problem01Solver {
    input: String,
}

impl Problem01Solver {
    pub fn new() -> Self {
        Self {
            input: fs::read_to_string("inputs/day01.txt")
                .expect("error reading")
        }
    }
}

impl ProblemSolver for Problem01Solver {
    fn solve_part1(&self) -> usize {
        self.input
            .chars()
            .fold(0, |acc, cur| acc + eval(cur))
            as usize
    }

    fn solve_part2(&self) -> usize {
        self.input
            .chars()
            .fold(Fold { i: 0, c: 0, found: false }, accumulate_fold)
            .i
    }
}

fn eval(c: char) -> i64 {
    if c == '(' { 1 } else { -1 }
}

struct Fold {
    i: usize,
    c: i64,
    found: bool,
}

fn accumulate_fold(acc: Fold, cur: char) -> Fold {
    if acc.found { return acc; }
    let c = acc.c + eval(cur);
    Fold {
        i: acc.i + 1,
        c,
        found: c < 0,
    }
}
