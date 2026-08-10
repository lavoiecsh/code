use crate::solver::AdventSolver;
use std::collections::VecDeque;
use itertools::Itertools;

pub struct Advent2019Day07Solver {
    program: Vec<i32>,
}

impl Advent2019Day07Solver {
    pub fn new(input: &str) -> Self {
        Self {
            program: input.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }
}

impl AdventSolver for Advent2019Day07Solver {
    fn solve_part1(&self) -> usize {
        let mut max_signal = 0;
        for permutation in (0..=4).permutations(5) {
            let mut computers: Vec<Computer> = permutation.iter()
                .map(|phase| {
                    let mut computer = Computer::new(self.program.clone());
                    computer.send_input(*phase);
                    computer
                })
                .collect();

            computers[0].send_input(0);
            computers[0].run();
            for i in 1..=4 {
                let output = computers[i-1].receive_output();
                computers[i].send_input(output);
                computers[i].run();
            }

            let signal = computers[4].receive_output();
            if signal > max_signal {
                max_signal = signal;
            }
        }
        max_signal as usize
    }

    fn solve_part2(&self) -> usize {
        let mut max_signal = 0;
        for permutation in (5..=9).permutations(5) {
            let mut computers: Vec<Computer> = permutation.iter()
                .map(|phase| {
                    let mut computer = Computer::new(self.program.clone());
                    computer.send_input(*phase);
                    computer
                })
                .collect();

            computers[0].send_input(0);
            let mut index = 0;
            while !matches!(computers[4].state, ComputerState::Halted) {
                let next_index = (index + 1) % 5;
                computers[index].run();
                let output = computers[index].receive_output();
                computers[next_index].send_input(output);
                index = next_index;
            }

            let signal = computers[0].input.pop_front().unwrap();
            if signal > max_signal {
                max_signal = signal;
            }
        }
        max_signal as usize
    }
}

struct Computer {
    program: Vec<i32>,
    pointer: usize,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
    state: ComputerState,
}

enum ComputerState {
    Running,
    Waiting,
    Halted,
}

impl Computer {
    fn new(program: Vec<i32>) -> Self {
        Self {
            program,
            pointer: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: ComputerState::Running,
        }
    }

    fn send_input(&mut self, input: i32) {
        self.input.push_back(input);
    }

    fn read_input(&mut self) -> i32 {
        self.input.pop_front().unwrap()
    }

    fn receive_output(&mut self) -> i32 {
        self.output.pop_front().unwrap()
    }

    fn run(&mut self) {
        self.state = ComputerState::Running;
        while matches!(self.state, ComputerState::Running) {
            self.pointer += self.execute_operation();
        }
    }

    fn execute_operation(&mut self) -> usize {
        let opcode = self.program[self.pointer];
        let instruction = opcode % 100;
        match instruction {
            1 => {
                let a = self.get_value(1);
                let b = self.get_value(2);
                let position = self.program[self.pointer + 3] as usize;
                self.program[position] = a + b;
                4
            }
            2 => {
                let a = self.get_value(1);
                let b = self.get_value(2);
                let position = self.program[self.pointer + 3] as usize;
                self.program[position] = a * b;
                4
            }
            3 => {
                if self.input.is_empty() {
                    self.state = ComputerState::Waiting;
                    return 0;
                }
                let position = self.program[self.pointer + 1] as usize;
                let value = self.read_input();
                self.program[position] = value;
                2
            }
            4 => {
                let value = self.get_value(1);
                self.output.push_back(value);
                2
            }
            5 => {
                if self.get_value(1) != 0 {
                    self.pointer = self.get_value(2) as usize;
                    0
                } else {
                    3
                }
            }
            6 => {
                if self.get_value(1) == 0 {
                    self.pointer = self.get_value(2) as usize;
                    0
                } else {
                    3
                }
            }
            7 => {
                let position = self.program[self.pointer + 3] as usize;
                self.program[position] = if self.get_value(1) < self.get_value(2) { 1 } else { 0 };
                4
            }
            8 => {
                let position = self.program[self.pointer + 3] as usize;
                self.program[position] = if self.get_value(1) == self.get_value(2) { 1 } else { 0 };
                4
            }
            99 => {
                self.state = ComputerState::Halted;
                1
            }
            _ => unreachable!("unknown instruction {instruction}"),
        }
    }

    fn get_value(&self, offset: u32) -> i32 {
        let div = 10i32.pow(offset + 1);
        let mode = (self.program[self.pointer] / div) % 10;
        let value = self.program[self.pointer + offset as usize];
        match mode {
            0 => self.program[value as usize],
            1 => value,
            _ => unreachable!("unknown mode {mode}"),
        }
    }
}
