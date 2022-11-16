#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(while_true)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::time::SystemTime;

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

use day22::{part1, part2};

fn main() {
    println!("Execution Starting");
    execute(part1);
    execute(part2);
    println!("Execution Completed");
}

fn execute(part: fn() -> usize) {
    let now = SystemTime::now();
    let solution = part();
    let elapsed_result = now.elapsed();
    println!("Solution: {}", solution);
    match elapsed_result {
        Ok(elapsed) => {
            println!("Duration: {}s {:0>3}_{:0>3}_{:0>3}ns",
                     elapsed.as_secs(),
                     elapsed.subsec_millis(),
                     elapsed.subsec_micros() % 1000,
                     elapsed.subsec_nanos() % 1000);
        }
        Err(error) => {
            println!("Duration errored: {:?}", error);
        }
    }
}
