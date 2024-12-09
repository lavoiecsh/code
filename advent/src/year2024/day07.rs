use crate::solver::AdventSolver;
use itertools::Itertools;
use std::str::FromStr;
use num_traits::pow;

pub struct Advent2024Day07Solver {
    equations: Vec<Equation>,
}

impl Advent2024Day07Solver {
    pub fn new(input: &str) -> Self {
        Self {
            equations: input
                .lines()
                .map(Equation::from_str)
                .filter_map(Result::ok)
                .collect(),
        }
    }
}

impl AdventSolver for Advent2024Day07Solver {
    fn solve_part1(&self) -> usize {
        self.equations.iter()
            .filter(|e| e.is_solvable(|l,r| vec![l + r, l * r]))
            .map(|e| e.left)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.equations.iter()
            .filter(|e| e.is_solvable(|l,r| vec![l + r, l * r, concatenate(l, r)]))
            .map(|e| e.left)
            .sum()
    }
}

fn concatenate(left: usize, right: usize) -> usize {
    let p = right.ilog10() + 1;
    left * pow(10, p as usize) + right
}

struct Equation {
    left: usize,
    right: Vec<usize>,
}

impl Equation {
    fn is_solvable(&self, ops: fn(usize, usize) -> Vec<usize>) -> bool {
        let mut results = vec![self.right[0]];
        for i in 1..self.right.len() {
            results = results.iter()
                .flat_map(|&r| ops(r, self.right[i]))
                .filter(|&r| r <= self.left)
                .collect();
        }
        results.contains(&self.left)
    }
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s2 = s.split(": ").collect_vec();
        Ok(Self {
            left: s2[0].parse().unwrap(),
            right: s2[1]
                .split(' ')
                .map(str::parse)
                .filter_map(Result::ok)
                .collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn finds_valid_equations_with_add_multiply() {
        let solver = Advent2024Day07Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 3749);
    }
    
    #[test]
    fn finds_valid_equations_with_add_multiply_concatenate() {
        let solver = Advent2024Day07Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 11387);
    }
}
