use std::collections::VecDeque;
use crate::solver::AdventSolver;

pub struct Advent2023Day09Solver {
    histories: Vec<History>,
}

impl Advent2023Day09Solver {
    pub fn new(input: String) -> Self {
        Self {
            histories: input.lines().map(History::from).collect()
        }
    }
}

impl AdventSolver for Advent2023Day09Solver {
    fn solve_part1(&self) -> usize {
        self.histories.iter()
            .map(|h| h.next_value())
            .sum::<i64>() as usize
    }

    fn solve_part2(&self) -> usize {
        self.histories.iter()
            .map(|h| h.previous_value())
            .sum::<i64>() as usize
    }
}

struct History {
    values: Vec<i64>,
}

impl History {
    fn next_value(&self) -> i64 {
        let sequences = self.sequences();
        let mut new_value = 0;
        for i in 1..sequences.len() {
            new_value += sequences[i].last().unwrap();
        }
        new_value
    }

    fn previous_value(&self) -> i64 {
        let sequences = self.sequences();
        let mut new_value = 0;
        for i in 1..sequences.len() {
            new_value = sequences[i][0] - new_value;
        }
        new_value
    }

    fn sequences(&self) -> VecDeque<Vec<i64>> {
        let mut sequences: VecDeque<Vec<i64>> = VecDeque::new();
        sequences.push_front(self.values.clone());
        while sequences.front().unwrap().iter().any(|&h| h != 0) {
            let current = sequences.front().unwrap();
            let mut next = Vec::new();
            for i in 1..current.len() {
                next.push(current[i] - current[i - 1]);
            }
            sequences.push_front(next);
        }
        sequences
    }
}

impl From<&str> for History {
    fn from(value: &str) -> Self {
        Self {
            values: value.split(" ").map(|v| v.parse().unwrap()).collect()
        }
    }
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day09Solver {
    Advent2023Day09Solver::new(String::from("\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"))
}

#[test]
fn next_values() {
    let solver = test_solver_1();
    let next_values = solver.histories.iter().map(|h| h.next_value()).collect::<Vec<i64>>();
    assert_eq!(next_values, vec!(18, 28, 68));
    assert_eq!(solver.solve_part1(), 114);
}

#[test]
fn previous_values() {
    let solver = test_solver_1();
    let next_values = solver.histories.iter().map(|h| h.previous_value()).collect::<Vec<i64>>();
    assert_eq!(next_values, vec!(-3, 0, 5));
    assert_eq!(solver.solve_part2(), 2);
}
