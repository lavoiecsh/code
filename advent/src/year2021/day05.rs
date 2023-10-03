use std::collections::HashMap;

use crate::solver::AdventSolver;

#[derive(Clone)]
struct Segment {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Segment {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }
}

pub struct Advent2021Day05Solver {
    segments: Vec<Segment>,
}

impl Advent2021Day05Solver {
    pub fn new(input: String) -> Self {
        Self {
            segments: input
                .lines()
                .map(|s| {
                    let mut arrow = s.split(" -> ");
                    let mut left = arrow.next().unwrap().split(",").map(|s| s.parse().expect("error parsing"));
                    let mut right = arrow.next().unwrap().split(",").map(|s| s.parse().expect("error parsing"));
                    Segment {
                        x1: left.next().unwrap(),
                        y1: left.next().unwrap(),
                        x2: right.next().unwrap(),
                        y2: right.next().unwrap(),
                    }
                })
                .collect()
        }
    }
}

impl AdventSolver for Advent2021Day05Solver {
    fn solve_part1(&self) -> usize {
        let segments: Vec<Segment> = self.segments
            .iter()
            .filter(|s| s.is_horizontal_or_vertical())
            .cloned()
            .collect();
        let mut grid: HashMap<(usize, usize), usize> = HashMap::new();
        for s in segments {
            if s.x1 == s.x2 {
                for y in usize::min(s.y1, s.y2)..=usize::max(s.y1, s.y2) {
                    let pos = (s.x1, y);
                    grid.insert(pos, grid.get(&pos).unwrap_or(&0) + 1);
                }
            } else {
                for x in usize::min(s.x1, s.x2)..=usize::max(s.x1, s.x2) {
                    let pos = (x, s.y1);
                    grid.insert(pos, grid.get(&pos).unwrap_or(&0) + 1);
                }
            }
        }
        grid.iter()
            .fold(0, |acc, (_, v)| acc + if v > &1 { 1 } else { 0 })
    }

    fn solve_part2(&self) -> usize {
        let mut grid: HashMap<(usize, usize), usize> = HashMap::new();
        for s in &self.segments {
            if s.x1 == s.x2 {
                for y in usize::min(s.y1, s.y2)..=usize::max(s.y1, s.y2) {
                    let pos = (s.x1, y);
                    grid.insert(pos, grid.get(&pos).unwrap_or(&0) + 1);
                }
            } else if s.y1 == s.y2 {
                for x in usize::min(s.x1, s.x2)..=usize::max(s.x1, s.x2) {
                    let pos = (x, s.y1);
                    grid.insert(pos, grid.get(&pos).unwrap_or(&0) + 1);
                }
            } else {
                if (s.x1 < s.x2 && s.y1 < s.y2) || (s.x1 > s.x2 && s.y1 > s.y2) {
                    let mut x = usize::min(s.x1, s.x2);
                    let mut y = usize::min(s.y1, s.y2);
                    while x <= usize::max(s.x1, s.x2) {
                        let pos = (x, y);
                        grid.insert(pos, grid.get(&pos).unwrap_or(&0) + 1);
                        x += 1;
                        y += 1;
                    }
                } else {
                    let mut x = usize::min(s.x1, s.x2);
                    let mut y = usize::max(s.y1, s.y2);
                    while x <= usize::max(s.x1, s.x2) {
                        let pos = (x, y);
                        grid.insert(pos, grid.get(&pos).unwrap_or(&0) + 1);
                        x += 1;
                        y -= 1;
                    }
                }
            }
        }
        grid.iter()
            .fold(0, |acc, (_, v)| acc + if v > &1 { 1 } else { 0 })
    }
}
