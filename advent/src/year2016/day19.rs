use std::collections::VecDeque;

use crate::solver::AdventSolver;

pub struct Advent2016Day19Solver {
    elf_count: usize,
}

impl Advent2016Day19Solver {
    pub fn new(input: String) -> Self {
        Self { elf_count: input.parse().unwrap() }
    }
}

impl AdventSolver for Advent2016Day19Solver {
    fn solve_part1(&self) -> usize {
        circle(self.elf_count) + 1
    }

    fn solve_part2(&self) -> usize {
        across(self.elf_count) + 1
    }
}

fn circle(count: usize) -> usize {
    let mut elves: VecDeque<usize> = (0..count).collect();
    while elves.len() != 1 {
        let first = elves.pop_front().unwrap();
        elves.pop_front();
        elves.push_back(first);
    }
    elves[0]
}

fn across(count: usize) -> usize {
    let mut elves: VecDeque<usize> = (0..count).collect();
    while elves.len() != 1 {
        elves.remove(elves.len() / 2);
        let first = elves.pop_front().unwrap();
        elves.push_back(first);
    }
    elves[0]
}
