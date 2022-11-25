use crate::solver::{AdventSolverBuilder};
use crate::year2016::day01::advent2016_day01_solver;

mod day01;

pub fn advent2016_solver_builders() -> Vec<AdventSolverBuilder> {
    vec!(
        advent2016_day01_solver
    )
}
