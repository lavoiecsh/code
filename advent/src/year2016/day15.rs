use crate::solver::AdventSolver;
use regex::Regex;

pub struct Advent2016Day15Solver {
    discs: Vec<Disc>,
}

impl Advent2016Day15Solver {
    pub fn new(input: &str) -> Self {
        let re =
            Regex::new(r"Disc #(\d) has (\d+) positions; at time=0, it is at position (\d+)\.")
                .unwrap();
        Self {
            discs: input
                .lines()
                .map(|l| {
                    let captures = re.captures(l).unwrap();
                    Disc {
                        index: captures.get(1).unwrap().as_str().parse().unwrap(),
                        positions: captures.get(2).unwrap().as_str().parse().unwrap(),
                        position: captures.get(3).unwrap().as_str().parse().unwrap(),
                    }
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2016Day15Solver {
    fn solve_part1(&self) -> usize {
        (0..)
            .find(|t| self.discs.iter().all(|d| d.drop_at(*t) == 0))
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        let mut discs = self.discs.clone();
        discs.push(Disc {
            index: self.discs.len() + 1,
            position: 0,
            positions: 11,
        });
        (0..)
            .find(|t| discs.iter().all(|d| d.drop_at(*t) == 0))
            .unwrap()
    }
}

#[derive(Clone)]
struct Disc {
    index: usize,
    positions: usize,
    position: usize,
}

impl Disc {
    fn drop_at(&self, time: usize) -> usize {
        (self.position + time + self.index) % self.positions
    }
}
