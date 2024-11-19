use crate::solver::AdventSolver;

pub struct Advent2017Day02Solver {
    numbers: Vec<Vec<usize>>,
}

impl Advent2017Day02Solver {
    pub fn new(input: &str) -> Self {
        Self {
            numbers: input
                .lines()
                .map(|l| l.split("\t").map(|n| n.parse().unwrap()).collect())
                .collect(),
        }
    }
}

impl AdventSolver for Advent2017Day02Solver {
    fn solve_part1(&self) -> usize {
        self.numbers
            .iter()
            .map(|l| l.iter().max().unwrap() - l.iter().min().unwrap())
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.numbers
            .iter()
            .map(|l| {
                for i in 0..l.len() {
                    for j in i + 1..l.len() {
                        if l[i] % l[j] == 0 {
                            return l[i] / l[j];
                        }
                        if l[j] % l[i] == 0 {
                            return l[j] / l[i];
                        }
                    }
                }
                0
            })
            .sum()
    }
}
