use itertools::Itertools;
use num_traits::abs;

use crate::solver::AdventSolver;

pub struct Advent2018Day05Solver {
    polymer: Vec<i8>,
}

impl Advent2018Day05Solver {
    pub fn new(input: String) -> Self {
        Self {
            polymer: input.chars()
                .map(|c|
                    if c.is_ascii_lowercase() {
                        'a' as i8 - c as i8 - 1
                    } else {
                        c as i8 - 'A' as i8 + 1
                    })
                .collect()
        }
    }
}

impl AdventSolver for Advent2018Day05Solver {
    fn solve_part1(&self) -> usize {
        simplify(&self.polymer).len()
    }

    fn solve_part2(&self) -> usize {
        (1..=26)
            .map(|r| simplify(&self.polymer.iter().filter(|u| r != abs(**u)).cloned().collect_vec()).len())
            .min()
            .unwrap()
    }
}

fn simplify(input: &[i8]) -> Vec<i8> {
    let mut output = input.to_owned();
    let mut prev_len = 0;
    while prev_len != output.len() {
        prev_len = output.len();
        let mut tmp = Vec::new();
        let mut skip = false;
        for i in 1..output.len() {
            if skip {
                skip = false;
                continue;
            }
            if output[i-1] + output[i] == 0 {
                skip = true;
            } else {
                tmp.push(output[i-1]);
            }
        }
        if !skip {
            tmp.push(output[output.len() - 1]);
        }
        output = tmp;
    }
    output
}
