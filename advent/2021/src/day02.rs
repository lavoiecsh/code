#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

const FILENAME: &str = "inputs/day02.txt";

struct Command {
    direction: char,
    units: usize,
}

#[allow(dead_code)]
fn read_input() -> Vec<Command> {
    fs::read_to_string(FILENAME).expect("error reading")
        .trim()
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(input: &str) -> Command {
    let mut sections = input.split(" ");
    let dir = sections.next().unwrap().chars().next().unwrap();
    let units = sections.next().unwrap().parse().expect("error parsing");
    Command { direction: dir, units: units }
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let commands = read_input();
    let mut pos: usize = 0;
    let mut depth: usize = 0;
    for command in commands {
        match command.direction {
            'f' => pos += command.units,
            'd' => depth += command.units,
            'u' => depth -= command.units,
            _ => panic!("unknown direction")
        }
    }
    pos * depth
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let commands = read_input();
    let mut pos: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;
    for command in commands {
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
