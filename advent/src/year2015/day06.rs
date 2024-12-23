use regex::{Captures, Regex};

use crate::solver::AdventSolver;

pub struct Advent2015Day06Solver {
    commands: Vec<Command>,
}

impl Advent2015Day06Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"(toggle|turn off|turn on) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        let cap = |m: &Captures, i: usize| String::from(m.get(i).unwrap().as_str());
        Self {
            commands: input
                .lines()
                .map(|l: &str| -> Command {
                    let m = re.captures(l).unwrap();
                    Command {
                        action: cap(&m, 1),
                        from: (cap(&m, 2).parse().unwrap(), cap(&m, 3).parse().unwrap()),
                        to: (cap(&m, 4).parse().unwrap(), cap(&m, 5).parse().unwrap()),
                    }
                })
                .collect(),
        }
    }
}

type Pos = (usize, usize);

struct Command {
    action: String,
    from: Pos,
    to: Pos,
}

impl AdventSolver for Advent2015Day06Solver {
    fn solve_part1(&self) -> usize {
        let mut lights: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
        for command in &self.commands {
            let a = match command.action.as_str() {
                "turn on" => |_| true,
                "turn off" => |_| false,
                "toggle" => |c: bool| !c,
                _ => panic!("unknown action"),
            };
            for i in command.from.0..(command.to.0 + 1) {
                for j in command.from.1..(command.to.1 + 1) {
                    lights[i][j] = a(lights[i][j]);
                }
            }
        }
        lights
            .iter()
            .map(|row| row.iter().filter(|x| **x).count())
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut lights: [[usize; 1000]; 1000] = [[0; 1000]; 1000];
        for command in &self.commands {
            let a = match command.action.as_str() {
                "turn on" => |c: usize| c + 1,
                "turn off" => |c: usize| if c == 0 { 0 } else { c - 1 },
                "toggle" => |c: usize| c + 2,
                _ => panic!("unknown command"),
            };
            for i in command.from.0..(command.to.0 + 1) {
                for j in command.from.1..(command.to.1 + 1) {
                    lights[i][j] = a(lights[i][j]);
                }
            }
        }
        lights.iter().map(|row| row.iter().sum::<usize>()).sum()
    }
}
