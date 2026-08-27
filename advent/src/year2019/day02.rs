use crate::solver::AdventSolver;
use crate::year2019::intcode::{Computer, Value};

pub struct Advent2019Day02Solver {
    program: Vec<Value>,
}

impl Advent2019Day02Solver {
    pub fn new(input: &str) -> Self {
        Self {
            program: input.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }
}

impl AdventSolver for Advent2019Day02Solver {
    fn solve_part1(&self) -> usize {
        let mut program = self.program.clone();
        program[1] = 12;
        program[2] = 2;
        let mut computer = Computer::new(program);
        computer.run();
        computer.read_value(0) as usize
    }

    fn solve_part2(&self) -> usize {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut program = self.program.clone();
                program[1] = noun;
                program[2] = verb;
                let mut computer = Computer::new(program);
                computer.run();
                if computer.read_value(0) == 19690720 {
                    return 100usize * noun as usize + verb as usize;
                }
            }
        }
        unreachable!("no solution found")
    }
}