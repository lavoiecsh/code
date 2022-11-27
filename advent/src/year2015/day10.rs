use crate::solver::AdventSolver;

const INPUT: &str = "1113122113";
pub struct Advent2015Day10Solver {}

impl AdventSolver for Advent2015Day10Solver {
    fn day(&self) -> usize { 10 }
    fn year(&self) -> usize { 2015 }

    fn solve_part1(&self) -> usize {
        let mut result = INPUT.to_string();
        for _ in 0..40 {
            result = look_and_say(result);
        }
        result.len()
    }

    fn solve_part2(&self) -> usize {
        let mut result = INPUT.to_string();
        for _ in 0..50 {
            result = look_and_say(result);
        }
        result.len()
    }
}

fn look_and_say(input: String) -> String {
    let mut output = String::new();
    let mut chars = input.chars();
    let mut prev = chars.next().unwrap();
    let mut count: usize = 1;
    for c in chars {
        if prev == c {
            count += 1;
            continue;
        }
        output += format!("{}{}", count, prev).as_str();
        count = 1;
        prev = c;
    }
    output += format!("{}{}", count, prev).as_str();
    output
}

pub fn advent2015_day10_solver() -> Box<dyn AdventSolver> {
    Box::new(Advent2015Day10Solver {})
}
