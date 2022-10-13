#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

const FILENAME: &str = "inputs/day03.txt";

#[allow(dead_code)]
fn read_input() -> Vec<usize>{ 
    fs::read_to_string(FILENAME).expect("error reading")
        .trim()
        .lines()
        .map(|s|usize::from_str_radix(s, 2).unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let numbers = read_input();
    let mut power = find_power(&numbers);
    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    while power > 0 {
        let c1 = count1(&numbers, power);
        if c1 > numbers.len()/2 {
            gamma += power;
        } else {
            epsilon += power;
        }
        power /= 2;
    }
    gamma * epsilon
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let numbers = read_input();
    let power = find_power(&numbers);
    let oxygen = find_rating(&numbers, power, |c0,c1| c0 > c1);
    let co2 = find_rating(&numbers, power, |c0,c1| c0 <= c1);
    debug!(oxygen);
    debug!(co2);
    oxygen * co2
}

fn find_rating(start_numbers: &Vec<usize>, start_power: usize, use0: fn (c0: usize, c1: usize) -> bool) -> usize {
    let mut numbers: Vec<usize> = start_numbers.to_vec();
    let mut power = start_power;
    while power > 0 {
        if numbers.len() == 1 {
            return numbers[0];
        }
        let c1 = count1(&numbers, power);
        let c0 = numbers.len() - c1;
        let x = if use0(c0, c1) { 0 } else { power };
        numbers = numbers.iter().cloned().filter(|n| n & power == x).collect();
        power /= 2;
    }
    assert!(numbers.len() == 1);
    numbers[0]
}

fn find_power(numbers: &Vec<usize>) -> usize {
    let largest = numbers.iter().cloned().fold(0, usize::max);
    let mut power = 1;
    while power < largest { power *= 2 };
    power / 2
}

fn count1(numbers: &Vec<usize>, power: usize) -> usize {
    numbers.iter().fold(0, |acc,n| acc + if n & power != 0 { 1 } else { 0 })
}
