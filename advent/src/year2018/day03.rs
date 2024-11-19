use crate::solver::AdventSolver;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

pub struct Advent2018Day03Solver {
    fabric: Fabric,
}

impl Advent2018Day03Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let claims = input
            .lines()
            .map(|l| {
                re.captures(l)
                    .unwrap()
                    .iter()
                    .skip(1)
                    .map(|c| c.unwrap().as_str().parse::<usize>().unwrap())
                    .collect_vec()
            })
            .map(|c| Claim {
                id: c[0],
                left: c[1],
                top: c[2],
                width: c[3],
                height: c[4],
            })
            .collect();
        Self {
            fabric: Fabric::new(claims),
        }
    }
}

impl AdventSolver for Advent2018Day03Solver {
    fn solve_part1(&self) -> usize {
        self.fabric.multiple_overlap_count()
    }

    fn solve_part2(&self) -> usize {
        self.fabric.not_overlapping()
    }
}

struct Fabric {
    claims: Vec<Claim>,
    squares: HashMap<(usize, usize), usize>,
}

impl Fabric {
    fn new(claims: Vec<Claim>) -> Self {
        let mut squares = HashMap::new();
        claims.iter().for_each(|claim| {
            for row in claim.left..claim.left + claim.width {
                for col in claim.top..claim.top + claim.height {
                    *squares.entry((row, col)).or_insert(0) += 1;
                }
            }
        });
        Self { claims, squares }
    }

    fn multiple_overlap_count(&self) -> usize {
        self.squares.values().filter(|c| **c >= 2).count()
    }

    fn not_overlapping(&self) -> usize {
        self.claims
            .iter()
            .find(|claim| {
                for row in claim.left..claim.left + claim.width {
                    for col in claim.top..claim.top + claim.height {
                        if *self.squares.get(&(row, col)).unwrap() != 1 {
                            return false;
                        }
                    }
                }
                true
            })
            .unwrap()
            .id
    }
}

struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}
