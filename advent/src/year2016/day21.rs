use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use itertools::Itertools;
use Operation::*;
use crate::solver::AdventSolver;
use crate::year2016::day21::Operation::SwapPosition;

pub struct Advent2016Day21Solver {
    operations: Vec<Operation>,
}

impl Advent2016Day21Solver {
    pub fn new(input: String) -> Self {
        Self {
            operations: input.lines()
                .map(|l| {
                    let s: Vec<&str> = l.split(" ").collect();
                    match (s[0], s[1]) {
                        ("swap", "position") => SwapPosition(s[2].parse().unwrap(), s[5].parse().unwrap()),
                        ("swap", "letter") => SwapLetter(s[2].chars().next().unwrap(), s[5].chars().next().unwrap()),
                        ("rotate", "left") => RotateLeft(s[2].parse().unwrap()),
                        ("rotate", "right") => RotateRight(s[2].parse().unwrap()),
                        ("rotate", "based") => RotateLetter(s[6].chars().next().unwrap()),
                        ("reverse", "positions") => Reverse(s[2].parse().unwrap(), s[4].parse().unwrap()),
                        ("move", "position") => Move(s[2].parse().unwrap(), s[5].parse().unwrap()),
                        _ => panic!("unknown operation {l}"),
                    }
                })
                .collect()
        }
    }

    fn scramble(&self, input: impl Into<String>) -> String {
        let mut password: VecDeque<char> = input.into().chars().collect();
        self.operations.iter()
            .for_each(|o| o.execute(&mut password));
        password.iter().collect()
    }

    fn unscramble(&self, input: impl Into<String>) -> String {
        let mut password: VecDeque<char> = input.into().chars().collect();
        self.operations.iter()
            .rev()
            .for_each(|o| o.reverse().execute(&mut password));
        password.iter().collect()
    }
}

impl AdventSolver for Advent2016Day21Solver {
    fn solve_part1_string(&self) -> String {
        self.scramble("abcdefgh")
    }

    fn solve_part2_string(&self) -> String {
        self.unscramble("fbgdceah")
    }
}

enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
    UnrotateLetter(char),
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SwapPosition(a, b) => f.write_fmt(format_args!("swap position {a} with {b}")),
            SwapLetter(a, b) => f.write_fmt(format_args!("swap letter {a} with {b}")),
            RotateLeft(a) => f.write_fmt(format_args!("rotate left by {a}")),
            RotateRight(a) => f.write_fmt(format_args!("rotate right by {a}")),
            RotateLetter(a) => f.write_fmt(format_args!("rotate to letter {a}")),
            Reverse(a, b) => f.write_fmt(format_args!("reverse between {a} and {b}")),
            Move(a, b) => f.write_fmt(format_args!("move {a} to {b}")),
            UnrotateLetter(a) => f.write_fmt(format_args!("unrotate to letter {a}")),
        }
    }
}

impl Operation {
    fn execute(&self, password: &mut VecDeque<char>) {
        match self {
            SwapPosition(x, y) => {
                password.swap(*x, *y);
            }
            SwapLetter(x, y) => {
                let x_indices = password.iter().enumerate().filter_map(|(i, c)| if c == x { Some(i) } else { None }).collect_vec();
                let y_indices = password.iter().enumerate().filter_map(|(i, c)| if c == y { Some(i) } else { None }).collect_vec();
                x_indices.iter().for_each(|i| password[*i] = *y);
                y_indices.iter().for_each(|i| password[*i] = *x);
            }
            RotateLeft(c) => {
                for _ in 0..*c {
                    let first = password.pop_front().unwrap();
                    password.push_back(first);
                }
            }
            RotateRight(c) => {
                for _ in 0..*c {
                    let last = password.pop_back().unwrap();
                    password.push_front(last);
                }
            }
            RotateLetter(l) => {
                let index = password.iter().find_position(|c| *c == l).unwrap().0;
                RotateRight(1 + index + if index >= 4 { 1 } else { 0 }).execute(password);
            }
            Reverse(x, y) => {
                let mut i = *x;
                let mut j = *y;
                while i <= j {
                    password.swap(i, j);
                    i += 1;
                    j -= 1;
                }
            }
            Move(x, y) => {
                let removed = password.remove(*x).unwrap();
                password.insert(*y, removed);
            }
            UnrotateLetter(l) => {
                let index = password.iter().find_position(|c| *c == l).unwrap().0;
                let rotate = match index {
                    0 => 9,
                    6 => 8,
                    4 => 7,
                    2 => 6,
                    7 => 4,
                    5 => 3,
                    3 => 2,
                    1 => 1,
                    _ => panic!(),
                };
                RotateLeft(rotate).execute(password);
            }
        }
    }

    fn reverse(&self) -> Self {
        match self {
            SwapPosition(x, y) => SwapPosition(*y, *x),
            SwapLetter(x, y) => SwapLetter(*y, *x),
            RotateLeft(c) => RotateRight(*c),
            RotateRight(c) => RotateLeft(*c),
            RotateLetter(l) => UnrotateLetter(*l),
            Reverse(x, y) => Reverse(*x, *y),
            Move(x, y) => Move(*y, *x),
            UnrotateLetter(_) => panic!(),
        }
    }
}
