mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use crate::solver::AdventSolverBuilder;
use crate::year2021::day01::advent2021_day01_solver;
use crate::year2021::day02::advent2021_day02_solver;
use crate::year2021::day03::advent2021_day03_solver;
use crate::year2021::day04::advent2021_day04_solver;
use crate::year2021::day05::advent2021_day05_solver;
use crate::year2021::day06::advent2021_day06_solver;
use crate::year2021::day07::advent2021_day07_solver;
use crate::year2021::day08::advent2021_day08_solver;
use crate::year2021::day09::advent2021_day09_solver;
use crate::year2021::day10::advent2021_day10_solver;
use crate::year2021::day11::advent2021_day11_solver;
use crate::year2021::day12::advent2021_day12_solver;
use crate::year2021::day13::advent2021_day13_solver;
use crate::year2021::day14::advent2021_day14_solver;
use crate::year2021::day15::advent2021_day15_solver;
use crate::year2021::day16::advent2021_day16_solver;
use crate::year2021::day17::advent2021_day17_solver;
use crate::year2021::day18::advent2021_day18_solver;
use crate::year2021::day19::advent2021_day19_solver;
use crate::year2021::day20::advent2021_day20_solver;
use crate::year2021::day21::advent2021_day21_solver;
use crate::year2021::day22::advent2021_day22_solver;
use crate::year2021::day23::advent2021_day23_solver;
use crate::year2021::day24::advent2021_day24_solver;
use crate::year2021::day25::advent2021_day25_solver;

pub fn advent2021_solver_builders() -> Vec<AdventSolverBuilder> {
    vec!(
        advent2021_day01_solver,
        advent2021_day02_solver,
        advent2021_day03_solver,
        advent2021_day04_solver,
        advent2021_day05_solver,
        advent2021_day06_solver,
        advent2021_day07_solver,
        advent2021_day08_solver,
        advent2021_day09_solver,
        advent2021_day10_solver,
        advent2021_day11_solver,
        advent2021_day12_solver,
        advent2021_day13_solver,
        advent2021_day14_solver,
        advent2021_day15_solver,
        advent2021_day16_solver,
        advent2021_day17_solver,
        advent2021_day18_solver,
        advent2021_day19_solver,
        advent2021_day20_solver,
        advent2021_day21_solver,
        advent2021_day22_solver,
        advent2021_day23_solver,
        advent2021_day24_solver,
        advent2021_day25_solver,
    )
}
