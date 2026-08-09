use crate::solver::AdventSolver;
use std::collections::VecDeque;

pub struct Advent2019Day05Solver {
    program: Vec<i32>,
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
        while computer.output[0] == 0 {
            computer.output.pop_front();
        }
        assert_eq!(computer.output.len(), 1);
        computer.output.pop_front().unwrap() as usize
    }

    fn solve_part2(&self) -> usize {
        let mut computer = Computer::new(self.program.clone());
        computer.send_input(5);
        computer.run();
        assert_eq!(computer.output.len(), 1);
        computer.output.pop_front().unwrap() as usize
    }
}

struct Computer {
    program: Vec<i32>,
    pointer: usize,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
    is_running: bool,
}

impl Computer {
    fn new(program: Vec<i32>) -> Self {
        Self {
            program,
            pointer: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            is_running: false,
        }
    }

    fn send_input(&mut self, input: i32) {
        self.input.push_back(input);
    }

    fn read_input(&mut self) -> i32 {
        self.input.pop_front().unwrap()
    }

    fn run(&mut self) {
        self.is_running = true;
        while self.is_running {
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
                self.is_running = false;
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
