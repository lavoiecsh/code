use crate::solver::AdventSolver;
use itertools::Itertools;
use std::fmt::Write;

pub struct Advent2017Day10Solver {
    lengths: Vec<u8>,
    length_bytes: Vec<u8>,
}

impl Advent2017Day10Solver {
    pub fn new(input: &str) -> Self {
        Self {
            lengths: input.split(",").map(|l| l.parse().unwrap()).collect(),
            length_bytes: input.chars().map(|c| c as u8).collect(),
        }
    }
}

impl AdventSolver for Advent2017Day10Solver {
    fn solve_part1(&self) -> usize {
        let mut knot_hash = KnotHash::new();
        knot_hash.round(&self.lengths);
        knot_hash.numbers[0] as usize * knot_hash.numbers[1] as usize
    }

    fn solve_part2_string(&self) -> String {
        KnotHash::new().hash(&self.length_bytes)
    }
}

pub struct KnotHash {
    numbers: Vec<u8>,
    position: usize,
    skip: usize,
}

impl KnotHash {
    pub fn new() -> Self {
        Self {
            numbers: (0..=255).collect(),
            position: 0,
            skip: 0,
        }
    }

    fn round(&mut self, lengths: &[u8]) {
        lengths.iter().for_each(|l| self.reverse(*l as usize));
    }

    fn reverse(&mut self, length: usize) {
        for i in 0..length / 2 {
            let x = (self.position + i) % self.numbers.len();
            let y = (self.position + length - i - 1) % self.numbers.len();
            self.numbers.swap(x, y);
        }

        self.position = (self.position + length + self.skip) % self.numbers.len();
        self.skip += 1;
    }

    pub fn hash(&mut self, lengths: &Vec<u8>) -> String {
        let mut extended_lengths = lengths.to_owned();
        extended_lengths.extend(vec![17, 31, 73, 47, 23]);
        (0..64).for_each(|_| self.round(&extended_lengths));
        self.dense_hash()
    }

    fn dense_hash(&self) -> String {
        self.numbers
            .iter()
            .chunks(16)
            .into_iter()
            .fold(String::new(), |mut a, c| {
                let _ = write!(a, "{:02x}", c.fold(0, |acc, cur| acc ^ cur));
                a
            })
    }
}
