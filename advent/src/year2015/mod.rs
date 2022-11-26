mod day01;
mod day02;
mod day03;

use crate::solver::AdventSolverBuilder;
use crate::year2015::day01::advent2015_day01_solver;
use crate::year2015::day02::advent2015_day02_solver;
use crate::year2015::day03::advent2015_day03_solver;

pub fn advent2015_solver_builders() -> Vec<AdventSolverBuilder> {
    vec!(
        advent2015_day01_solver,
        advent2015_day02_solver,
        advent2015_day03_solver,
    )
}
