use crate::solver::AdventSolver;
use itertools::Itertools;
use std::cmp::Ordering;

pub struct Advent2024Day05Solver {
    rules: Rules,
    updates: Vec<Update>,
}

impl Advent2024Day05Solver {
    pub fn new(input: &str) -> Self {
        let lines = input.lines();
        Self {
            rules: Rules {
                rules: lines
                    .clone()
                    .take_while(|l| !l.is_empty())
                    .map(|l| {
                        l.split('|')
                            .map(|n| n.parse().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect(),
            },
            updates: lines
                .skip_while(|l| !l.is_empty())
                .skip(1)
                .map(Update::from)
                .collect(),
        }
    }
}

impl AdventSolver for Advent2024Day05Solver {
    fn solve_part1(&self) -> usize {
        self.updates
            .iter()
            .filter(|u| u.satisfies(&self.rules))
            .map(|u| u.middle_page() as usize)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.updates
            .iter()
            .filter(|u| !u.satisfies(&self.rules))
            .map(|u| u.fix(&self.rules))
            .map(|u| u.middle_page() as usize)
            .sum()
    }
}

struct Rules {
    rules: Vec<(u32, u32)>,
}

impl Rules {
    fn cmp(&self, a: &u32, b: &u32) -> Ordering {
        if self.rules.iter().any(|r| r.0 == *a && r.1 == *b) {
            Ordering::Less
        } else if self.rules.iter().any(|r| r.1 == *a && r.0 == *b) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Debug)]
struct Update {
    update: Vec<u32>,
}

impl Update {
    fn middle_page(&self) -> u32 {
        self.update[self.update.len() / 2]
    }

    fn satisfies(&self, rules: &Rules) -> bool {
        for i in 0..self.update.len() {
            for j in i + 1..self.update.len() {
                if rules.cmp(&self.update[i], &self.update[j]) == Ordering::Greater {
                    return false;
                }
            }
        }
        true
    }

    fn fix(&self, rules: &Rules) -> Self {
        Self {
            update: self
                .update
                .iter()
                .cloned()
                .sorted_by(|a, b| rules.cmp(a, b))
                .collect(),
        }
    }
}

impl From<&str> for Update {
    fn from(value: &str) -> Self {
        Self {
            update: value.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn finds_correctly_ordered_updates() {
        let solver = Advent2024Day05Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 143);
    }

    #[test]
    fn finds_proper_ordering() {
        let solver = Advent2024Day05Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 123);
    }
}
