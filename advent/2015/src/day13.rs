use std::collections::{HashMap, HashSet};
use std::fs;
use itertools::Itertools;
use regex::Regex;
use crate::problem_solver::ProblemSolver;

struct Happiness {
    subject: String,
    value: isize,
    next_to: String,
}

pub struct Problem13Solver {
    happiness_changes: HashMap<(String, String), isize>
}

impl Problem13Solver {
    pub fn new() -> Self {
        let re = Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).").unwrap();
        Self {
            happiness_changes: fs::read_to_string("inputs/day13.txt")
                .expect("error reading")
                .trim()
                .lines()
                .map(|l| re.captures(l).expect(format!("Failed to parse: \"{}\"", l).as_str()))
                .map(|c| {
                    ((c.get(1).unwrap().as_str().to_string(), c.get(4).unwrap().as_str().to_string()),
                     c.get(3).unwrap().as_str().parse::<isize>().unwrap() * if c.get(2).unwrap().as_str() == "gain" { 1 } else { -1 })
                })
                .collect()
        }
    }
}

impl ProblemSolver for Problem13Solver {
    fn solve_part1(&self) -> usize {
        let people: HashSet<String> = self.happiness_changes.iter().flat_map(|e| [e.0.0.clone(), e.0.1.clone()]).collect();
        people
            .iter()
            .permutations(people.len())
            .unique()
            .map(|p| compute_happiness(&self.happiness_changes, p))
            .max()
            .unwrap() as usize
    }

    fn solve_part2(&self) -> usize {
        let me = "me".to_string();
        let mut happiness_changes = self.happiness_changes.clone();
        let mut people: HashSet<String> = happiness_changes.iter().flat_map(|e| [e.0.0.clone(), e.0.1.clone()]).collect();
        people.iter()
            .for_each(|p| {
                happiness_changes.insert((me.clone(), p.clone()), 0);
                happiness_changes.insert((p.clone(), me.clone()), 0);
            });
        people.insert(me.clone());
        people
            .iter()
            .permutations(people.len())
            .unique()
            .map(|p| compute_happiness(&happiness_changes, p))
            .max()
            .unwrap() as usize
    }
}

fn compute_happiness(hm: &HashMap<(String, String), isize>, arrangement: Vec<&String>) -> isize {
    let mut happiness: isize = 0;
    for i in 1..arrangement.len() {
        let a1 = arrangement[i];
        let a2 = arrangement[i-1];
        happiness += hm.get(&(a1.clone(), a2.clone())).unwrap();
        happiness += hm.get(&(a2.clone(), a1.clone())).unwrap();
    }
    let a1 = arrangement[0];
    let a2 = arrangement[arrangement.len()-1];
    happiness += hm.get(&(a1.clone(), a2.clone())).unwrap();
    happiness += hm.get(&(a2.clone(), a1.clone())).unwrap();
    happiness
}
