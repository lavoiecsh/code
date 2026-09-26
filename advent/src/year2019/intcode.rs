use std::collections::VecDeque;

pub(crate) type Value = i64;

pub(crate) fn parse_program(input: &str) -> Vec<Value> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

pub(crate) struct Computer {
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
    pub(crate) fn new(program: Vec<Value>) -> Self {
        Self {
            program,
            pointer: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: ComputerState::Running,
            relative_base: 0,
        }
    }

    pub(crate) fn is_running(&self) -> bool {
        matches!(self.state, ComputerState::Waiting | ComputerState::Running)
    }

    pub(crate) fn is_waiting(&self) -> bool {
        matches!(self.state, ComputerState::Waiting)
    }

    pub(crate) fn read_value(&self, index: usize) -> Value {
        self.program[index]
    }

    pub(crate) fn send_input(&mut self, input: Value) {
        self.input.push_back(input);
    }

    pub(crate) fn send_inputs(&mut self, inputs: impl Iterator<Item = Value>) {
        self.input.extend(inputs);
    }

    pub(crate) fn send_ascii_instruction(&mut self, input: &str) {
        self.input.extend(ascii::line_to_ascii(input))
    }

    pub(crate) fn read_input(&mut self) -> Value {
        self.input.pop_front().unwrap()
    }

    pub(crate) fn receive_output(&mut self) -> Option<Value> {
        self.output.pop_front()
    }

    pub(crate) fn run(&mut self) {
        self.state = ComputerState::Running;
        while matches!(self.state, ComputerState::Running) {
            self.pointer += self.execute_operation();
        }
    }

    pub(crate) fn run_script(&mut self, print_output: bool) -> Option<Value> {
        self.run();
        while let Some(output) = self.receive_output() {
            if output > ascii::MAX {
                return Some(output);
            }
            if print_output {
                print!("{}", output as u8 as char);
            }
        }
        None
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
                let value = if self.get_value(1) < self.get_value(2) {
                    1
                } else {
                    0
                };
                self.set(position, value);
                4
            }
            8 => {
                let position = self.get_address(3);
                let value = if self.get_value(1) == self.get_value(2) {
                    1
                } else {
                    0
                };
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
            0 => {
                let address = value as usize;
                if address >= self.program.len() {
                    0
                } else {
                    self.program[address]
                }
            }
            1 => value,
            2 => {
                let address = (value + self.relative_base) as usize;
                if address >= self.program.len() {
                    0
                } else {
                    self.program[address]
                }
            }
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

#[allow(unused)]
pub(crate) mod ascii {
    use itertools::Itertools;
    use crate::year2019::intcode::Value;

    pub(crate) const MAX: Value = 127;

    pub(crate) const A: Value = 'A' as Value;
    pub(crate) const B: Value = 'B' as Value;
    pub(crate) const C: Value = 'C' as Value;
    pub(crate) const K: Value = 'K' as Value;
    pub(crate) const L: Value = 'L' as Value;
    pub(crate) const N: Value = 'N' as Value;
    pub(crate) const O: Value = 'O' as Value;
    pub(crate) const R: Value = 'R' as Value;
    pub(crate) const T: Value = 'T' as Value;
    pub(crate) const W: Value = 'W' as Value;
    pub(crate) const DOT: Value = '.' as Value;
    pub(crate) const HASH: Value = '#' as Value;
    pub(crate) const COMMA: Value = ',' as Value;
    pub(crate) const NEW_LINE: Value = 10;
    pub(crate) const UP: Value = '^' as Value;
    pub(crate) const DOWN: Value = 'v' as Value;
    pub(crate) const LEFT: Value = '<' as Value;
    pub(crate) const RIGHT: Value = '>' as Value;
    pub(crate) const YES: Value = 'y' as Value;
    pub(crate) const NO: Value = 'n' as Value;
    pub(crate) const ZERO: Value = '0' as Value;

    pub(crate) fn number_to_ascii(number: usize) -> impl Iterator<Item = Value> {
        let mut digits = vec![];
        let mut number = number;
        while number > 0 {
            digits.push((number % 10) as Value + ZERO);
            number /= 10;
        }
        digits.reverse();
        digits.into_iter()
    }

    pub(crate) fn string_to_ascii(input: &str) -> impl Iterator<Item = Value> {
        input.chars().map(|c| c as Value)
    }

    pub(crate) fn line_to_ascii(input: &str) -> impl Iterator<Item = Value> {
        input.chars().map(|c| c as Value).chain([NEW_LINE].into_iter())
    }
}
