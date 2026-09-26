use crate::solver::AdventSolver;
use crate::year2019::intcode::{parse_program, Computer, Value};

pub struct Advent2019Day09Solver {
    program: Vec<Value>,
}

impl Advent2019Day09Solver {
    pub fn new(input: &str) -> Self {
        Self {
            program: parse_program(input),
        }
    }
}

impl AdventSolver for Advent2019Day09Solver {
    fn solve_part1(&self) -> usize {
        let mut computer = Computer::new(self.program.clone());
        computer.send_input(1);
        computer.run();
        computer.receive_output().unwrap() as usize
    }

    fn solve_part2(&self) -> usize {
        let mut computer = Computer::new(self.program.clone());
        computer.send_input(2);
        computer.run();
        computer.receive_output().unwrap() as usize
    }
}
