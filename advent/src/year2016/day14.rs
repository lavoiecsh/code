use std::collections::{HashMap, VecDeque};

use crate::solver::AdventSolver;

pub struct Advent2016Day14Solver {
    salt: String,
}

impl Advent2016Day14Solver {
    pub fn new(input: &str) -> Self {
        Self {
            salt: input.to_string(),
        }
    }
}

impl AdventSolver for Advent2016Day14Solver {
    fn solve_part1(&self) -> usize {
        let mut keygen = KeyGen::new(&self.salt, KeyGenHash::new_simple);
        keygen.generate_n(64)
    }

    fn solve_part2(&self) -> usize {
        let mut keygen = KeyGen::new(&self.salt, KeyGenHash::new_stretched);
        keygen.generate_n(64)
    }
}

struct KeyGen<'a> {
    salt: &'a str,
    hashes: HashMap<usize, KeyGenHash>,
    generator: fn(&str, usize) -> KeyGenHash,
}

#[derive(Clone)]
struct KeyGenHash {
    triples: Vec<char>,
    quintuples: Vec<char>,
}

impl<'a> KeyGen<'a> {
    fn new(salt: &'a str, generator: fn(&str, usize) -> KeyGenHash) -> Self {
        Self {
            salt,
            hashes: HashMap::new(),
            generator,
        }
    }

    fn generate_n(&mut self, n: usize) -> usize {
        let mut count = 0;
        let mut index = 0;
        while count < n {
            let current = self.get(index);
            index += 1;
            if current
                .triples
                .first()
                .is_some_and(|t| (index..index + 1000).any(|i| self.get(i).quintuples.contains(t)))
            {
                count += 1;
            }
        }
        index - 1
    }

    fn get(&mut self, index: usize) -> KeyGenHash {
        self.hashes
            .entry(index)
            .or_insert_with(|| (self.generator)(self.salt, index))
            .clone()
    }
}

impl KeyGenHash {
    fn new_simple(salt: &str, index: usize) -> Self {
        Self::new(format!("{:x}", md5::compute(format!("{salt}{index}"))))
    }

    fn new_stretched(salt: &str, index: usize) -> Self {
        let mut hash = format!("{salt}{index}");
        for _ in 0..=2016 {
            hash = format!("{:x}", md5::compute(hash));
        }
        Self::new(hash)
    }

    fn new(hash: String) -> Self {
        let hash: Vec<char> = hash.chars().collect();
        let mut triples = Vec::new();
        let mut quintuples = Vec::new();
        let mut triple_window = VecDeque::from(vec![hash[0], hash[1], hash[2]]);
        if triple_window.iter().all(|d| *d == triple_window[0]) {
            triples.push(triple_window[0]);
        }
        triple_window.pop_front();
        triple_window.push_back(hash[3]);
        if triple_window.iter().all(|d| *d == triple_window[0]) {
            triples.push(triple_window[0]);
        }
        triple_window.pop_front();
        triple_window.push_back(hash[4]);
        if triple_window.iter().all(|d| *d == triple_window[0]) {
            triples.push(triple_window[0]);
        }
        let mut quintuple_window =
            VecDeque::from(vec![hash[0], hash[1], hash[2], hash[3], hash[4]]);
        if quintuple_window.iter().all(|d| *d == quintuple_window[0]) {
            quintuples.push(quintuple_window[0]);
        }
        (5..hash.len()).for_each(|i| {
            triple_window.pop_front();
            triple_window.push_back(hash[i]);
            if triple_window.iter().all(|d| *d == triple_window[0]) {
                triples.push(hash[i]);
            }
            quintuple_window.pop_front();
            quintuple_window.push_back(hash[i]);
            if quintuple_window.iter().all(|d| *d == quintuple_window[0]) {
                quintuples.push(hash[i]);
            }
        });
        Self {
            triples,
            quintuples,
        }
    }
}
