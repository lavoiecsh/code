use std::env;
use std::process::Command;
use std::time::SystemTime;
use itertools::Itertools;
use crate::solvers::{get_solver, Solver};

mod solvers;
mod libs;

fn main() -> Result<(), String> {
    let problem: usize = env::args().nth(1)
        .or_else(get_modified_solver)
        .ok_or_else(|| "No argument passed and no modified solver in tree".to_string())?
        .parse::<usize>()
        .map_err(|e| e.to_string())?;

    println!("Solving problem {problem}");

    let solver: Solver = get_solver(problem)?;

    let now = SystemTime::now();
    println!("Solution: {}", solver());
    now.elapsed()
        .map(|d| println!("Time: {}s {:0>3}.{:0>3}ms", d.as_secs(), d.subsec_millis(), d.subsec_micros() % 1000))
        .map_err(|e| e.to_string())
}

fn get_modified_solver() -> Option<String> {
    Command::new(r"sh")
        .arg("-c")
        .arg(r"git status --porcelain | sed -n 's/.*\/p\([0-9]\{4\}\)\.rs/\1/p'")
        .output()
        .ok()
        .map(|o| o.stdout.iter().take(4).map(|&c| c as char).join(""))
}
