#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(while_true)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::borrow::Borrow;
use std::collections::HashMap;
use std::time::SystemTime;
use crate::day01::Problem01Solver;
use crate::day02::Problem02Solver;
use crate::day03::Problem03Solver;
use crate::day04::Problem04Solver;
use crate::day05::Problem05Solver;
use crate::day06::Problem06Solver;
use crate::day07::Problem07Solver;
use crate::day08::Problem08Solver;
use crate::day09::Problem09Solver;
use crate::day10::Problem10Solver;
use crate::day11::Problem11Solver;
use crate::day12::Problem12Solver;
use crate::day13::Problem13Solver;
use crate::day14::Problem14Solver;
use crate::day15::Problem15Solver;
use crate::day16::Problem16Solver;
use crate::day17::Problem17Solver;
use crate::day18::Problem18Solver;
use crate::day19::Problem19Solver;
use crate::day20::Problem20Solver;
use crate::day21::Problem21Solver;
use crate::day22::Problem22Solver;
use crate::day23::Problem23Solver;
use crate::day24::Problem24Solver;
use crate::day25::Problem25Solver;
use crate::problem_solver::ProblemSolver;

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
mod problem_solver;

fn main() {
    let mut solver = get_solver(25);
    println!("Execution Starting");
    execute_part(solver.borrow(), 1);
    execute_part(solver.borrow(), 2);
    println!("Execution Completed");
}

pub fn get_solver(day: usize) -> Box<dyn ProblemSolver> {
    match day {
        01 => Box::new(Problem01Solver::new()),
        02 => Box::new(Problem02Solver::new()),
        03 => Box::new(Problem03Solver::new()),
        04 => Box::new(Problem04Solver::new()),
        05 => Box::new(Problem05Solver::new()),
        06 => Box::new(Problem06Solver::new()),
        07 => Box::new(Problem07Solver::new()),
        08 => Box::new(Problem08Solver::new()),
        09 => Box::new(Problem09Solver::new()),
        10 => Box::new(Problem10Solver::new()),
        11 => Box::new(Problem11Solver::new()),
        12 => Box::new(Problem12Solver::new()),
        13 => Box::new(Problem13Solver::new()),
        14 => Box::new(Problem14Solver::new()),
        15 => Box::new(Problem15Solver::new()),
        16 => Box::new(Problem16Solver::new()),
        17 => Box::new(Problem17Solver::new()),
        18 => Box::new(Problem18Solver::new()),
        19 => Box::new(Problem19Solver::new()),
        20 => Box::new(Problem20Solver::new()),
        21 => Box::new(Problem21Solver::new()),
        22 => Box::new(Problem22Solver::new()),
        23 => Box::new(Problem23Solver::new()),
        24 => Box::new(Problem24Solver::new()),
        25 => Box::new(Problem25Solver::new()),
        other => panic!("problem {} not found", day),
    }
}

fn execute_part(solver: &dyn ProblemSolver, part: usize) {
    let now = SystemTime::now();
    let solution = if part == 1 { solver.solve_part1() } else { solver.solve_part2() };
    let elapsed_result = now.elapsed();
    println!("Solution: {}", solution);
    match elapsed_result {
        Ok(elapsed) => {
            println!("Duration: {}s {:0>3}_{:0>3}us",
                     elapsed.as_secs(),
                     elapsed.subsec_millis(),
                     elapsed.subsec_micros() % 1000);
        }
        Err(error) => {
            println!("Duration errored: {:?}", error);
        }
    }
}
