use std::fs;
use regex::Regex;
use crate::problem_solver::ProblemSolver;

pub struct Problem06Solver {
    commands: Vec<Command>
}

impl Problem06Solver {
    pub fn new() -> Self {
        let re = Regex::new(r"(toggle|turn off|turn on) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        Self {
            commands: fs::read_to_string("inputs/day06.txt")
                .expect("error reading")
                .trim()
                .lines()
                .map(|l: &str| -> Command {
                    let tm = re.captures(l);
                    let m = tm.unwrap();
                    Command {
                        action: String::from(m.get(1).unwrap().as_str()),
                        from: (m.get(2).unwrap().as_str().parse().unwrap(), m.get(3).unwrap().as_str().parse().unwrap()),
                        to: (m.get(4).unwrap().as_str().parse().unwrap(), m.get(5).unwrap().as_str().parse().unwrap()),
                    }
                })
                .collect()
        }
    }
}

impl ProblemSolver for Problem06Solver {
    fn solve_part1(&self) -> usize {
        let mut lights: [[bool; 1000]; 1000] = [[false; 1000]; 1000];
        self.commands
            .iter()
            .for_each(|c| {
                let a = if c.action == "turn on" { |_current: bool| true } else if c.action == "turn off" { |_current: bool| false } else { |current: bool| !current };
                for i in c.from.0..(c.to.0 + 1) {
                    for j in c.from.1..(c.to.1 + 1) {
                        lights[i][j] = a(lights[i][j]);
                    }
                }
            });
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
        self.commands
            .iter()
            .for_each(|c| {
                let a = if c.action == "turn on" { |current: usize| current + 1 } else if c.action == "turn off" { |current: usize| if current == 0 { 0 } else { current - 1 } } else { |current: usize| current + 2 };
                for i in c.from.0..(c.to.0 + 1) {
                    for j in c.from.1..(c.to.1 + 1) {
                        lights[i][j] = a(lights[i][j]);
                    }
                }
            });
        let mut brightness: usize = 0;
        for i in 0..1000 {
            for j in 0..1000 {
                brightness += lights[i][j];
            }
        }
        brightness
    }
}


type Pos = (usize, usize);

#[derive(Debug)]
struct Command {
    action: String,
    from: Pos,
    to: Pos,
}
