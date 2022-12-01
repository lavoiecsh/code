use std::fs::read_to_string;
use crate::solver::AdventSolver;

pub struct Advent2022Day01Solver {
    calories: Vec<Vec<usize>>
}

impl Advent2022Day01Solver {
    pub fn new() -> Self {
        let mut calories = Vec::new();
        let text = read_to_string("src/year2022/day01.txt").unwrap();
        let lines: Vec<&str> = text.trim().lines().collect();
        let mut elf: Vec<usize> = Vec::new();
        for line in lines {
            if line.is_empty() {
                calories.push(elf);
                elf = Vec::new();
            } else {
                elf.push(line.parse().unwrap());
            }
        }
        calories.push(elf);
        Self {
            calories
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
        elves.iter().skip(elves.len() - 3).sum()
    }
}
