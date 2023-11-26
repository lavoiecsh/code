use itertools::Itertools;
use num_traits::abs;
use regex::{Match, Regex};

use crate::solver::AdventSolver;

pub struct Advent2018Day10Solver {
    message: Message,
}

impl Advent2018Day10Solver {
    pub fn new(input: String) -> Self {
        let re = Regex::new(r"position=<([ -]\d+), ([ -]\d+)> velocity=<([ -]\d+), ([ -]\d+)>").unwrap();
        let parse = |c: Option<Match>| c.unwrap().as_str().trim().parse().unwrap();
        let mut message = Message {
            points: input.lines()
                .filter_map(|l| re.captures(l))
                .map(|c| Point {
                    position: Vec2 { x: parse(c.get(1)), y: parse(c.get(2)) },
                    velocity: Vec2 { x: parse(c.get(3)), y: parse(c.get(4)) },
                })
                .collect(),
            iterations: 0,
        };
        message.find_message();
        Self { message }
    }
}

impl AdventSolver for Advent2018Day10Solver {
    fn solve_part1_string(&self) -> String {
        self.message.print()
    }

    fn solve_part2(&self) -> usize {
        self.message.iterations
    }
}

struct Message {
    points: Vec<Point>,
    iterations: usize,
}

impl Message {
    fn find_message(&mut self) {
        let mut prev_distance = usize::MAX;
        while self.distance() < prev_distance {
            prev_distance = self.distance();
            self.iterate();
        }
        self.revert();
    }

    fn iterate(&mut self) {
        self.points.iter_mut().for_each(|p| p.move_pos());
        self.iterations += 1;
    }

    fn revert(&mut self) {
        self.points.iter_mut().for_each(|p| p.revert());
        self.iterations -= 1;
    }

    fn distance(&self) -> usize {
        let (min_x, max_x) = self.points.iter().map(|p| p.position.x).minmax().into_option().unwrap();
        let (min_y, max_y) = self.points.iter().map(|p| p.position.y).minmax().into_option().unwrap();
        abs(max_x - min_x) as usize + abs(max_y - min_y) as usize
    }

    fn print(&self) -> String {
        let (min_x, max_x) = self.points.iter().map(|p| p.position.x).minmax().into_option().unwrap();
        let (min_y, max_y) = self.points.iter().map(|p| p.position.y).minmax().into_option().unwrap();
        let mut output: Vec<String> = Vec::new();
        for y in min_y..=max_y {
            let mut row: Vec<char> = Vec::new();
            for x in min_x..=max_x {
                row.push(if self.points.iter()
                    .any(|p| p.position.x == x && p.position.y == y) {
                    '#'
                } else { ' ' });
            }
            output.push(row.iter().join(""));
        }
        output.iter().join("\n")
    }
}

struct Point {
    position: Vec2,
    velocity: Vec2,
}

impl Point {
    fn move_pos(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn revert(&mut self) {
        self.position.x -= self.velocity.x;
        self.position.y -= self.velocity.y;
    }
}

struct Vec2 {
    x: i64,
    y: i64,
}
