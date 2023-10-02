use crate::solver::AdventSolver;

pub struct Advent2021Day10Solver {
    expressions: Vec<String>,
}

impl Advent2021Day10Solver {
    pub fn new(input: String) -> Self {
        Self {
            expressions: input
                .lines()
                .map(String::from)
                .collect()
        }
    }
}

impl AdventSolver for Advent2021Day10Solver {
    fn day(&self) -> usize { 10 }
    fn year(&self) -> usize { 2021 }

    fn solve_part1(&self) -> usize {
        self.expressions
            .iter()
            .map(validate)
            .filter(|(_, c)| c.is_some())
            .map(|(_, c)| score_corrupted(c.unwrap()))
            .sum::<usize>()
    }

    fn solve_part2(&self) -> usize {
        let mut scores: Vec<usize> = self.expressions
            .iter()
            .map(validate)
            .filter(|(_, c)| c.is_none())
            .map(|(s, _)| score_missing(&s))
            .collect();
        scores.sort();
        scores[scores.len() / 2]
    }
}

fn validate(expression: &String) -> (Vec<char>, Option<char>) {
    let mut stack: Vec<char> = Vec::new();
    for c in expression.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let last = stack.pop();
                if last.is_none() {
                    panic!("empty stack");
                }
                if last.unwrap() != opening(c) {
                    return (stack, Some(c));
                }
            }
            _ => panic!("invalid character")
        }
    }
    (stack, None)
}

fn opening(chunk: char) -> char {
    match chunk {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("invalid character")
    }
}

fn score_corrupted(chunk: char) -> usize {
    match chunk {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn score_missing(stack: &Vec<char>) -> usize {
    stack.iter().rev().fold(0, |acc, c| acc * 5 + match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("invalid character {}", c)
    })
}
