use std::collections::HashSet;
use crate::solver::AdventSolver;

pub struct Advent2018Day01Solver {
    frequencies: Vec<i64>,
}

impl Advent2018Day01Solver {
    pub fn new(input: String) -> Self {
        Self { frequencies: input.lines().map(|l| l.parse().unwrap()).collect() }
    }
}

impl AdventSolver for Advent2018Day01Solver {
    fn solve_part1(&self) -> usize {
        self.frequencies.iter().sum::<i64>() as usize
    }

    fn solve_part2(&self) -> usize {
        let mut frequency: i64 = 0;
        let mut seen: HashSet<i64> = HashSet::new();
        let mut index = 0;
        while !seen.contains(&frequency) {
            seen.insert(frequency);
            frequency += self.frequencies[index];
            index += 1;
            index %= self.frequencies.len();
        }
        frequency as usize
    }
}
