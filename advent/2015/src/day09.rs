use std::collections::{HashMap, HashSet};
use std::fs;
use itertools::Itertools;
use regex::Regex;

const FILENAME: &str = "inputs/day09.txt";

fn read_input() -> HashMap<(String,String), usize> {
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    fs::read_to_string(FILENAME)
        .expect("error reading")
        .trim()
        .lines()
        .map(|l| {
            let m = re.captures(l).unwrap();
            ((m[1].to_string(), m[2].to_string()), m[3].parse().unwrap())
        })
        .collect()
}

pub fn part1() -> usize {
    let distances = read_input();
    let destinations: HashSet<String> = distances.iter().flat_map(|e| [e.0.0.clone(), e.0.1.clone()]).collect();
    destinations
        .iter()
        .permutations(destinations.len())
        .unique()
        .map(|p| compute_distance(&distances, p))
        .min()
        .unwrap()
}

pub fn part2() -> usize {

    let distances = read_input();
    let destinations: HashSet<String> = distances.iter().flat_map(|e| [e.0.0.clone(), e.0.1.clone()]).collect();
    destinations
        .iter()
        .permutations(destinations.len())
        .unique()
        .map(|p| compute_distance(&distances, p))
        .max()
        .unwrap()
}

fn compute_distance(distances: &HashMap<(String, String), usize>, path: Vec<&String>) -> usize {
    let mut distance: usize = 0;
    for i in 1..path.len() {
        let destination1 = path[i];
        let destination2 = path[i-1];
        let leg_distance = distances.get(&(destination1.clone(), destination2.clone()))
            .or(distances.get(&(destination2.clone(), destination1.clone())))
            .unwrap();
        distance += *leg_distance;
    }
    distance
}
