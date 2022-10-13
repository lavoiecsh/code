#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

use std::collections::HashMap;
use std::cmp::{min,max};

const FILENAME: &str = "inputs/day05.txt";

#[derive(Clone)]
struct Segment {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[allow(dead_code)]
fn read_input() -> Vec<Segment> {
    fs::read_to_string(FILENAME).expect("error reading")
        .trim()
        .lines()
        .map(|s| {
            let mut arrow = s.split(" -> ");
            let mut left = arrow.next().unwrap().split(",").map(|s|s.parse().expect("error parsing"));
            let mut right = arrow.next().unwrap().split(",").map(|s|s.parse().expect("error parsing"));
            Segment {
                x1: left.next().unwrap(),
                y1: left.next().unwrap(),
                x2: right.next().unwrap(),
                y2: right.next().unwrap(),
            }
        })
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let segments: Vec<Segment> = read_input()
        .iter()
        .filter(|s|is_horizontal_or_vertical(s))
        .cloned()
        .collect();
    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();
    for s in segments {
        if s.x1 == s.x2 {
            for y in min(s.y1, s.y2)..=max(s.y1, s.y2) {
                let pos = (s.x1, y);
                grid.insert(pos, grid.get(&pos).unwrap_or(&0) + 1);
            }
        } else {
            for x in min(s.x1, s.x2)..=max(s.x1, s.x2) {
                let pos = (x, s.y1);
                grid.insert(pos, grid.get(&pos).unwrap_or(&0) + 1);
            }
        }
    }
    grid.iter()
        .fold(0, |acc,(_,v)|acc + if v > &1 { 1 } else { 0 })
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let segments: Vec<Segment> = read_input();
    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();
    for s in segments {
        if s.x1 == s.x2 {
            for y in min(s.y1, s.y2)..=max(s.y1, s.y2) {
                let pos = (s.x1, y);
                grid.insert(pos, grid.get(&pos).unwrap_or(&0) + 1);
            }
        } else if s.y1 == s.y2 {
            for x in min(s.x1, s.x2)..=max(s.x1, s.x2) {
                let pos = (x, s.y1);
                grid.insert(pos, grid.get(&pos).unwrap_or(&0) + 1);
            }
        } else {
            if (s.x1 < s.x2 && s.y1 < s.y2) || (s.x1 > s.x2 && s.y1 > s.y2) {
                let mut x = min(s.x1, s.x2);
                let mut y = min(s.y1, s.y2);
                while x <= max(s.x1, s.x2) {
                    let pos = (x, y);
                    grid.insert(pos, grid.get(&pos).unwrap_or(&0) + 1);
                    x += 1;
                    y += 1;
                }
            } else {
                let mut x = min(s.x1, s.x2);
                let mut y = max(s.y1, s.y2);
                while x <= max(s.x1, s.x2) {
                    let pos = (x, y);
                    grid.insert(pos, grid.get(&pos).unwrap_or(&0) + 1);
                    x += 1;
                    y -= 1;
                }
            }
        }
    }
    debug!(&grid);
    grid.iter()
        .fold(0, |acc,(_,v)|acc + if v > &1 { 1 } else { 0 })
}

fn is_horizontal_or_vertical(segment: &Segment) -> bool {
    segment.x1 == segment.x2 || segment.y1 == segment.y2
}
