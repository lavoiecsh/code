use std::collections::{HashMap, HashSet};

use multimap::MultiMap;

use crate::solver::AdventSolver;

type Molecule = Vec<String>;
type Replacements = MultiMap<String, Molecule>;
type ReverseReplacements = HashMap<Molecule, String>;

pub struct Advent2015Day19Solver {
    replacements: Replacements,
    medicine: Molecule,
}

impl Advent2015Day19Solver {
    pub fn new(input: String) -> Self {
        let lines: Vec<String> = input
            .lines()
            .map(String::from)
            .collect();
        let mut replacements: Replacements = MultiMap::new();
        for l in lines.iter().take_while(|l| !l.is_empty()) {
            let mut s = l.split(" => ");
            let from = s.next().unwrap().to_string();
            let to = s.next().unwrap().to_string();
            replacements.insert(from, split_molecule(&to));
        }
        Self {
            replacements,
            medicine: split_molecule(&lines.last().unwrap().to_string()),
        }
    }
}

impl AdventSolver for Advent2015Day19Solver {
    fn solve_part1(&self) -> usize {
        nexts(&self.replacements, &self.medicine).len()
    }

    fn solve_part2(&self) -> usize {
        let mut medicine = self.medicine.clone();
        let reverse_replacements: ReverseReplacements = self.replacements.iter_all().flat_map(|(k,vs)| vs.iter().map(|v| (v.clone(),k.clone()))).collect();
        let longest = reverse_replacements.keys().map(|k| k.len()).max().unwrap();
        let mut steps = 0;
        while medicine.len() > 1 {
            let (temp_medicine, temp_steps) = reverse_split(&reverse_replacements, longest, &medicine);
            medicine = temp_medicine;
            steps += temp_steps;
        }
        steps
    }
}

fn reverse_split(replacements: &ReverseReplacements, longest: usize, input: &Molecule) -> (Molecule, usize) {
    let mut best_i = 0;
    let mut best_j = 0;
    let mut best_rep = "".to_string();
    for i in 0..input.len() {
        for j in 2..(longest+1) {
            if i + j > input.len() {
                continue;
            }
            let section: Molecule = input.iter().skip(i).take(j).map(|s| s.clone()).collect();
            let test = replacements.get(&section);
            if test.is_none() {
                continue;
            }
            if j < best_j {
                continue;
            }
            if j > best_j {
                best_i = i;
                best_j = j;
                best_rep = test.unwrap().clone();
            }
        }
    }
    if best_j == 0 {
        return (input.clone(), 0);
    }

    let mut left = Vec::new();
    for i in 0..best_i {
        left.push(input[i].clone());
    }
    let (left, left_steps) = reverse_split(replacements, longest, &left);

    let mut right = Vec::new();
    for i in best_i+best_j..input.len() {
        right.push(input[i].clone());
    }
    let (right, right_steps) = reverse_split(replacements, longest, &right);

    let mut output = Vec::new();
    for l in left {
        output.push(l.clone());
    }
    output.push(best_rep);
    for r in right {
        output.push(r.clone());
    }
    (output, left_steps + right_steps + 1)
}

fn nexts(replacements: &Replacements, start: &Molecule) -> HashSet<Molecule> {
    let mut molecules = HashSet::new();
    for (from, tos) in replacements {
        for i in 0..start.len() {
            if from != &start[i] {
                continue;
            }
            for to in tos {
                let mut molecule: Molecule = Vec::new();
                for j in 0..i {
                    molecule.push(start[j].clone());
                }
                for j in 0..to.len() {
                    molecule.push(to[j].clone());
                }
                for j in i+1..start.len() {
                    molecule.push(start[j].clone());
                }
                molecules.insert(molecule);
            }
        }
    }
    molecules
}

fn split_molecule(molecule: &String) -> Molecule {
    let mut output = Vec::new();
    let chars: Vec<char> = molecule.chars().collect();
    for i in 1..chars.len() {
        if chars[i].is_ascii_lowercase() {
            output.push(vec!(chars[i-1],chars[i]).iter().collect::<String>());
        } else if chars[i-1].is_ascii_uppercase() {
            output.push(chars[i-1].to_string());
        }
    }
    if chars[chars.len()-1].is_ascii_uppercase() {
        output.push(chars[chars.len()-1].to_string());
    }
    output
}
