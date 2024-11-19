use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};

use itertools::Itertools;
use regex::Regex;

use crate::solver::AdventSolver;

const BOTTOM: u8 = 1;
const TOP: u8 = 4;

pub struct Advent2016Day11Solver {
    generators: HashMap<String, u8>,
    microchips: HashMap<String, u8>,
}

impl Advent2016Day11Solver {
    pub fn new(input: &str) -> Self {
        let generator_re = Regex::new(r"a (\w+) generator").unwrap();
        let microchip_re = Regex::new(r"a (\w+)-compatible microchip").unwrap();
        let mut generators = HashMap::new();
        let mut microchips = HashMap::new();
        input.lines().enumerate().for_each(|(i, l)| {
            generator_re.captures_iter(l).for_each(|c| {
                generators.insert(c.get(1).unwrap().as_str().to_string(), i as u8 + 1);
            });
            microchip_re.captures_iter(l).for_each(|c| {
                microchips.insert(c.get(1).unwrap().as_str().to_string(), i as u8 + 1);
            });
        });
        Self {
            generators,
            microchips,
        }
    }

    fn generators(&self) -> Vec<u8> {
        self.generators
            .iter()
            .sorted_by_key(|(k, _)| *k)
            .map(|(_, v)| *v)
            .collect()
    }

    fn microchips(&self) -> Vec<u8> {
        self.microchips
            .iter()
            .sorted_by_key(|(k, _)| *k)
            .map(|(_, v)| *v)
            .collect()
    }
}

impl AdventSolver for Advent2016Day11Solver {
    fn solve_part1(&self) -> usize {
        let mut states: HashSet<State> = HashSet::new();
        states.insert(State::init(self));
        let mut count = 0;
        while !states.iter().any(|s| s.is_completed()) {
            states = states.iter().flat_map(|s| s.next_states()).collect();
            count += 1;
        }
        count
    }

    fn solve_part2(&self) -> usize {
        let mut states: HashSet<State> = HashSet::new();
        let mut state = State::init(self);
        state.generators.extend([BOTTOM, BOTTOM]);
        state.microchips.extend([BOTTOM, BOTTOM]);
        states.insert(state);
        let mut count = 0;
        while !states.iter().any(|s| s.is_completed()) {
            states = states.iter().flat_map(|s| s.next_states()).collect();
            count += 1;
        }
        count
    }
}

#[derive(Eq, Hash, PartialEq)]
struct State {
    generators: Vec<u8>,
    microchips: Vec<u8>,
    elevator: u8,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} [{}]",
            self.elevator,
            (0..self.generators.len())
                .map(|i| format!("({},{})", self.generators[i], self.microchips[i]))
                .join(" ")
        ))
    }
}

impl State {
    fn init(advent: &Advent2016Day11Solver) -> Self {
        Self {
            generators: advent.generators(),
            microchips: advent.microchips(),
            elevator: BOTTOM,
        }
    }

    fn is_completed(&self) -> bool {
        self.generators.iter().all(|g| *g == TOP) && self.microchips.iter().all(|m| *m == TOP)
    }

    fn is_valid(&self) -> bool {
        !self.is_invalid()
    }

    fn is_invalid(&self) -> bool {
        (0..self.microchips.len())
            .filter(|mi| self.generators[*mi] != self.microchips[*mi])
            .any(|mi| {
                (0..self.generators.len()).any(|gi| self.generators[gi] == self.microchips[mi])
            })
    }

    fn next_states(&self) -> impl Iterator<Item = State> {
        let mut next_states = Vec::new();
        if self.elevator != BOTTOM {
            next_states.extend(self.next_states_inner(|x| x - 1));
        }
        if self.elevator != TOP {
            next_states.extend(self.next_states_inner(|x| x + 1));
        }
        next_states.into_iter().filter(|s| s.is_valid())
    }

    fn next_states_inner(&self, modification: fn(u8) -> u8) -> impl Iterator<Item = State> {
        let elevator = modification(self.elevator);
        let mut next_states = Vec::new();
        let gis: Vec<usize> = (0..self.generators.len())
            .filter(|gi| self.generators[*gi] == self.elevator)
            .collect();
        let mis: Vec<usize> = (0..self.microchips.len())
            .filter(|mi| self.microchips[*mi] == self.elevator)
            .collect();
        gis.iter().for_each(|gi1| {
            let gc = modify_vec(&self.generators, *gi1, modification);
            next_states.push(Self {
                generators: gc.clone(),
                microchips: self.microchips.clone(),
                elevator,
            });
            next_states.extend(gis.iter().filter(|gi2| gi1 != *gi2).map(|gi2| Self {
                generators: modify_vec(&gc, *gi2, modification),
                microchips: self.microchips.clone(),
                elevator,
            }));
            next_states.extend(mis.iter().map(|mi| Self {
                generators: gc.clone(),
                microchips: modify_vec(&self.microchips, *mi, modification),
                elevator,
            }));
        });
        mis.iter().for_each(|mi1| {
            let mc = modify_vec(&self.microchips, *mi1, modification);
            next_states.push(Self {
                generators: self.generators.clone(),
                microchips: mc.clone(),
                elevator,
            });
            next_states.extend(mis.iter().filter(|mi2| mi1 != *mi2).map(|mi2| Self {
                generators: self.generators.clone(),
                microchips: modify_vec(&mc, *mi2, modification),
                elevator,
            }));
        });
        next_states.into_iter()
    }
}

fn modify_vec(initial: &[u8], index: usize, modification: fn(u8) -> u8) -> Vec<u8> {
    let mut clone = initial.to_owned();
    clone[index] = modification(clone[index]);
    clone
}
