#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

const FILENAME: &str = "inputs/day07.txt";

#[allow(dead_code)]
fn read_input() -> Vec<usize> {
    fs::read_to_string(FILENAME).expect("error reading")
        .trim()
        .split(",")
        .map(|s|s.parse().expect("error parsing"))
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let crabs = read_input();
    let max: usize = *crabs.iter().max().unwrap();
    let mut best = usize::MAX;
    for n in 0..=max {
        let sum = crabs.iter().fold(0, |acc, cur| acc + if cur > &n { cur - n } else { n - cur });
        if sum < best {
            best = sum;
        }
    }
    best
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let crabs = read_input();
    let max = *crabs.iter().max().unwrap();
    let mut best = usize::MAX;
    for n in 0..=max {
        let sum = crabs.iter().fold(0, |acc, cur| acc + cost(*cur, n));
        if sum < best {
            best = sum;
        }
    }
    best
}

fn cost(crab: usize, pos: usize) -> usize {
    let dist = if crab > pos { crab - pos } else { pos - crab };
    (dist * dist + dist) / 2
}
