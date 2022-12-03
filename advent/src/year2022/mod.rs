use crate::solver::AdventSolverBuilder;
use crate::year2022::day01::{Advent2022Day01Solver};
use crate::year2022::day02::Advent2022Day02Solver;
use crate::year2022::day03::Advent2022Day03Solver;

mod day01;
mod day02;
mod day03;

pub fn advent2022_solver_builders() -> Vec<AdventSolverBuilder> {
    vec!(
        || Box::new(Advent2022Day01Solver::new()),
        || Box::new(Advent2022Day02Solver::new()),
        || Box::new(Advent2022Day03Solver::new()),
    )
}
