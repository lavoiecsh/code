use std::collections::HashMap;

use crate::solver::AdventSolver;

type Rules = HashMap<(char, char), char>;
type Pairs = HashMap<(char, char), usize>;

pub struct Advent2021Day14Solver {
    polymer: String,
    rules: Rules,
}

impl Advent2021Day14Solver {
    pub fn new(input: String) -> Self {
        let mut lines = input.lines();
        let polymer = lines.next().unwrap().to_string();
        lines.next();
        let mut rules = HashMap::new();
        for line in lines {
            let mut split = line.split(" -> ");
            let from = split.next().unwrap();
            let to = split.next().unwrap();
            rules.insert((from.chars().nth(0).unwrap(), from.chars().nth(1).unwrap()), to.chars().nth(0).unwrap());
        }
        Self { polymer, rules }
    }

    fn execute(&self, count: usize) -> usize {
        let mut polymer_pairs = self.build_pairs();
        for _ in 0..count {
            polymer_pairs = self.step_pairs(&polymer_pairs);
        }
        let counts = self.count_pairs(&polymer_pairs);
        counts.values().max().unwrap() - counts.values().min().unwrap()
    }

    fn build_pairs(&self) -> Pairs {
        let mut pairs: Pairs = HashMap::new();
        let mut chars = self.polymer.chars();
        let mut prev = chars.next().unwrap();
        for c in chars {
            pairs.insert((prev, c), 1);
            prev = c;
        }
        pairs
    }

    fn step_pairs(&self, input: &Pairs) -> Pairs {
        let mut output: Pairs = HashMap::new();
        for (pair, count) in input {
            let sep = self.rules.get(pair).unwrap();
            output.insert((pair.0, *sep), output.get(&(pair.0, *sep)).unwrap_or(&0) + count);
            output.insert((*sep, pair.1), output.get(&(*sep, pair.1)).unwrap_or(&0) + count);
        }
        output
    }

    fn count_pairs(&self, pairs: &Pairs) -> HashMap<char, usize> {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for ((c, _), count) in pairs {
            counts.insert(*c, counts.get(c).unwrap_or(&0) + count);
        }
        let last = self.polymer.chars().last().unwrap();
        counts.insert(last, counts.get(&last).unwrap() + 1);
        counts
    }
}

impl AdventSolver for Advent2021Day14Solver {
    fn solve_part1(&self) -> usize {
        self.execute(10)
    }

    fn solve_part2(&self) -> usize {
        self.execute(40)
    }
}
