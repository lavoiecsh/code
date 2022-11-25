mod day01;

use crate::solver::AdventSolverBuilder;
use crate::year2015::day01::advent2015_day01_solver;

pub fn advent2015_solver_builders() -> Vec<AdventSolverBuilder> {
    vec!(
        advent2015_day01_solver
    )
}
