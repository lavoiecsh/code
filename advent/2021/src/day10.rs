#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

const FILENAME: &str = "inputs/day10.txt";

#[allow(dead_code)]
fn read_input() -> Vec<String> {
    fs::read_to_string(FILENAME).expect("error reading")
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let expressions = read_input();
    let mut score: usize = 0;
    for expression in expressions {
        let (_, corrupted) = validate(&expression);
        if corrupted.is_none() {
            continue;
        }
        score += score_corrupted(corrupted.unwrap());
    }
    score
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let expressions = read_input();
    let mut scores: Vec<usize> = Vec::new();
    for expression in expressions {
        debug!(&expression);
        let (stack, corrupted) = validate(&expression);
        debug!(&stack);
        debug!(corrupted);
        if corrupted.is_some() {
            continue;
        }
        scores.push(score_missing(&stack));
    }
    scores.sort();
    scores[scores.len() / 2]
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
            },
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
