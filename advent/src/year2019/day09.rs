use crate::solver::AdventSolver;
use std::collections::VecDeque;

type Value = i64;
pub struct Advent2019Day09Solver {
    program: Vec<Value>,
}

impl Advent2019Day09Solver {
    pub fn new(input: &str) -> Self {
        Self {
            program: input.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }
}

impl AdventSolver for Advent2019Day09Solver {
    fn solve_part1(&self) -> usize {
        let mut computer = Computer::new(self.program.clone());
        computer.send_input(1);
        computer.run();
        computer.receive_output() as usize
    }

    fn solve_part2(&self) -> usize {
        let mut computer = Computer::new(self.program.clone());
        computer.send_input(2);
        computer.run();
        computer.receive_output() as usize
    }
}

struct Computer {
    program: Vec<Value>,
    pointer: usize,
    input: VecDeque<Value>,
    output: VecDeque<Value>,
    state: ComputerState,
    relative_base: Value,
}

enum ComputerState {
    Running,
    Waiting,
    Halted,
}

impl Computer {
    fn new(program: Vec<Value>) -> Self {
        Self {
            program,
            pointer: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: ComputerState::Running,
            relative_base: 0,
        }
    }

    fn send_input(&mut self, input: Value) {
        self.input.push_back(input);
    }

    fn read_input(&mut self) -> Value {
        self.input.pop_front().unwrap()
    }

    fn receive_output(&mut self) -> Value {
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
                let position = self.get_address(3);
                self.set(position, a + b);
                4
            }
            2 => {
                let a = self.get_value(1);
                let b = self.get_value(2);
                let position = self.get_address(3);
                self.set(position, a * b);
                4
            }
            3 => {
                if self.input.is_empty() {
                    self.state = ComputerState::Waiting;
                    return 0;
                }
                let position = self.get_address(1);
                let value = self.read_input();
                self.set(position, value);
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
                let position = self.get_address(3);
                let value = if self.get_value(1) < self.get_value(2) { 1 } else { 0 };
                self.set(position, value);
                4
            }
            8 => {
                let position = self.get_address(3);
                let value = if self.get_value(1) == self.get_value(2) { 1 } else { 0 };
                self.set(position, value);
                4
            }
            9 => {
                let value = self.get_value(1);
                self.relative_base += value;
                2
            }
            99 => {
                self.state = ComputerState::Halted;
                1
            }
            _ => unreachable!("unknown instruction {instruction}"),
        }
    }

    fn set(&mut self, position: usize, value: Value) {
        if position < self.program.len() {
            self.program[position] = value;
        } else {
            self.program.extend(vec![0; position - self.program.len()]);
            self.program.push(value);
        }
    }

    fn get_value(&self, offset: usize) -> Value {
        let div = (10 as Value).pow(offset as u32 + 1);
        let mode = (self.program[self.pointer] / div) % 10;
        let value = self.program[self.pointer + offset];
        match mode {
            0 => self.program[value as usize],
            1 => value,
            2 => self.program[(value + self.relative_base) as usize],
            _ => unreachable!("unknown mode {mode}"),
        }
    }

    fn get_address(&self, offset: u32) -> usize {
        let div = (10 as Value).pow(offset + 1);
        let mode = (self.program[self.pointer] / div) % 10;
        let value = self.program[self.pointer + offset as usize];
        match mode {
            0 => value as usize,
            1 => unreachable!("cannot get address for immediate mode"),
            2 => (value + self.relative_base) as usize,
            _ => unreachable!("unknown mode {mode}"),
        }
    }
}
