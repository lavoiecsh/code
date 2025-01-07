use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};

pub struct Advent2018Day12Solver {
    initial_state: Vec<char>,
    rules: HashMap<VecDeque<char>, char>,
}

impl Advent2018Day12Solver {
    pub fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let initial_state = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .chars()
            .collect();
        lines.next();
        Self {
            initial_state,
            rules: lines
                .filter_map(|l| l.split(" => ").collect_tuple())
                .map(|(f, t)| (f.chars().collect(), t.chars().next().unwrap()))
                .collect(),
        }
    }
}

impl AdventSolver for Advent2018Day12Solver {
    fn solve_part1(&self) -> usize {
        let mut tunnel = Tunnel::new(&self.rules, &self.initial_state);
        tunnel.generate(20);
        tunnel.score() as usize
    }

    fn solve_part2(&self) -> usize {
        let mut tunnel = Tunnel::new(&self.rules, &self.initial_state);
        tunnel.generate_many(50000000000);
        tunnel.score() as usize
    }
}

struct Tunnel<'a> {
    rules: &'a HashMap<VecDeque<char>, char>,
    state: VecDeque<char>,
    zero: isize,
}

impl Debug for Tunnel<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:4} {:3} {}",
            self.state.len(),
            self.zero,
            self.state.iter().join("")
        ))
    }
}

impl<'a> Tunnel<'a> {
    fn new(rules: &'a HashMap<VecDeque<char>, char>, initial_state: &[char]) -> Self {
        Self {
            rules,
            state: initial_state.iter().cloned().collect(),
            zero: 0,
        }
    }

    fn score(&self) -> isize {
        (0..self.state.len())
            .filter(|i| self.state[*i] == '#')
            .map(|i| i as isize - self.zero)
            .sum()
    }

    fn generate(&mut self, generations: usize) {
        (0..generations).for_each(|_| self.iterate());
    }

    fn generate_many(&mut self, generations: usize) {
        let mut seen: Vec<(VecDeque<char>, isize)> = vec![(self.state.clone(), self.zero)];
        let mut generation = 1;
        self.iterate();
        while !seen.iter().any(|s| s.0 == self.state) {
            seen.push((self.state.clone(), self.zero));
            self.iterate();
            generation += 1;
        }
        let position = seen.iter().position(|s| s.0 == self.state).unwrap();
        let length = generation - position;
        let count = (generations - position) / length;
        let remaining = generations - count * length - position;

        self.state = seen[position + remaining].0.clone();
        self.zero = seen[remaining + position].1 + count as isize * (self.zero - seen[position].1);
    }

    fn iterate(&mut self) {
        (0..5).for_each(|_| {
            self.state.push_front('.');
            self.state.push_back('.');
        });
        self.zero += 3;

        let mut next_state: VecDeque<char> = VecDeque::new();
        let mut window: VecDeque<char> = VecDeque::new();
        for c in self.state.iter() {
            window.push_back(*c);
            if window.len() < 5 {
                continue;
            }

            next_state.push_back(*self.rules.get(&window).unwrap());
            window.pop_front();
        }

        self.state = next_state;
        while self.state[0] == '.' {
            self.state.pop_front();
            self.zero -= 1;
        }

        while self.state[self.state.len() - 1] == '.' {
            self.state.pop_back();
        }
    }
}
