use crate::solver::AdventSolverBuilder;
use crate::year2022::day01::{Advent2022Day01Solver};

mod day01;

pub fn advent2022_solver_builders() -> Vec<AdventSolverBuilder> {
    vec!(
        || Box::new(Advent2022Day01Solver::new()),
    )
}
