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
struct Cli {
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

pub type AdventSolverBuilder = fn(input: String) -> Box<dyn AdventSolver>;

#[derive(Debug)]
pub enum AdventError {
    UnknownYear(u16),
    UnknownDay(u16, u8),
    InvalidPartOptions,
    InvalidInputOptions,
}

fn main() -> Result<(), AdventError> {
    let cli = Cli::parse();

    let (solver_builder, year, day) = solver_builder(&cli.year, &cli.day)?;
    let input = read_input(&cli, &year, &day)?;

    println!("Building solver");
    let solver = build_solver(solver_builder, input)?;

    let (part1, part2) = read_parts(&cli)?;

    if part1 {
        println!("Solving part 1");
        let (solution, duration) = time(|| solver.solve_part1_string());
        println!("Solution: {}", solution);
        println!("Duration: {}", duration);
    }

    if part2 {
        println!("Solving part 2");
        let (solution, duration) = time(|| solver.solve_part2_string());
        println!("Solution: {}", solution);
        println!("Duration: {}", duration);
    }

    Ok(())
}

fn read_input(cli: &Cli, year: &str, day: &str) -> Result<String, AdventError> {
    match (&cli.file, &cli.input, &cli.stdin) {
        (None, None, true) => todo!(), // read from stdin,
        (None, None, false) => read_to_string(format!("input/year{}/day{}.txt", year, day)).map_err(|_| AdventError::InvalidInputOptions),
        (Some(f), None, false) => todo!(), // read from given file
        (None, Some(i), false) => Ok(i.to_string()),
        _ => Err(AdventError::InvalidInputOptions),
    }
}

fn read_parts(cli: &Cli) -> Result<(bool, bool), AdventError> {
    match (cli.part1, cli.part2) {
        (false, false) => Ok((true, true)),
        (false, true) => Ok((false, true)),
        (true, false) => Ok((true, false)),
        (true, true) => Err(AdventError::InvalidPartOptions)
    }
}

fn build_solver(builder: AdventSolverBuilder, input: String) -> Result<Box<dyn AdventSolver>, AdventError> {
    Ok(builder(input))
}

fn time<T>(func: impl Fn() -> T) -> (T, String) {
    let now = SystemTime::now();
    let result = func();
    match now.elapsed() {
        Ok(d) => (result, format!("{}s {:0>3}.{:0>3}ms", d.as_secs(), d.subsec_millis(), d.subsec_micros() % 1000)),
        Err(e) => (result, format!("{:?}", e))
    }
}
