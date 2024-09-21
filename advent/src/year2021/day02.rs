use crate::solver::AdventSolver;

struct Command { direction: char, units: usize }
pub struct Advent2021Day02Solver {
    commands: Vec<Command>
}

impl Advent2021Day02Solver {
    pub fn new(input: String) -> Self {
        Self {
            commands: input
                .trim()
                .lines()
                .map(parse_line)
                .collect()
        }
    }
}

impl AdventSolver for Advent2021Day02Solver {
    fn solve_part1(&self) -> usize {
        let mut pos: usize = 0;
        let mut depth: usize = 0;
        for command in &self.commands {
            match command.direction {
                'f' => pos += command.units,
                'd' => depth += command.units,
                'u' => depth -= command.units,
                _ => panic!("unknown direction")
            }
        }
        pos * depth
    }

    fn solve_part2(&self) -> usize {
        let mut pos: usize = 0;
        let mut depth: usize = 0;
        let mut aim: usize = 0;
        for command in &self.commands {
            match command.direction {
                'd' => aim += command.units,
                'u' => aim -= command.units,
                'f' => {
                    pos += command.units;
                    depth += aim * command.units;
                }
                _ => panic!("unknown direction")
            }
        }
        pos * depth
    }
}

fn parse_line(input: &str) -> Command {
    let mut sections = input.split(" ");
    let dir = sections.next().unwrap().chars().next().unwrap();
    let units = sections.next().unwrap().parse().expect("error parsing");
    Command { direction: dir, units }
}
