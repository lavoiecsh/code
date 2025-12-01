use std::io::stdout;
use std::io::Write;
use std::time::SystemTime;

use clap::Parser;

use options::AdventOptions;

use crate::options::AdventError;

mod options;
mod solver;
mod year2015;
mod year2016;
mod year2017;
mod year2018;
mod year2021;
mod year2022;
mod year2023;
mod year2024;
mod year2025;

macro_rules! time {
    ($p: expr, $s: stmt) => {
        print!($p);
        stdout().flush().unwrap();
        let now = SystemTime::now();
        $s
        match now.elapsed() {
            Ok(d) => println!(" ({}s {:0>3}.{:0>3}ms)", d.as_secs(), d.subsec_millis(), d.subsec_micros() % 1000),
            Err(_) => println!(" (duration errored)"),
        }
    }
}

fn main() -> Result<(), AdventError> {
    let options = AdventOptions::parse();

    let (solver_builder, year, day) = options.solver_builder()?;
    println!("Solving year {year} day {day}\n");

    time!("Reading input", let input = options.read_input(&year, &day)?);

    time!("Building solver", let solver = solver_builder(&input));

    if options.part1() {
        time!("\nSolving part 1", let solution = solver.solve_part1_string());
        println!("Solution:\n{solution}");
    }

    if options.part2() {
        time!("\nSolving part 2", let solution = solver.solve_part2_string());
        println!("Solution:\n{solution}");
    }

    Ok(())
}
