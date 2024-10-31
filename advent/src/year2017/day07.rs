use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;
use crate::solver::AdventSolver;

pub struct Advent2017Day07Solver {
    programs: Vec<Program>,
}

impl Advent2017Day07Solver {
    pub fn new(input: String) -> Self {
        let re = Regex::new(r"(\w+) \((\d+)\)").unwrap();
        Self {
            programs: input.lines()
                .map(|l| {
                    let mut s = l.split(" -> ");
                    let cap = re.captures(s.next().unwrap()).unwrap();
                    Program {
                        name: cap.get(1).unwrap().as_str().to_string(),
                        weight: cap.get(2).unwrap().as_str().parse().unwrap(),
                        above: s.next().map(|a| a.split(", ").map(String::from).collect::<Vec<String>>()).unwrap_or_default(),
                    }
                })
                .collect()
        }
    }
}

impl AdventSolver for Advent2017Day07Solver {
    fn solve_part1_string(&self) -> String {
        let above: HashSet<String> = self.programs.iter().flat_map(|p| p.above.clone()).collect();
        self.programs.iter()
            .map(|p| p.name.clone())
            .find(|n| !above.contains(n))
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        fix_weight(&self.programs)
    }
}

struct Program {
    name: String,
    weight: usize,
    above: Vec<String>,
}

fn fix_weight(programs: &[Program]) -> usize {
    let mut remaining: Vec<&Program> = programs.iter().filter(|p| !p.above.is_empty()).collect();
    let mut weights: HashMap<String, usize> = programs.iter().filter(|p| p.above.is_empty()).map(|p| (p.name.clone(), p.weight)).collect();
    while !remaining.is_empty() {
        let position = remaining.iter().position(|p| p.above.iter().all(|a| weights.contains_key(a))).unwrap();
        let program = remaining.swap_remove(position);
        let above_weights: Vec<(&String, usize)> = program.above.iter().map(|a| (a, *weights.get(a).unwrap())).sorted_by_key(|a| a.1).collect();
        if above_weights.iter().map(|a| a.1).all_equal() {
            weights.insert(program.name.clone(), program.weight + above_weights.iter().map(|a| a.1).sum::<usize>());
        } else if above_weights[0].1 != above_weights[1].1 {
            let smaller = above_weights[0].1;
            let larger = above_weights[1].1;
            let prev_weight = programs.iter().find(|p| &p.name == above_weights[0].0).unwrap().weight;
            return prev_weight + larger - smaller;
        } else {
            let smaller = above_weights[0].1;
            let larger = above_weights[above_weights.len() - 1].1;
            let prev_weight = programs.iter().find(|p| &p.name == above_weights[above_weights.len() - 1].0).unwrap().weight;
            return prev_weight + smaller - larger;
        }
    }
    0
}
