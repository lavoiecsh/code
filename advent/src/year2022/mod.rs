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
use crate::year2022::day13::Advent2022Day13Solver;
use crate::year2022::day14::Advent2022Day14Solver;
use crate::year2022::day15::Advent2022Day15Solver;
use crate::year2022::day16::Advent2022Day16Solver;
use crate::year2022::day17::Advent2022Day17Solver;
use crate::year2022::day18::Advent2022Day18Solver;
use crate::year2022::day19::Advent2022Day19Solver;
use crate::year2022::day20::Advent2022Day20Solver;
use crate::year2022::day21::Advent2022Day21Solver;

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
        || Box::new(Advent2022Day13Solver::new()),
        || Box::new(Advent2022Day14Solver::new()),
        || Box::new(Advent2022Day15Solver::new()),
        || Box::new(Advent2022Day16Solver::new()),
        || Box::new(Advent2022Day17Solver::new()),
        || Box::new(Advent2022Day18Solver::new()),
        || Box::new(Advent2022Day19Solver::new()),
        || Box::new(Advent2022Day20Solver::new()),
        || Box::new(Advent2022Day21Solver::new()),
    )
}
