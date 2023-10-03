use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2022Day06Solver {
    datastream: Vec<char>,
}

impl Advent2022Day06Solver {
    pub fn new(input: String) -> Self {
        Self {
            datastream: input
                .chars()
                .collect()
        }
    }

    fn find_unique_of_length(&self, length: usize) -> usize {
        for i in 0..self.datastream.len() - length {
            if self.datastream.iter().skip(i).take(length).all_unique() {
                return i + length;
            }
        }
        0
    }
}

impl AdventSolver for Advent2022Day06Solver {
    fn solve_part1(&self) -> usize {
        self.find_unique_of_length(4)
    }

    fn solve_part2(&self) -> usize {
        self.find_unique_of_length(14)
    }
}
