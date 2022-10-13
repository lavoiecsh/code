#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

const FILENAME: &str = "inputs/day06.txt";

#[allow(dead_code)]
fn read_input() -> [usize; 9] {
    let mut fish_count = [0; 9];
    fs::read_to_string(FILENAME).expect("error reading")
        .trim()
        .split(",")
        .map(|s|s.parse().expect("error parsing"))
        .for_each(|f: usize|fish_count[f] += 1);
    fish_count
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let mut fish = read_input();
    for _ in 0..80 {
        fish = iterate(&fish);
    }
    fish.iter().fold(0, |acc, count| acc + count)
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let mut fish = read_input();
    for _ in 0..256 {
        fish = iterate(&fish);
    }
    fish.iter().fold(0, |acc, count| acc + count)
}

fn iterate(fish: &[usize; 9]) -> [usize; 9] {
    let mut new_fish = [0; 9];
    for n in 0..8 {
        new_fish[n] = fish[n+1];
    }
    new_fish[6] += fish[0];
    new_fish[8] = fish[0];
    new_fish
}
