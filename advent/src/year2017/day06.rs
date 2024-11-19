use std::collections::HashMap;

use crate::solver::AdventSolver;

pub struct Advent2017Day06Solver {
    blocks: Vec<usize>,
}

impl Advent2017Day06Solver {
    pub fn new(input: &str) -> Self {
        Self {
            blocks: input
                .split_ascii_whitespace()
                .map(|b| b.parse().unwrap())
                .collect(),
        }
    }
}

impl AdventSolver for Advent2017Day06Solver {
    fn solve_part1(&self) -> usize {
        cycle(&self.blocks).0
    }

    fn solve_part2(&self) -> usize {
        cycle(&self.blocks).1
    }
}

fn cycle(input: &[usize]) -> (usize, usize) {
    let mut seen: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut current = input.to_owned();
    let mut index = 0;
    while !seen.contains_key(&current) {
        seen.insert(current.clone(), index);
        current = reallocate(&current);
        index += 1;
    }
    (index, index - seen.get(&current).unwrap())
}

fn reallocate(input: &[usize]) -> Vec<usize> {
    let (mut index, mut value) = select_redistribution(input);
    let mut output = input.to_owned();
    output[index] = 0;
    while value != 0 {
        index = (index + 1) % input.len();
        output[index] += 1;
        value -= 1;
    }
    output
}

fn select_redistribution(input: &[usize]) -> (usize, usize) {
    input.iter().enumerate().fold((usize::MAX, 0), |acc, cur| {
        if *cur.1 > acc.1 {
            (cur.0, *cur.1)
        } else {
            acc
        }
    })
}
