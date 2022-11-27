use std::fs::read_to_string;
use regex::{Captures, Regex};
use crate::solver::AdventSolver;

pub struct Advent2015Day06Solver {
    commands: Vec<Command>,
}

type Pos = (usize, usize);

struct Command {
    action: String,
    from: Pos,
    to: Pos,
}

impl AdventSolver for Advent2015Day06Solver {
    fn day(&self) -> usize { 06 }
    fn year(&self) -> usize { 2015 }

    fn solve_part1(&self) -> usize {
        let mut lights: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
        for command in &self.commands {
            let a = match command.action.as_str() {
                "turn on" => |_| true,
                "turn off" => |_| false,
                "toggle" => |c: bool| !c,
                _ => panic!("unknown action"),
            };
            for i in command.from.0..(command.to.0+1) {
                for j in command.from.1..(command.to.1+1) {
                    lights[i][j] = a(lights[i][j]);
                }
            }
        }
        let mut count: usize = 0;
        for i in 0..1000 {
            for j in 0..1000 {
                if lights[i][j] {
                    count += 1;
                }
            }
        }
        count
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
            for i in command.from.0..(command.to.0+1) {
                for j in command.from.1..(command.to.1+1) {
                    lights[i][j] = a(lights[i][j]);
                }
            }
        }
        let mut brightness: usize = 0;
        for i in 0..1000 {
            for j in 0..1000 {
                brightness += lights[i][j];
            }
        }
        brightness
    }
}

pub fn advent2015_day06_solver() -> Box<dyn AdventSolver> {
    let re = Regex::new(r"(toggle|turn off|turn on) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let cap = |m: &Captures, i: usize| String::from(m.get(i).unwrap().as_str());
    Box::new(Advent2015Day06Solver {
        commands: read_to_string("src/year2016/day06.txt")
            .unwrap()
            .trim()
            .lines()
            .map(|l: &str| -> Command {
                let m = re.captures(l).unwrap();
                Command {
                    action: cap(&m, 1),
                    from: (cap(&m, 2).parse().unwrap(), cap(&m, 3).parse().unwrap()),
                    to: (cap(&m, 4).parse().unwrap(), cap(&m, 5).parse().unwrap()),
                }
            })
            .collect()
    })
}
