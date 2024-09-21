use std::collections::{HashSet, VecDeque};
use crate::solver::AdventSolver;

pub struct Advent2017Day12Solver {
    programs: Vec<Vec<usize>>,
}

impl Advent2017Day12Solver {
    pub fn new(input: String) -> Self {
        Self {
            programs: input.lines()
                .map(|l| l.split(" <-> ").nth(1).unwrap().split(", ").map(|c| c.parse().unwrap()).collect())
                .collect()
        }
    }
}

impl AdventSolver for Advent2017Day12Solver {
    fn solve_part1(&self) -> usize {
        groups(&self.programs)[0].len()
    }

    fn solve_part2(&self) -> usize {
        groups(&self.programs).len()
    }
}

fn groups(programs: &[Vec<usize>]) -> Vec<HashSet<usize>> {
    let mut connected: Vec<bool> = Vec::new();
    connected.resize(programs.len(), false);

    let mut groups: Vec<HashSet<usize>> = Vec::new();

    let mut remaining: VecDeque<usize> = (0..programs.len()).collect();

    while let Some(current) = remaining.pop_front() {
        if connected[current] { continue }

        let mut connections: HashSet<usize> = programs[current].iter().cloned().collect();
        connections.insert(current);
        let mut previous_size = 0;
        while connections.len() != previous_size {
            previous_size = connections.len();
            connections = connections.iter().flat_map(|c| programs[*c].clone()).collect();
        }

        connections.iter().for_each(|c| connected[*c] = true);
        groups.push(connections);
    }
    groups
}
