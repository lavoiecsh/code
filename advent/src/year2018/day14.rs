use itertools::Itertools;
use crate::solver::AdventSolver;

pub struct Advent2018Day14Solver {
    input: String,
}

impl Advent2018Day14Solver {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl AdventSolver for Advent2018Day14Solver {
    fn solve_part1_string(&self) -> String {
        let mut kitchen = Kitchen::new();
        let scores = kitchen.ten_after(self.input.parse().unwrap());
        scores.iter().join("")
    }

    fn solve_part2(&self) -> usize {
        let mut kitchen = Kitchen::new();
        let index = kitchen.until_score(&self.input);
        index
    }
}

struct Kitchen {
    recipes: Vec<u8>,
    elf1: usize,
    elf2: usize,
}

impl Kitchen {
    fn new() -> Self {
        Self { recipes: vec!(3, 7), elf1: 0, elf2: 1 }
    }

    fn ten_after(&mut self, count: usize) -> Vec<u8> {
        while self.recipes.len() < count + 10 {
            self.iterate();
        }
        self.recipes[count..count + 10].iter().cloned().collect()
    }

    fn until_score(&mut self, scores: &str) -> usize {
        let sequence: Vec<u8> = scores.chars().map(|c| c as u8 - '0' as u8).collect();
        while self.recipes.len() < sequence.len() + 1 ||
            (self.recipes[self.recipes.len() - sequence.len()..] != sequence &&
                self.recipes[self.recipes.len() - sequence.len() - 1..self.recipes.len() - 1] != sequence) {
            self.iterate();
        }
        self.recipes.len() - sequence.len() -
            if self.recipes[self.recipes.len() - sequence.len()..] == sequence { 0 } else { 1 }
    }

    fn iterate(&mut self) {
        let score1 = self.recipes[self.elf1];
        let score2 = self.recipes[self.elf2];
        let sum = score1 + score2;
        if sum >= 10 {
            self.recipes.push(sum / 10);
        }
        self.recipes.push(sum % 10);
        self.elf1 += 1 + score1 as usize;
        self.elf1 %= self.recipes.len();
        self.elf2 += 1 + score2 as usize;
        self.elf2 %= self.recipes.len();
    }
}
