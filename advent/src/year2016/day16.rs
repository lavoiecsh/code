use itertools::Itertools;
use crate::solver::AdventSolver;

pub struct Advent2016Day16Solver {
    input: Vec<bool>,
}

impl Advent2016Day16Solver {
    pub fn new(input: String) -> Self {
        Self { input: input.chars().map(|c| c == '1').collect() }
    }
}

impl AdventSolver for Advent2016Day16Solver {
    fn solve_part1_string(&self) -> String {
        to_binary_string(checksum(self.input.clone().into_iter(), 272))
    }

    fn solve_part2_string(&self) -> String {
        to_binary_string(checksum(self.input.clone().into_iter(), 35651584))
    }
}

fn to_binary_string(input: impl Iterator<Item=bool>) -> String {
    input.map(|b| if b { '1' } else { '0' }).collect()
}

fn checksum(input: impl Iterator<Item=bool>, length: usize) -> impl Iterator<Item=bool> {
    shorten(lengthen(input, length).take(length))
}

fn lengthen(input: impl Iterator<Item=bool>, min_length: usize) -> impl Iterator<Item=bool> {
    let mut output: Vec<bool> = input.collect();
    while output.len() < min_length {
        let mut b: Vec<bool> = output.iter().map(|d| !d).collect();
        b.reverse();
        output.push(false);
        output.extend(b.iter());
    }
    output.into_iter()
}

fn shorten(input: impl Iterator<Item=bool>) -> impl Iterator<Item=bool> {
    let mut output: Vec<bool> = input.collect();
    while output.len() % 2 != 1 {
        let tmp = output.iter().chunks(2).into_iter().map(|mut c| c.next().unwrap() == c.next().unwrap()).collect();
        output = tmp;
    }
    output.into_iter()
}
