use crate::solver::AdventSolver;
use itertools::Itertools;
use prime_factorization::Factorization;
use regex::{Match, Regex};
use std::collections::HashMap;

pub struct Advent2023Day08Solver {
    instructions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
}

impl Advent2023Day08Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
        let s = |c: Option<Match>| c.unwrap().as_str().to_string();
        let mut lines = input.lines();
        Self {
            instructions: lines.next().unwrap().chars().collect(),
            nodes: lines
                .skip(1)
                .map(|l| re.captures(l).unwrap())
                .map(|c| (s(c.get(1)), (s(c.get(2)), s(c.get(3)))))
                .collect(),
        }
    }

    fn steps_to_z(&self, start: &str) -> u64 {
        let mut current = String::from(start);
        let mut count = 0;
        let mut i = 0;
        while !current.ends_with("Z") {
            let (left, right) = self.nodes.get(&current).unwrap();
            current = (if self.instructions[i] == 'L' {
                left
            } else {
                right
            })
            .clone();
            i += 1;
            i %= self.instructions.len();
            count += 1;
        }
        count
    }
}

impl AdventSolver for Advent2023Day08Solver {
    fn solve_part1(&self) -> usize {
        self.steps_to_z("AAA") as usize
    }

    fn solve_part2(&self) -> usize {
        self.nodes
            .keys()
            .filter(|k| k.ends_with("A"))
            .map(|c| self.steps_to_z(c))
            .flat_map(|c| Factorization::<u64>::run(c).factors)
            .unique()
            .product::<u64>() as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn single_path() {
        let solver = Advent2023Day08Solver::new(
            "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
",
        );
        assert_eq!(solver.solve_part1(), 6);
    }

    #[test]
    fn multiple_paths() {
        let solver = Advent2023Day08Solver::new(
            "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
",
        );
        assert_eq!(solver.solve_part2(), 6);
    }
}
