use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::ops::RangeInclusive;

use itertools::Itertools;
use regex::{Match, Regex};

use Falling::*;

use crate::solver::AdventSolver;

pub struct Advent2023Day22Solver {
    stack: Stack,
}

impl Advent2023Day22Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();
        let parse = |c: Option<Match>| c.unwrap().as_str().parse().unwrap();
        Self {
            stack: Stack::new(
                input
                    .lines()
                    .filter_map(|l| re.captures(l))
                    .map(|cap| {
                        (
                            parse(cap.get(1))..=parse(cap.get(4)),
                            parse(cap.get(2))..=parse(cap.get(5)),
                            parse(cap.get(3))..=parse(cap.get(6)),
                        )
                    })
                    .collect(),
            ),
        }
    }
}

impl AdventSolver for Advent2023Day22Solver {
    fn solve_part1(&self) -> usize {
        self.stack.disintegratable().count()
    }

    fn solve_part2(&self) -> usize {
        self.stack.falling()
    }
}

struct Stack {
    bricks: Vec<Brick>,
}

impl Stack {
    fn new(brick_defs: Vec<BrickDef>) -> Self {
        let mut below: Vec<Vec<usize>> = vec![];
        below.resize_with(brick_defs.len(), Vec::new);
        let mut above: Vec<Vec<usize>> = vec![];
        above.resize_with(brick_defs.len(), Vec::new);
        for i in 0..brick_defs.len() {
            for j in i + 1..brick_defs.len() {
                if !overlaps(&brick_defs[i], &brick_defs[j]) {
                    continue;
                }
                if brick_defs[i].2.start() < brick_defs[j].2.start() {
                    below[j].push(i);
                    above[i].push(j);
                } else {
                    below[i].push(j);
                    above[j].push(i);
                }
            }
        }

        let mut bricks = vec![];
        for i in 0..brick_defs.len() {
            bricks.push(Brick {
                index: i,
                x: brick_defs[i].0.clone(),
                y: brick_defs[i].1.clone(),
                z: brick_defs[i].2.clone(),
                below: below[i].clone(),
                above: above[i].clone(),
                supporting: vec![],
                supported_by: vec![],
            });
        }
        let mut s = Self { bricks };
        s.settle();
        s
    }

    fn settle(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..self.bricks.len() {
                let brick = &self.bricks[i];
                let min = brick
                    .below
                    .iter()
                    .map(|&bi| *self.bricks[bi].z.end())
                    .max()
                    .unwrap_or(0);
                if *brick.z.start() == min + 1 {
                    continue;
                }
                self.bricks[i].lower_to(min + 1);
                changed = true;
            }
        }
        for i in 0..self.bricks.len() {
            let z_start = *self.bricks[i].z.start();
            self.bricks[i].supported_by = self.bricks[i]
                .below
                .iter()
                .filter(|&&b| *self.bricks[b].z.end() + 1 == z_start)
                .cloned()
                .collect();

            let z_end = *self.bricks[i].z.end();
            self.bricks[i].supporting = self.bricks[i]
                .above
                .iter()
                .filter(|&&a| *self.bricks[a].z.start() == z_end + 1)
                .cloned()
                .collect();
        }
    }

    fn disintegratable(&self) -> impl Iterator<Item = &Brick> {
        let single_supports = self
            .bricks
            .iter()
            .filter(|b| b.supported_by.len() == 1)
            .map(|b| b.index)
            .collect_vec();
        self.bricks
            .iter()
            .filter(move |b| b.supporting.iter().all(|a| !single_supports.contains(a)))
    }

    fn falling(&self) -> usize {
        let mut falling: Vec<Falling> = Vec::new();
        falling.resize(self.bricks.len(), Empty);
        self.bricks
            .iter()
            .filter(|b| b.supporting.is_empty())
            .for_each(|b| falling[b.index] = Falling::initial(b.index));
        let mut queue: VecDeque<usize> = self
            .bricks
            .iter()
            .sorted_by_key(|b| b.z.start())
            .rev()
            .map(|b| b.index)
            .collect();
        while let Some(i) = queue.pop_front() {
            if falling[i].is_some() {
                continue;
            }
            let supporting = self.bricks[i]
                .supporting
                .iter()
                .map(|&s| &falling[s])
                .collect_vec();
            if supporting.iter().any(|f| f.is_none()) {
                queue.push_back(i);
                continue;
            }
            falling[i] = self.build_falling(&falling, i);
        }
        falling.iter().map(|f| f.len()).sum()
    }

    fn build_falling(&self, falling: &[Falling], current: usize) -> Falling {
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.extend(self.bricks[current].supporting.clone());
        let mut will_fall: Vec<bool> = Vec::new();
        will_fall.resize(self.bricks.len(), false);
        will_fall[current] = true;
        let mut complete = true;

        while let Some(current) = queue.pop_front() {
            if will_fall[current] {
                continue;
            }
            will_fall[current] = self.bricks[current]
                .supported_by
                .iter()
                .all(|&u| will_fall[u]);
            if !will_fall[current] {
                complete = false;
                continue;
            }
            match &falling[current] {
                Empty => panic!("should have been computed already"),
                Complete(s) => {
                    s.iter().for_each(|&t| will_fall[t] = true);
                }
                Incomplete(_) => {
                    queue.extend(self.bricks[current].supporting.clone());
                }
            };
        }

        let will_fall_set = (0..will_fall.len()).filter(|&i| will_fall[i]).collect();
        if complete {
            Complete(will_fall_set)
        } else {
            Incomplete(will_fall_set.len())
        }
    }
}

#[derive(Clone)]
enum Falling {
    Empty,
    Complete(HashSet<usize>),
    Incomplete(usize),
}

impl Falling {
    fn initial(index: usize) -> Self {
        let mut set = HashSet::new();
        set.insert(index);
        Complete(set)
    }

    fn is_none(&self) -> bool {
        matches!(self, Empty)
    }

    fn is_some(&self) -> bool {
        !matches!(self, Empty)
    }

    fn len(&self) -> usize {
        match self {
            Empty => panic!("shouldn't call len on empty falling"),
            Complete(s) => s.len() - 1,
            Incomplete(s) => s - 1,
        }
    }
}

impl Debug for Falling {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Empty => f.write_str("None"),
            Complete(s) => f.write_fmt(format_args!(
                "Complete({}) = {}",
                s.len(),
                s.iter().join(",")
            )),
            Incomplete(s) => f.write_fmt(format_args!("Incomplete({s})")),
        }
    }
}

#[derive(Clone)]
struct Brick {
    index: usize,
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: RangeInclusive<usize>,
    supporting: Vec<usize>,
    supported_by: Vec<usize>,
    below: Vec<usize>,
    above: Vec<usize>,
}

impl Brick {
    fn lower_to(&mut self, min: usize) {
        self.z = min..=(self.z.end() - self.z.start() + min);
    }
}

impl Debug for Brick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:3}= {},{},{}~{},{},{}  below = ({} -> {}) + above = ({} -> {})",
            self.index,
            self.x.start(),
            self.y.start(),
            self.z.start(),
            self.x.end(),
            self.y.end(),
            self.z.end(),
            self.below.iter().join(","),
            self.supported_by.iter().join(","),
            self.above.iter().join(","),
            self.supporting.iter().join(","),
        ))
    }
}

type BrickDef = (
    RangeInclusive<usize>,
    RangeInclusive<usize>,
    RangeInclusive<usize>,
);

fn overlaps(a: &BrickDef, b: &BrickDef) -> bool {
    a.0.start() <= b.0.end()
        && a.0.end() >= b.0.start()
        && a.1.start() <= b.1.end()
        && a.1.end() >= b.1.start()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[test]
    fn counts_disintegratable() {
        let solver = Advent2023Day22Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 5);
    }

    #[test]
    fn counts_falling_bricks() {
        let solver = Advent2023Day22Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2(), 7);
    }
}
