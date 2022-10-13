#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

use std::collections::HashMap;

const FILENAME: &str = "inputs/day14.txt";

type Rules = HashMap<(char,char), char>;
type Pairs = HashMap<(char,char), usize>;

#[allow(dead_code)]
fn read_input() -> (String, Rules) {
    let input = fs::read_to_string(FILENAME).expect("error reading").trim().to_string();
    let mut lines = input.lines();
    let start = lines.next().unwrap().to_string();
    lines.next();
    let mut rules = HashMap::new();
    for line in lines {
        let mut split = line.split(" -> ");
        let from = split.next().unwrap();
        let to = split.next().unwrap();
        rules.insert((from.chars().nth(0).unwrap(), from.chars().nth(1).unwrap()), to.chars().nth(0).unwrap());
    }
    (start, rules)
    
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let (polymer, rules) = read_input();
    execute(&polymer, &rules, 10)
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let (polymer, rules) = read_input();
    execute(&polymer, &rules, 40)
}

fn execute(polymer: &String, rules: &Rules, count: usize) -> usize {
    let mut polymer_pairs = build_pairs(&polymer);
    for _ in 0..count {
        polymer_pairs = step_pairs(&rules, &polymer_pairs);
    }
    let counts = count_pairs(&polymer_pairs, polymer.chars().last().unwrap());
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn build_pairs(polymer: &String) -> Pairs {
    let mut pairs: Pairs = HashMap::new();
    let mut chars = polymer.chars();
    let mut prev = chars.next().unwrap();
    for c in chars {
        pairs.insert((prev, c), 1);
        prev = c;
    }
    pairs
}

fn step_pairs(rules: &Rules, input: &Pairs) -> Pairs {
    let mut output: Pairs = HashMap::new();
    for (pair, count) in input {
        let sep = rules.get(&pair).unwrap();
        output.insert((pair.0, *sep), output.get(&(pair.0, *sep)).unwrap_or(&0) + count);
        output.insert((*sep, pair.1), output.get(&(*sep, pair.1)).unwrap_or(&0) + count);
    }
    output
}

fn count_pairs(pairs: &Pairs, last: char) -> HashMap<char, usize> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for ((c, _), count) in pairs {
        counts.insert(*c, counts.get(&c).unwrap_or(&0) + count);
    }
    counts.insert(last, counts.get(&last).unwrap() + 1);
    counts
}
