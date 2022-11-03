use std::fs;
use itertools::Itertools;

const FILENAME: &str = "inputs/day17.txt";

fn read_input() -> Vec<usize> {
    fs::read_to_string(FILENAME)
        .expect("error reading")
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}

pub fn part1() -> usize {
    const LIMIT: usize = 150;
    let containers = read_input();
    let mut count: usize = 0;
    let max: usize = usize::pow(2, containers.len() as u32);
    for i in 0..max {
        let total = compute_total(&containers, i);
        if total == LIMIT {
            count += 1;
        }
    }
    count
}

pub fn part2() -> usize {
    const LIMIT: usize = 150;
    let containers = read_input();
    let mut min: usize = containers.len();
    let mut count: usize = 0;
    let max: usize = usize::pow(2, containers.len() as u32);
    for i in 0..max {
        let total = compute_total(&containers, i);
        if total != LIMIT {
            continue;
        }
        let container_count = compute_count(i);
        if container_count > min {
            continue;
        }
        if container_count == min {
            count += 1;
            continue;
        }
        if container_count < min {
            count = 1;
            min = container_count;
        }
    }
    count
}

fn compute_total(containers: &Vec<usize>, index: usize) -> usize {
    let mut total: usize = 0;
    for i in 0..containers.len() {
        if index & usize::pow(2, i as u32) != 0 {
            total += containers[i];
        }
    }
    total
}

fn compute_count(index: usize) -> usize {
    if index == 0 {
        return 0;
    }
    compute_count(index / 2) + index % 2
}
