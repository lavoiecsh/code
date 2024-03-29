use std::env;
use std::time::SystemTime;
use crate::solvers::{get_solver, Solver};

mod solvers;

fn main() -> Result<(), String> {
    let problem: usize = env::args()
        .skip(1)
        .next()
        .ok_or_else(|| "No argument passed".to_string())?
        .parse::<usize>().map_err(|e| e.to_string())?;

    println!("Solving problem {problem}");

    let solver: Solver = get_solver(problem)?;

    let now = SystemTime::now();
    println!("Solution: {}", solver());
    now.elapsed()
        .map(|d| println!("Time: {}s {:0>3}.{:0>3}ms", d.as_secs(), d.subsec_millis(), d.subsec_micros() % 1000))
        .map_err(|e| e.to_string())
}
