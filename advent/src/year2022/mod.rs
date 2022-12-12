#![allow(dead_code)]

use crate::solver::AdventSolverBuilder;
use crate::year2022::day01::Advent2022Day01Solver;
use crate::year2022::day02::Advent2022Day02Solver;
use crate::year2022::day03::Advent2022Day03Solver;
use crate::year2022::day04::Advent2022Day04Solver;
use crate::year2022::day05::Advent2022Day05Solver;
use crate::year2022::day06::Advent2022Day06Solver;
use crate::year2022::day07::Advent2022Day07Solver;
use crate::year2022::day08::Advent2022Day08Solver;
use crate::year2022::day09::Advent2022Day09Solver;
use crate::year2022::day10::Advent2022Day10Solver;
use crate::year2022::day11::Advent2022Day11Solver;
use crate::year2022::day12::Advent2022Day12Solver;

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

pub fn advent2022_solver_builders() -> Vec<AdventSolverBuilder> {
    vec!(
        || Box::new(Advent2022Day01Solver::new()),
        || Box::new(Advent2022Day02Solver::new()),
        || Box::new(Advent2022Day03Solver::new()),
        || Box::new(Advent2022Day04Solver::new()),
        || Box::new(Advent2022Day05Solver::new()),
        || Box::new(Advent2022Day06Solver::new()),
        || Box::new(Advent2022Day07Solver::new()),
        || Box::new(Advent2022Day08Solver::new()),
        || Box::new(Advent2022Day09Solver::new()),
        || Box::new(Advent2022Day10Solver::new()),
        || Box::new(Advent2022Day11Solver::new()),
        || Box::new(Advent2022Day12Solver::new()),
    )
}
