use regex::{Match, Regex};

use crate::solver::AdventSolver;

type Instructions = Vec<Box<dyn Instruction>>;

pub struct Advent2015Day23Solver {
    instructions: Instructions
}

impl Advent2015Day23Solver {
    pub fn new(input: String) -> Self {
        Self {
            instructions: input
                .lines()
                .map(line_to_instruction)
                .collect()
        }
    }
}

impl AdventSolver for Advent2015Day23Solver {
    fn solve_part1(&self) -> usize {
        let mut computer = Computer::new();
        computer.run(&self.instructions);
        computer.register_b as usize
    }

    fn solve_part2(&self) -> usize {
        let mut computer = Computer::new();
        computer.register_a = 1;
        computer.run(&self.instructions);
        computer.register_b as usize
    }
}

#[derive(Debug, Copy, Clone)]
struct Computer {
    register_a: isize,
    register_b: isize,
    pointer: usize,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            register_a: 0,
            register_b: 0,
            pointer: 0
        }
    }

    fn is_finished(&self, instructions: &Instructions) -> bool {
        self.pointer >= instructions.len()
    }

    fn run(&mut self, instructions: &Instructions) {
        while !self.is_finished(instructions) {
            instructions[self.pointer].execute(self);
        }
    }
}

trait Instruction {
    fn execute(&self, computer: &mut Computer);
}

#[derive(Debug, Copy, Clone)]
struct Half {
    register: char,
}

impl Instruction for Half {
    fn execute(&self, computer: &mut Computer) {
        if self.register == 'a' {
            computer.register_a /= 2;
        }
        if self.register == 'b' {
            computer.register_b /= 2;
        }
        computer.pointer += 1;
    }
}

#[derive(Debug, Copy, Clone)]
struct Triple {
    register: char,
}

impl Instruction for Triple {
    fn execute(&self, computer: &mut Computer) {
        if self.register == 'a' {
            computer.register_a *= 3;
        }
        if self.register == 'b' {
            computer.register_b *= 3;
        }
        computer.pointer += 1;
    }
}

#[derive(Debug, Copy, Clone)]
struct Increment {
    register: char,
}

impl Instruction for Increment {
    fn execute(&self, computer: &mut Computer) {
        if self.register == 'a' {
            computer.register_a += 1;
        }
        if self.register == 'b' {
            computer.register_b += 1;
        }
        computer.pointer += 1;
    }
}

#[derive(Debug, Copy, Clone)]
struct Jump {
    offset: isize,
}

impl Instruction for Jump {
    fn execute(&self, computer: &mut Computer) {
        computer.pointer = (computer.pointer as isize + self.offset) as usize;
    }
}

#[derive(Debug, Copy, Clone)]
struct JumpIfEven {
    register: char,
    offset: isize,
}

impl Instruction for JumpIfEven {
    fn execute(&self, computer: &mut Computer) {
        let register = if self.register == 'a' { computer.register_a } else { computer.register_b };
        if register % 2 == 0 {
            computer.pointer = (computer.pointer as isize + self.offset) as usize;
        } else {
            computer.pointer += 1;
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct JumpIfOne {
    register: char,
    offset: isize,
}

impl Instruction for JumpIfOne {
    fn execute(&self, computer: &mut Computer) {
        let register = if self.register == 'a' { computer.register_a } else { computer.register_b };
        if register == 1 {
            computer.pointer = (computer.pointer as isize + self.offset) as usize;
        } else {
            computer.pointer += 1;
        }
    }
}

fn line_to_instruction(line: &str) -> Box<dyn Instruction> {
    let half_regex: Regex = Regex::new(r"hlf (\w+)").unwrap();
    let triple_regex: Regex = Regex::new(r"tpl (\w+)").unwrap();
    let increment_regex: Regex = Regex::new(r"inc (\w+)").unwrap();
    let jump_regex: Regex = Regex::new(r"jmp ([+-]\d+)").unwrap();
    let jump_if_even_regex: Regex = Regex::new(r"jie (\w+), ([+-]\d+)").unwrap();
    let jump_if_one_regex: Regex = Regex::new(r"jio (\w+), ([+-]\d+)").unwrap();
    let first_char = |cap: Option<Match>| cap.unwrap().as_str().chars().next().unwrap();
    let as_number = |cap: Option<Match>| cap.unwrap().as_str().parse().unwrap();

    if let Some(cap) = half_regex.captures(line) {
        return Box::new(Half { register: first_char(cap.get(1)) });
    }

    if let Some(cap) = triple_regex.captures(line) {
        return Box::new(Triple { register: first_char(cap.get(1)) });
    }

    if let Some(cap) = increment_regex.captures(line) {
        return Box::new(Increment { register: first_char(cap.get(1)) });
    }

    if let Some(cap) = jump_regex.captures(line) {
        return Box::new(Jump { offset: as_number(cap.get(1)) });
    }

    if let Some(cap) = jump_if_even_regex.captures(line) {
        return Box::new(JumpIfEven { register: first_char(cap.get(1)), offset: as_number(cap.get(2)) });
    }

    if let Some(cap) = jump_if_one_regex.captures(line) {
        return Box::new(JumpIfOne { register: first_char(cap.get(1)), offset: as_number(cap.get(2)) });
    }

    panic!("unknown instruction {}", line);
}
