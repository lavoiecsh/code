use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use crate::solver::AdventSolver;
use itertools::Itertools;
use std::ops::BitXor;

pub struct Advent2024Day17Solver {
    register_a: Value,
    register_b: Value,
    register_c: Value,
    program: Vec<Value>,
}

impl Advent2024Day17Solver {
    pub fn new(input: &str) -> Self {
        let lines = input.lines().collect_vec();
        Self {
            register_a: lines[0].split(": ").nth(1).unwrap().parse().unwrap(),
            register_b: lines[1].split(": ").nth(1).unwrap().parse().unwrap(),
            register_c: lines[2].split(": ").nth(1).unwrap().parse().unwrap(),
            program: lines[4]
                .split(": ")
                .nth(1)
                .unwrap()
                .split(',')
                .map(|p| p.parse().unwrap())
                .collect_vec(),
        }
    }

    fn computer(&self) -> Computer {
        Computer::new(
            self.register_a,
            self.register_b,
            self.register_c,
            self.program.clone(),
        )
    }
}

impl AdventSolver for Advent2024Day17Solver {
    fn solve_part1_string(&self) -> String {
        let mut computer = self.computer();
        computer.execute();
        computer.output.pp()
    }

    fn solve_part2(&self) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        while let Some((input, ip)) = queue.pop_front() {
            for i in 0..8 {
                let next_input = (input << 3) + i;
                let mut computer = self.computer();
                computer.register_a = next_input;
                computer.execute();
                if computer.fully_matches() {
                    return next_input as usize;
                }
                if computer.reverse_matches(ip + 1) {
                    queue.push_back((next_input, ip + 1));
                }
            }
        }
        unreachable!("no solution found");
    }
}

type Value = u64;

trait PrettyPrint {
    fn pp(&self) -> String;
}

impl PrettyPrint for Vec<Value> {
    fn pp(&self) -> String {
        self.iter().join(",")
    }
}

struct Computer {
    register_a: Value,
    register_b: Value,
    register_c: Value,
    program: Vec<Value>,
    instruction_pointer: usize,
    output: Vec<Value>,
}

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Computer {
    fn new(register_a: Value, register_b: Value, register_c: Value, program: Vec<Value>) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
            program,
            instruction_pointer: 0,
            output: Vec::new(),
        }
    }

    fn fully_matches(&self) -> bool {
        self.program == self.output
    }

    fn reverse_matches(&self, count: usize) -> bool {
        if self.output.len() < count {
            return false;
        }
        self.program.iter().rev().take(count).rev().eq(&self.output)
    }

    fn execute(&mut self) {
        while self.instruction_pointer < self.program.len() {
            self.execute_operation();
        }
    }

    fn execute_operation(&mut self) {
        match self.instruction() {
            Instruction::Adv => {
                self.register_a /= 1 << self.combo_operand();
                self.instruction_pointer += 2;
            }
            Instruction::Bxl => {
                self.register_b = self.register_b.bitxor(self.literal_operand());
                self.instruction_pointer += 2;
            }
            Instruction::Bst => {
                self.register_b = self.combo_operand() % 8;
                self.instruction_pointer += 2;
            }
            Instruction::Jnz => {
                if self.register_a == 0 {
                    self.instruction_pointer += 2;
                } else {
                    self.instruction_pointer = self.literal_operand() as usize;
                }
            }
            Instruction::Bxc => {
                self.register_b = self.register_b.bitxor(self.register_c);
                self.instruction_pointer += 2;
            }
            Instruction::Out => {
                self.output.push(self.combo_operand() % 8);
                self.instruction_pointer += 2;
            }
            Instruction::Bdv => {
                self.register_b = self.register_a / (1 << self.combo_operand());
                self.instruction_pointer += 2;
            }
            Instruction::Cdv => {
                self.register_c = self.register_a / (1 << self.combo_operand());
                self.instruction_pointer += 2;
            }
        }
    }

    fn instruction(&self) -> Instruction {
        self.program[self.instruction_pointer].into()
    }

    fn literal_operand(&self) -> Value {
        self.program[self.instruction_pointer + 1]
    }

    fn combo_operand(&self) -> Value {
        match self.program[self.instruction_pointer + 1] {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => unreachable!("invalid operand 7"),
            _i => unreachable!("outside of 3 bit operand: {_i}"),
        }
    }
}

impl Debug for Computer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("\ni = {}", self.instruction_pointer))?;
        f.write_fmt(format_args!("\na = {:o}", self.register_a))?;
        f.write_fmt(format_args!("\nb = {:o}", self.register_b))?;
        f.write_fmt(format_args!("\nc = {:o}", self.register_c))?;
        f.write_fmt(format_args!("\no = {:x?}", self.output.pp()))
    }
}

impl From<Value> for Instruction {
    fn from(value: Value) -> Self {
        match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => unreachable!("unknown instruction {value}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
    static EXAMPLE_2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn finds_output_of_program() {
        assert_eq!(
            Advent2024Day17Solver::new(EXAMPLE_1).solve_part1_string(),
            "4,6,3,5,6,3,5,2,1,0"
        );
    }

    #[test]
    fn finds_lowest_register_a_copying_itself() {
        assert_eq!(Advent2024Day17Solver::new(EXAMPLE_2).solve_part2(), 117440);
    }
}
