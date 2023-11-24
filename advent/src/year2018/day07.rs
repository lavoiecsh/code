use std::collections::VecDeque;
use itertools::Itertools;
use regex::{Match, Regex};

use crate::solver::AdventSolver;

pub struct Advent2018Day07Solver {
    requirements: Vec<Vec<usize>>,
}

impl Advent2018Day07Solver {
    pub fn new(input: String) -> Self {
        let re = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
        let mut requirements: Vec<Vec<usize>> = (0..26).map(|_| Vec::new()).collect();
        let to_index = |c: Option<Match>| c.unwrap().as_str().chars().next().unwrap() as usize - 'A' as usize;
        input.lines()
            .filter_map(|l| re.captures(l))
            .for_each(|c| requirements[to_index(c.get(2))].push(to_index(c.get(1))));
        Self { requirements }
    }
}

impl AdventSolver for Advent2018Day07Solver {
    fn solve_part1_string(&self) -> String {
        let mut completed: Vec<usize> = Vec::new();
        while completed.len() != 26 {
            let completable = (0..26)
                .filter(|c| !completed.contains(c))
                .filter(|c| self.requirements[*c].iter().all(|r| completed.contains(r)))
                .next()
                .unwrap();
            completed.push(completable);
        }
        completed.iter()
            .map(|c| *c as u8 + 'A' as u8)
            .map(|c| c as char)
            .collect()
    }

    fn solve_part2(&self) -> usize {
        let mut completed: Vec<usize> = Vec::new();
        let mut workers: VecDeque<(usize, usize)> = VecDeque::new();
        let mut time = 0;
        while completed.len() != 26 {
            while workers.len() > 0 && workers[0].0 == 0 {
                completed.push(workers.pop_front().unwrap().1);
            }

            let mut completable: VecDeque<usize> = (0..26)
                .filter(|c| !completed.contains(c))
                .filter(|c| workers.iter().all(|(_,w)| c != w))
                .filter(|c| self.requirements[*c].iter().all(|r| completed.contains(r)))
                .collect();

            while workers.len() != 5 && !completable.is_empty() {
                let next = completable.pop_front().unwrap();
                workers.push_back((60 + next + 1, next));
            }

            workers = workers.into_iter().sorted_by_key(|w| w.0).collect();
            for worker in workers.iter_mut() {
                worker.0 -= 1;
            }
            time += 1;
        }
        time - 1
    }
}
