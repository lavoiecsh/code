use crate::solver::AdventSolver;

pub struct Advent2022Day01Solver {
    calories: Vec<Vec<usize>>,
}

impl Advent2022Day01Solver {
    pub fn new(input: &str) -> Self {
        Self {
            calories: input
                .split("\n\n")
                .map(|e| e.lines().map(|c| c.parse().unwrap()).collect())
                .collect(),
        }
    }
}

impl AdventSolver for Advent2022Day01Solver {
    fn solve_part1(&self) -> usize {
        self.calories
            .iter()
            .map(|e| e.iter().sum::<usize>())
            .max()
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        let mut elves: Vec<usize> = self
            .calories
            .iter()
            .map(|e| e.iter().sum::<usize>())
            .collect();
        elves.sort();
        elves.iter().rev().take(3).sum()
    }
}
