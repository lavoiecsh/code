use std::cmp::Ordering;
use std::collections::HashSet;

use crate::solver::AdventSolver;

type Pos = (isize, isize);

struct Motion {
    direction: char,
    distance: usize,
}

pub struct Advent2022Day09Solver {
    motions: Vec<Motion>,
}

impl Advent2022Day09Solver {
    pub fn new(input: &str) -> Self {
        Self {
            motions: input
                .lines()
                .map(|l| {
                    let mut s = l.split(" ");
                    Motion {
                        direction: s.next().unwrap().chars().next().unwrap(),
                        distance: s.next().unwrap().parse().unwrap(),
                    }
                })
                .collect(),
        }
    }

    fn move_rope(&self, knot_count: usize) -> usize {
        let mut rope = Rope::new(knot_count);
        let mut visited: HashSet<Pos> = HashSet::new();
        self.motions.iter().for_each(|m| {
            rope.execute(m).iter().for_each(|v| {
                visited.insert(*v);
            });
        });
        visited.len()
    }
}

impl AdventSolver for Advent2022Day09Solver {
    fn solve_part1(&self) -> usize {
        self.move_rope(2)
    }

    fn solve_part2(&self) -> usize {
        self.move_rope(10)
    }
}

struct Rope {
    knots: Vec<Pos>,
}

impl Rope {
    fn new(knot_count: usize) -> Self {
        let mut knots = Vec::new();
        (0..knot_count).for_each(|_| knots.push((0, 0)));
        Self { knots }
    }

    fn execute(&mut self, motion: &Motion) -> HashSet<Pos> {
        let mut visited = HashSet::new();
        for _ in 0..motion.distance {
            self.move_head(motion.direction);
            (1..self.knots.len()).for_each(|i| self.catch_up(i));
            visited.insert(*self.knots.last().unwrap());
        }
        visited
    }

    fn move_head(&mut self, direction: char) {
        match direction {
            'U' => self.knots[0].0 -= 1,
            'D' => self.knots[0].0 += 1,
            'L' => self.knots[0].1 -= 1,
            'R' => self.knots[0].1 += 1,
            _ => panic!("unknown direction {0}", direction),
        }
    }

    fn catch_up(&mut self, index: usize) {
        let diff0 = self.knots[index].0 - self.knots[index - 1].0;
        let diff1 = self.knots[index].1 - self.knots[index - 1].1;

        if diff0 > -2 && diff0 < 2 && diff1 > -2 && diff1 < 2 {
            return;
        }

        self.knots[index].0 += match diff0.cmp(&0) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };
        self.knots[index].1 += match diff1.cmp(&0) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0,
        };
    }
}
