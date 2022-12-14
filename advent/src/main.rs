use std::collections::HashMap;
use std::convert;
use std::process::ExitCode;
use std::time::SystemTime;

use clap::Parser;

use crate::solver::{AdventSolver, AdventSolverBuilder};
use crate::year2015::advent2015_solver_builders;
use crate::year2016::advent2016_solver_builders;
use crate::year2021::advent2021_solver_builders;
use crate::year2022::advent2022_solver_builders;

mod solver;
mod year2015;
mod year2016;
mod year2021;
mod year2022;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, help("Year to solve, defaults to latest available year"))]
    year: Option<usize>,
    #[arg(short, long, help("Day to solve, defaults to latest solved problem within the year"))]
    day: Option<usize>,

    #[arg(long, default_value_t = false, help("Skip part 1 of the problem"))]
    no_part1: bool,
    #[arg(long, default_value_t = false, help("Skip part 2 of the problem"))]
    no_part2: bool,
}

fn main() -> ExitCode {
    let cli: Cli = Cli::parse();

    build_solver(cli.year, cli.day).map_or_else(
        |e| {
            println!("Encountered error:\n{}", e);
            ExitCode::FAILURE
        },
        |s| {
            println!("Solving Year {}, Day {}", s.year(), s.day());
            if !cli.no_part1 {
                println!("\nSolving part 1");
                execute(|| s.solve_part1_string());
            }
            if !cli.no_part2 {
                println!("\nSolving part 2");
                execute(|| s.solve_part2_string());
            }
            ExitCode::SUCCESS
        })
}

fn build_solver(year: Option<usize>, day: Option<usize>) -> Result<Box<dyn AdventSolver>, String> {
    let solver_factories: HashMap<usize, Vec<AdventSolverBuilder>> = HashMap::from([
        (2015, advent2015_solver_builders()),
        (2016, advent2016_solver_builders()),
        (2021, advent2021_solver_builders()),
        (2022, advent2022_solver_builders()),
    ]);
    let latest_year = solver_factories.keys().max().unwrap();

    solver_factories.get(&year.unwrap_or(*latest_year))
        .ok_or(format!("No solver factory for year {}",
                       year.map_or("None".to_string(), |y| y.to_string())))
        .map(|f|
            day.map_or_else(|| f.last(), |d| f.get(d - 1))
                .map(|b| b())
                .ok_or(format!("No solver for year {} day {}",
                               year.map_or("None".to_string(), |y| y.to_string()),
                               day.map_or("None".to_string(), |d| d.to_string()))))
        .and_then(convert::identity)
}

fn execute(solver: impl Fn() -> String) {
    let now = SystemTime::now();
    let solution = solver();
    match now.elapsed() {
        Ok(d) => {
            println!("Duration: {}s {:0>3}.{:0>3}ms", d.as_secs(), d.subsec_millis(), d.subsec_micros() % 1000);
        }
        Err(e) => {
            println!("Duration errored: {:?}", e);
        }
    }
    println!("Solution:\n{}", solution);
}
