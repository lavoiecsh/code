use crate::solver::AdventSolver;
use regex::{Match, Regex};
use std::fmt::{Debug, Formatter};

pub struct Advent2024Day14Solver {
    bathroom: Bathroom,
}

impl Advent2024Day14Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let parse = |c: Option<Match>| c.unwrap().as_str().parse().unwrap();
        Self {
            bathroom: Bathroom::new(
                input
                    .lines()
                    .map(|l| re.captures(l).unwrap())
                    .map(|c| {
                        Robot::new(
                            (parse(c.get(1)), parse(c.get(2))),
                            (parse(c.get(3)), parse(c.get(4))),
                        )
                    })
                    .collect(),
            ),
        }
    }
}

impl AdventSolver for Advent2024Day14Solver {
    fn solve_part1(&self) -> usize {
        let mut bathroom = self.bathroom.clone();
        for _ in 0..100 {
            bathroom.iterate();
        }
        bathroom.safety_factor()
    }

    fn solve_part2(&self) -> usize {
        let mut bathroom = self.bathroom.clone();
        let mut i = 0;
        while !bathroom.is_tree_like() {
            bathroom.iterate();
            i += 1;
        }
        i
    }
}

#[derive(Clone)]
struct Bathroom {
    width: i32,
    height: i32,
    robots: Vec<Robot>,
}

impl Bathroom {
    fn new(robots: Vec<Robot>) -> Self {
        Self {
            width: 101,
            height: 103,
            robots,
        }
    }

    fn iterate(&mut self) {
        self.robots
            .iter_mut()
            .for_each(|r| r.iterate(self.width, self.height));
    }

    fn safety_factor(&self) -> usize {
        let half_width = self.width / 2;
        let half_height = self.height / 2;
        let mut quadrant_counts = [0; 4];
        self.robots.iter().for_each(|r| {
            if r.position.0 < half_width {
                if r.position.1 < half_height {
                    quadrant_counts[0] += 1;
                } else if r.position.1 > half_height {
                    quadrant_counts[1] += 1;
                }
            } else if r.position.0 > half_width {
                if r.position.1 < half_height {
                    quadrant_counts[2] += 1;
                } else if r.position.1 > half_height {
                    quadrant_counts[3] += 1;
                }
            }
        });
        quadrant_counts.iter().product()
    }

    fn is_tree_like(&self) -> bool {
        let mut rows = vec![0; self.width as usize];
        let mut cols = vec![0; self.height as usize];
        let threshold = self.width as usize / 3;
        self.robots
            .iter()
            .for_each(|r| {
                rows[r.position.0 as usize] += 1;
                cols[r.position.1 as usize] += 1;
            });
        rows.iter().any(|&r| r > threshold) &&
            cols.iter().any(|&r| r > threshold)
    }
}

impl Debug for Bathroom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut board = String::with_capacity(self.width as usize * (self.height as usize + 2));
        board.push('\n');
        for y in 0..self.height {
            for x in 0..self.width {
                let robot_present = self.robots.iter().any(|r| r.position == (x, y));
                board.push(if robot_present { '*' } else { ' ' });
            }
            board.push('\n');
        }
        f.write_str(&board)
    }
}

type Pos = (i32, i32);

#[derive(Clone)]
struct Robot {
    position: Pos,
    velocity: Pos,
}

impl Robot {
    fn new(position: Pos, velocity: Pos) -> Self {
        Self { position, velocity }
    }

    fn iterate(&mut self, width: i32, height: i32) {
        self.position.0 += self.velocity.0 + width;
        self.position.0 %= width;
        self.position.1 += self.velocity.1 + height;
        self.position.1 %= height;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn finds_safety_factor_after_100_seconds() {
        let mut solver = Advent2024Day14Solver::new(EXAMPLE);
        solver.bathroom.width = 11;
        solver.bathroom.height = 7;
        assert_eq!(solver.solve_part1(), 12);
    }
}
