use std::time::SystemTime;

use clap::Parser;

use options::AdventOptions;

use crate::options::AdventError;
use crate::solver::AdventSolver;

mod solver;
mod year2015;
mod year2016;
mod year2021;
mod year2022;
mod options;

pub type AdventSolverBuilder = fn(input: String) -> Box<dyn AdventSolver>;

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
    let options = AdventOptions::parse();

    let (solver_builder, year, day) = options.solver_builder()?;
    println!("Solving year {year} day {day}");

    println!("Reading input");
    time!(let input = options.read_input(&year, &day)?);

    println!("Building solver");
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
