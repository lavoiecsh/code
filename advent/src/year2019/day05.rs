use crate::solver::AdventSolver;
use crate::year2019::intcode::{Computer, Value};

pub struct Advent2019Day05Solver {
    program: Vec<Value>,
}

impl Advent2019Day05Solver {
    pub fn new(input: &str) -> Self {
        Self {
            program: input.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }
}

impl AdventSolver for Advent2019Day05Solver {
    fn solve_part1(&self) -> usize {
        let mut computer = Computer::new(self.program.clone());
        computer.send_input(1);
        computer.run();
        while let Some(output) = computer.receive_output() {
            if output == 0 {
                continue;
            }
            return output as usize;
        }
        unreachable!("no valid solution found in the output")
    }

    fn solve_part2(&self) -> usize {
        let mut computer = Computer::new(self.program.clone());
        computer.send_input(5);
        computer.run();
        computer.receive_output().unwrap() as usize
    }
}
