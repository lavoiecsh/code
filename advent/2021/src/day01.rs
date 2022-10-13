#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

const FILENAME: &str = "inputs/day01.txt";

#[allow(dead_code)]
fn read_input() -> Vec<usize> {
    fs::read_to_string(FILENAME)
        .expect("error reading")
        .trim()
        .lines()
        .map(|s|s.parse().expect("error parsing"))
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let depths = read_input();
    let mut prev: usize = 10000000;
    let mut count: usize = 0;
    for d in depths {
        if d > prev {
            count += 1;
        }
        prev = d;
    }
    count
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let depths = read_input();
    let mut prev: usize = 100000;
    let mut count: usize = 0;
    for i in 2..depths.len() {
        let sum = depths[i-2] + depths[i-1] + depths[i];
        if sum > prev {
            count += 1;
        }
        prev = sum;
    }
    count
}
