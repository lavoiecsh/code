use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;
use std::io;
use std::io::stdin;
use std::path::MAIN_SEPARATOR;

use clap::Parser;

use crate::solver::AdventSolverBuilder;

pub enum AdventError {
    UnknownYear(u16),
    UnknownDay(u16, u8),
    InvalidInputOptions,
    FailedToReadStdin,
    FailedToReadFile(String),
}

impl Debug for AdventError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            AdventError::UnknownYear(year) => f.write_fmt(format_args!("Unknown year {year}")),
            AdventError::UnknownDay(year, day) => f.write_fmt(format_args!("Unknown day {day} within year {year}")),
            AdventError::InvalidInputOptions => f.write_fmt(format_args!("Input options are not valid, only one of input, file and stdin can be specified at a time")),
            AdventError::FailedToReadStdin => f.write_fmt(format_args!("Failed to read input from stdin")),
            AdventError::FailedToReadFile(file) => f.write_fmt(format_args!("Failed to read input from file {file}"))
        }
    }
}

#[derive(Parser)]
pub struct AdventOptions {
    #[arg(
        short = 'y',
        long,
        help("Year to solve, defaults to latest available year")
    )]
    year: Option<u16>,
    #[arg(
        short = 'd',
        long,
        help("Day to solve, defaults to latest solved problem within the year")
    )]
    day: Option<u8>,

    #[arg(
        short = '1',
        long,
        default_value_t = false,
        help("Only run part 1 of the problem")
    )]
    part1: bool,
    #[arg(
        short = '2',
        long,
        default_value_t = false,
        help("Only run part 2 of the problem")
    )]
    part2: bool,

    #[arg(
        short = 'f',
        long,
        help("Specify which input file to use, defaults to input matching year and day")
    )]
    file: Option<String>,
    #[arg(short = 'i', long, help("Use string input instead of reading a file"))]
    input: Option<String>,
    #[arg(
        short = 's',
        long,
        default_value_t = false,
        help("Read from standard input instead of reading a file")
    )]
    stdin: bool,
}

impl AdventOptions {
    pub fn part1(&self) -> bool {
        self.both_parts() || self.part1
    }

    pub fn part2(&self) -> bool {
        self.both_parts() || self.part2
    }

    fn both_parts(&self) -> bool {
        self.part1 == self.part2
    }

    pub fn read_input(&self, year: &str, day: &str) -> Result<String, AdventError> {
        match (&self.file, &self.input, &self.stdin) {
            (None, None, true) => read_stdin(),
            (None, None, false) => read_file(&format!(
                "input{MAIN_SEPARATOR}year{year}{MAIN_SEPARATOR}day{day}.txt"
            )),
            (Some(f), None, false) => read_file(f),
            (None, Some(i), false) => Ok(i.to_string()),
            _ => Err(AdventError::InvalidInputOptions),
        }
    }

    pub fn solver_builder(&self) -> Result<(AdventSolverBuilder, String, String), AdventError> {
        solver_builder(&self.year, &self.day)
    }
}

fn read_file(path: &str) -> Result<String, AdventError> {
    read_to_string(path)
        .map(trim)
        .map_err(|_| AdventError::FailedToReadFile(path.to_string()))
}

fn read_stdin() -> Result<String, AdventError> {
    stdin()
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()
        .map(|v| trim(v.join("\n")))
        .map_err(|_| AdventError::FailedToReadStdin)
}

fn trim(input: String) -> String {
    input.trim_end_matches('\n').to_string()
}

//noinspection RsLiveness
fn solver_builder(
    year: &Option<u16>,
    day: &Option<u8>,
) -> Result<(AdventSolverBuilder, String, String), AdventError> {
    include!(concat!(env!("OUT_DIR"), "/", "matches.txt"))
}
