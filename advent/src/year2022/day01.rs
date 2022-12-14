use std::fs::read_to_string;
use crate::solver::AdventSolver;

pub struct Advent2022Day01Solver {
    calories: Vec<Vec<usize>>
}

impl Advent2022Day01Solver {
    pub fn new() -> Self {
        Self {
            calories: read_to_string("src/year2022/day01.txt")
                .unwrap()
                .split("\n\n")
                .map(|e| e.lines().map(|c| c.parse().unwrap()).collect())
                .collect()
        }
    }
}

impl AdventSolver for Advent2022Day01Solver {
    fn day(&self) -> usize { 01 }
    fn year(&self) -> usize { 2022 }

    fn solve_part1(&self) -> usize {
        self.calories
            .iter()
            .map(|e| e.iter().sum::<usize>())
            .max()
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        let mut elves: Vec<usize> = self.calories.iter().map(|e| e.iter().sum::<usize>()).collect();
        elves.sort();
        elves.iter().rev().take(3).sum()
    }
}
