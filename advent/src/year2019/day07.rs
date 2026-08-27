use crate::solver::AdventSolver;
use crate::year2019::intcode::{Computer, Value};
use itertools::Itertools;

pub struct Advent2019Day07Solver {
    program: Vec<Value>,
}

impl Advent2019Day07Solver {
    pub fn new(input: &str) -> Self {
        Self {
            program: input.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }

    fn permutation_computers(&self, permutation: Vec<Value>) -> Vec<Computer> {
        permutation.iter()
            .map(|phase| {
                let mut computer = Computer::new(self.program.clone());
                computer.send_input(*phase);
                computer
            })
            .collect()
    }
}

impl AdventSolver for Advent2019Day07Solver {
    fn solve_part1(&self) -> usize {
        let mut max_signal = 0;
        for permutation in (0..=4).permutations(5) {
            let mut computers = self.permutation_computers(permutation);
            computers[0].send_input(0);
            computers[0].run();
            for i in 1..=4 {
                let output = computers[i-1].receive_output().unwrap();
                computers[i].send_input(output);
                computers[i].run();
            }

            let signal = computers[4].receive_output().unwrap();
            if signal > max_signal {
                max_signal = signal;
            }
        }
        max_signal as usize
    }

    fn solve_part2(&self) -> usize {
        let mut max_signal = 0;
        for permutation in (5..=9).permutations(5) {
            let mut computers = self.permutation_computers(permutation);
            computers[0].send_input(0);
            let mut index = 0;
            while computers[4].is_running() {
                let next_index = (index + 1) % 5;
                computers[index].run();
                let output = computers[index].receive_output().unwrap();
                computers[next_index].send_input(output);
                index = next_index;
            }

            let signal = computers[0].read_input();
            if signal > max_signal {
                max_signal = signal;
            }
        }
        max_signal as usize
    }
}
