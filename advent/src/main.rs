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
        println!($p);
        let now = SystemTime::now();
        $s
        match now.elapsed() {
            Ok(d) => println!("Duration: {}s {:0>3}.{:0>3}ms", d.as_secs(), d.subsec_millis(), d.subsec_micros() % 1000),
            Err(_) => println!("Duration errored"),
        }
    }
}

fn main() -> Result<(), AdventError> {
    let options = AdventOptions::parse();

    let (solver_builder, year, day) = options.solver_builder()?;
    println!("Solving year {year} day {day}");

    time!("\nReading input", let input = options.read_input(&year, &day)?);

    time!("\nBuilding solver", let solver = solver_builder(&input));

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
