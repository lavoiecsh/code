use std::collections::HashSet;
use itertools::Itertools;
use crate::solver::AdventSolver;

pub struct Advent2017Day04Solver {
    passphrases: Vec<Passphrase>,
}

impl Advent2017Day04Solver {
    pub fn new(input: String) -> Self {
        Self {
            passphrases: input.lines()
                .map(|l| Passphrase { words: l.split(" ").map(|w| w.to_string()).collect() })
                .collect()
        }
    }
}

impl AdventSolver for Advent2017Day04Solver {
    fn solve_part1(&self) -> usize {
        self.passphrases.iter()
            .filter(|p| p.is_valid_part_1())
            .count()
    }

    fn solve_part2(&self) -> usize {
        self.passphrases.iter()
            .filter(|p| p.is_valid_part_2())
            .count()
    }
}

struct Passphrase {
    words: Vec<String>,
}

impl Passphrase {
    fn is_valid_part_1(&self) -> bool {
        self.words.len() == self.words.iter().cloned().collect::<HashSet<String>>().len()
    }

    fn is_valid_part_2(&self) -> bool {
        self.words.len() == self.words.iter()
            .map(|w| w.chars().sorted().join(""))
            .collect::<HashSet<String>>().len()
    }
}
