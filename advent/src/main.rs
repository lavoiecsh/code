use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;
use std::time::SystemTime;

use clap::Parser;

use crate::solver::AdventSolver;
use crate::solver_builder::solver_builder;

mod solver;
mod solver_builder;
mod year2015;
mod year2016;
mod year2021;
mod year2022;

#[derive(Parser)]
struct Options {
    #[arg(short = 'y', long, help("Year to solve, defaults to latest available year"))]
    year: Option<u16>,
    #[arg(short = 'd', long, help("Day to solve, defaults to latest solved problem within the year"))]
    day: Option<u8>,

    #[arg(short = '1', long, default_value_t = false, help("Only run part 1 of the problem"))]
    part1: bool,
    #[arg(short = '2', long, default_value_t = false, help("Only run part 2 of the problem"))]
    part2: bool,

    #[arg(short = 'f', long, help("Specify which input file to use, defaults to input matching year and day"))]
    file: Option<String>,
    #[arg(short = 'i', long, help("Use string input instead of reading a file"))]
    input: Option<String>,
    #[arg(short = 's', long, default_value_t = false, help("Read from standard input instead of reading a file"))]
    stdin: bool,
}

impl Options {
    fn part1(&self) -> bool {
        self.both_parts() || self.part1
    }

    fn part2(&self) -> bool {
        self.both_parts() || self.part2
    }

    fn both_parts(&self) -> bool {
        self.part1 == self.part2
    }

    fn read_input(&self, year: &str, day: &str) -> Result<String, AdventError> {
        match (&self.file, &self.input, &self.stdin) {
            (None, None, true) => {
                let mut buffer = String::new();
                std::io::stdin().read_line(&mut buffer).map_err(|_| AdventError::InvalidInputOptions)?;
                Ok(buffer.trim().to_string())
            },
            (None, None, false) => read_to_string(format!("input/year{}/day{}.txt", year, day)).map(|i| i.trim().to_string()).map_err(|_| AdventError::InvalidInputOptions),
            (Some(f), None, false) => read_to_string(f).map(|i| i.trim().to_string()).map_err(|_| AdventError::InvalidInputOptions),
            (None, Some(i), false) => Ok(i.to_string()),
            _ => Err(AdventError::InvalidInputOptions),
        }
    }
}

pub type AdventSolverBuilder = fn(input: String) -> Box<dyn AdventSolver>;

pub enum AdventError {
    UnknownYear(u16),
    UnknownDay(u16, u8),
    InvalidInputOptions,
}

impl Debug for AdventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            AdventError::UnknownYear(y) => f.write_fmt(format_args!("Unknown year {}", y)),
            AdventError::UnknownDay(y, d) => f.write_fmt(format_args!("Unknown day {} within year {}", d, y)),
            AdventError::InvalidInputOptions => f.write_fmt(format_args!("Input options are not valid, only one of input, file and stdin can be specified at a time")),
        }
    }
}

macro_rules! time {
    ($s: stmt) => {
        let now = SystemTime::now();
        $s
        match now.elapsed() {
            Ok(d) => println!("Duration: {}s {:0>3}.{:0>3}ms", d.as_secs(), d.subsec_millis(), d.subsec_micros() % 1000),
            Err(_) => println!("Duration errored"),
        }
    }
}

fn main() -> Result<(), AdventError> {
    let options = Options::parse();

    let (solver_builder, year, day) = solver_builder(&options.year, &options.day)?;
    let input = options.read_input(&year, &day)?;

    println!("Building solver for year {} day {}", &year, &day);
    time!(let solver = solver_builder(input));

    if options.part1() {
        println!("Solving part 1");
        time!(let solution = solver.solve_part1_string());
        println!("Solution: {}", solution);
    }

    if options.part2() {
        println!("Solving part 2");
        time!(let solution = solver.solve_part2_string());
        println!("Solution: {}", solution);
    }

    Ok(())
}
