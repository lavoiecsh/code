use crate::solver::AdventSolver;
use regex::{Captures, Match, Regex};
use State::*;

pub struct Advent2024Day03Solver {
    input: String,
    re: Regex,
}

impl Advent2024Day03Solver {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            re: Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap(),
        }
    }
}

impl AdventSolver for Advent2024Day03Solver {
    fn solve_part1(&self) -> usize {
        self.re.captures_iter(&self.input).fold(0, all_enabled_sum)
    }

    fn solve_part2(&self) -> usize {
        self.re
            .captures_iter(&self.input)
            .fold(Enabled(0), toggleable_sum)
            .sum()
    }
}

fn parse(c: Option<Match>) -> usize {
    c.unwrap().as_str().parse().unwrap()
}

fn all_enabled_sum(prev: usize, cap: Captures) -> usize {
    match cap.get(0).unwrap().as_str().chars().nth(2).unwrap() {
        '(' => prev,
        'n' => prev,
        'l' => prev + parse(cap.get(1)) * parse(cap.get(2)),
        _ => unreachable!("invalid capture {}", cap.get(1).unwrap().as_str()),
    }
}

enum State {
    Enabled(usize),
    Disabled(usize),
}

impl State {
    fn sum(self) -> usize {
        match self {
            Enabled(s) => s,
            Disabled(s) => s,
        }
    }
}

fn toggleable_sum(prev: State, cap: Captures) -> State {
    match (prev, cap.get(0).unwrap().as_str().chars().nth(2).unwrap()) {
        (Enabled(p), '(') => Enabled(p),
        (Enabled(p), 'n') => Disabled(p),
        (Enabled(p), 'l') => Enabled(p + parse(cap.get(1)) * parse(cap.get(2))),
        (Disabled(p), '(') => Enabled(p),
        (Disabled(p), 'n') => Disabled(p),
        (Disabled(p), 'l') => Disabled(p),
        (p, _) => unreachable!(
            "invalid state {} and capture {}",
            matches!(p, Enabled(_)),
            cap.get(0).unwrap().as_str()
        ),
    }
}

#[cfg(test)]
//noinspection SpellCheckingInspection
mod test {
    use super::*;

    #[test]
    fn finds_sum_of_multiplications() {
        let solver = Advent2024Day03Solver::new(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        assert_eq!(solver.solve_part1(), 161);
    }

    #[test]
    fn finds_sum_of_enabled_multiplications() {
        let solver = Advent2024Day03Solver::new(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(solver.solve_part2(), 48);
    }
}
