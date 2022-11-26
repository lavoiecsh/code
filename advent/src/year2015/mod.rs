mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

use crate::solver::AdventSolverBuilder;
use crate::year2015::day01::advent2015_day01_solver;
use crate::year2015::day02::advent2015_day02_solver;
use crate::year2015::day03::advent2015_day03_solver;
use crate::year2015::day04::advent2015_day04_solver;
use crate::year2015::day05::advent2015_day05_solver;
use crate::year2015::day06::advent2015_day06_solver;
use crate::year2015::day07::advent2015_day07_solver;
use crate::year2015::day08::advent2015_day08_solver;
use crate::year2015::day09::advent2015_day09_solver;

pub fn advent2015_solver_builders() -> Vec<AdventSolverBuilder> {
    vec!(
        advent2015_day01_solver,
        advent2015_day02_solver,
        advent2015_day03_solver,
        advent2015_day04_solver,
        advent2015_day05_solver,
        advent2015_day06_solver,
        advent2015_day07_solver,
        advent2015_day08_solver,
        advent2015_day09_solver,
    )
}
