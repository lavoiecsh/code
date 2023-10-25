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
        elves.push_back(first);
        elves.pop_front();
    }
    elves[0]
}

fn across(count: usize) -> usize {
    let mut elves: VecDeque<usize> = (count/2..count).collect();
    elves.extend(0..count/2);
    if count % 2 == 1 {
        elves.pop_front();
        let first = elves.pop_front().unwrap();
        elves.push_back(first);
    }
    while elves.len() > 3 {
        elves.pop_front();
        elves.pop_front();
        let first = elves.pop_front().unwrap();
        elves.push_back(first);
    }
    if elves.len() == 2 {
        elves.pop_front();
    }
    elves[0]
}
