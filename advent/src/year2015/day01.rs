use crate::solver::AdventSolver;

pub struct Advent2015Day01Solver {
    input: String,
}

impl Advent2015Day01Solver {
    pub fn new(input: String) -> Self {
        Self {
            input
        }
    }
}

impl AdventSolver for Advent2015Day01Solver {
    fn day(&self) -> usize { 1 }
    fn year(&self) -> usize { 2015 }

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
