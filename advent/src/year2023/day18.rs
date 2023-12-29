use std::fmt::Debug;

use regex::Regex;

use Direction::*;

use crate::solver::AdventSolver;

pub struct Advent2023Day18Solver {
    dig_plan: Vec<TrenchPlan>,
}

impl Advent2023Day18Solver {
    pub fn new(input: String) -> Self {
        let re = Regex::new(r"(R|L|U|D) (\d+) \(#([0-9a-f]{6})\)").unwrap();
        Self {
            dig_plan: input.lines()
                .filter_map(|l| re.captures(l))
                .map(|c| TrenchPlan {
                    direction: Direction::from(c.get(1).unwrap().as_str()),
                    distance: c.get(2).unwrap().as_str().parse().unwrap(),
                    color: c.get(3).unwrap().as_str().to_string(),
                })
                .collect()
        }
    }
}

impl AdventSolver for Advent2023Day18Solver {
    fn solve_part1(&self) -> usize {
        dig(self.dig_plan.iter().map(|tp| tp.simple()).collect()) as usize
    }

    fn solve_part2(&self) -> usize {
        dig(self.dig_plan.iter().map(|tp| tp.colored()).collect()) as usize
    }
}

fn dig(dig_plan: Vec<(i64, Direction)>) -> i64 {
    let mut corners = vec!();
    let mut x = 0;
    let mut y = 0;
    let len = dig_plan.len();
    for i in 0..len {
        let distance = dig_plan[i].0;
        let prev = dig_plan[(i + len - 1) % len].1;
        let curr = dig_plan[i].1;
        let next = dig_plan[(i + 1) % len].1;
        match (prev, curr, next) {
            (Up, Right, Down) => x += distance + 1,
            (Right, Down, Left) => y += distance + 1,
            (Down, Left, Down) => x -= distance,
            (Left, Down, Right) => y += distance - 1,
            (Down, Right, Down) => x += distance,
            (Down, Left, Up) => x -= distance + 1,
            (Left, Up, Left) => y -= distance,
            (Up, Left, Up) => x -= distance,
            (Left, Up, Right) => y -= distance + 1,
            (Up, Right, Up) => x += distance,
            (Right, Up, Left) => y -= distance - 1,
            (Right, Down, Right) => y += distance,
            (Up, Left, Down) => x -= distance - 1,
            (Left, Down, Left) => y += distance,
            (Right, Up, Right) => y -= distance,
            (Down, Right, Up) => x += distance - 1,
            _ => panic!("Unknown pathing {prev:?} -> {curr:?} -> {next:?}"),
        }
        corners.push((x, y));
    }

    let mut area = 0;
    for i in 0..len {
        let (x1, y1) = corners[(i + len - 1) % len];
        let (x2, y2) = corners[i];
        area += x1*y2 - x2*y1;
    }
    area / 2
}

struct TrenchPlan {
    direction: Direction,
    distance: i64,
    color: String,
}

impl TrenchPlan {
    fn simple(&self) -> (i64, Direction) {
        (self.distance, self.direction)
    }

    fn colored(&self) -> (i64, Direction) {
        (i64::from_str_radix(&self.color[..5], 16).unwrap(), match &self.color[5..6] {
            "0" => Right,
            "1" => Down,
            "2" => Left,
            "3" => Up,
            _ => panic!("unknown direction {}", self.color),
        })
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!("unknown direction {value}"),
        }
    }
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day18Solver {
    Advent2023Day18Solver::new(String::from("\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"))
}

#[test]
fn cubic_meters_dug() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part1(), 62);
}

#[test]
fn colored_cubic_meters_dug() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part2(), 952408144115);
}
