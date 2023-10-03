use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2022Day03Solver {
    rucksacks: Vec<String>,
}

impl Advent2022Day03Solver {
    pub fn new(input: String) -> Self {
        Self {
            rucksacks: input
                .trim()
                .lines()
                .map(|l| l.to_string())
                .collect()
        }
    }
}

impl AdventSolver for Advent2022Day03Solver {
    fn solve_part1(&self) -> usize {
        self.rucksacks
            .iter()
            .map(compartmentalize)
            .map(identify_duplicate)
            .map(prioritize)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.rucksacks
            .iter()
            .chunks(3)
            .into_iter()
            .map(|c| c.map(|s| s.clone()).collect::<Vec<String>>())
            .map(identify_triplicate)
            .map(prioritize)
            .sum()
    }
}

fn compartmentalize(rucksack: &String) -> (Vec<char>, Vec<char>) {
    let half_length = rucksack.len() / 2;
    (rucksack.chars().take(half_length).collect(), rucksack.chars().skip(half_length).collect())
}

fn identify_duplicate(compartments: (Vec<char>, Vec<char>)) -> char {
    *compartments.0
        .iter()
        .find(|c| compartments.1.contains(c))
        .unwrap()
}

fn identify_triplicate(rucksacks: Vec<String>) -> char {
    rucksacks[0]
        .chars()
        .filter(|c| rucksacks[1].contains(*c))
        .filter(|c| rucksacks[2].contains(*c))
        .next()
        .unwrap()
}

fn prioritize(item: char) -> usize {
    if item.is_uppercase() {
        item as usize - 65 + 27
    } else {
        item as usize - 97 + 1
    }
}
