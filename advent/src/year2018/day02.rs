use std::collections::HashMap;
use itertools::Itertools;
use crate::solver::AdventSolver;

pub struct Advent2018Day02Solver {
    ids: Vec<Id>,
}

impl Advent2018Day02Solver {
    pub fn new(input: String) -> Self {
        Self { ids: input.lines().map(Id::new).collect() }
    }
}

impl AdventSolver for Advent2018Day02Solver {
    fn solve_part1(&self) -> usize {
        self.ids.iter().filter(|id| id.has_twice()).count() *
            self.ids.iter().filter(|id| id.has_thrice()).count()
    }

    fn solve_part2_string(&self) -> String {
        self.ids.iter()
            .flat_map(|id| self.ids.iter().map(|id2| id.common_letters(id2)))
            .find(|(_,c)| *c == 1)
            .unwrap()
            .0
    }
}

struct Id {
    id: Vec<char>,
    counts: HashMap<char, usize>,
}

impl Id {
    fn new(input: &str) -> Self {
        let mut counts = HashMap::new();
        input.chars()
            .for_each(|c| *counts.entry(c).or_insert(0) += 1);
        Self { id: input.chars().collect(), counts }
    }

    fn has_twice(&self) -> bool {
        self.counts.values().contains(&2)
    }

    fn has_thrice(&self) -> bool {
        self.counts.values().contains(&3)
    }

    fn common_letters(&self, other: &Self) -> (String, usize) {
        let mut common = String::new();
        for i in 0..self.id.len() {
            if self.id[i] == other.id[i] {
                common.push(self.id[i]);
            }
        }
        let difference_count = self.id.len() - common.len();
        (common, difference_count)
    }
}
