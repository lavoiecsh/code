use std::collections::HashMap;
use crate::solver::AdventSolver;

pub struct Advent2016Day06Solver {
    messages: Vec<String>,
}

impl Advent2016Day06Solver {
    pub fn new(input: String) -> Self {
        Self { messages: input.lines().map(|s| s.to_string()).collect() }
    }

    fn count_characters(&self) -> Vec<HashMap<char, usize>> {
        let mut counts: Vec<HashMap<char, usize>> = (0..self.messages.first().unwrap().len()).map(|_| HashMap::new()).collect();
        self.messages.iter()
            .for_each(|message| {
                message.chars().enumerate().for_each(|(i, c)| {
                    counts[i].entry(c).and_modify(|n| *n += 1).or_insert(1);
                });
            });
        counts
    }
}

impl AdventSolver for Advent2016Day06Solver {
    fn solve_part1_string(&self) -> String {
        self.count_characters().iter()
            .map(|hm| hm.iter().max_by(|l, r| l.1.cmp(r.1)).unwrap().0)
            .collect()
    }

    fn solve_part2_string(&self) -> String {
        self.count_characters().iter()
            .map(|hm| hm.iter().min_by(|l, r| l.1.cmp(r.1)).unwrap().0)
            .collect()
    }
}
