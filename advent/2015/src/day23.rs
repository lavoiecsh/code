use std::fs;
use regex::Regex;

const FILENAME: &str = "inputs/day23.txt";
type Instructions = Vec<Box<dyn Instruction>>;

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

    fn is_finished(self: &Self, instructions: &Instructions) -> bool {
        self.pointer >= instructions.len()
    }

    fn run(self: &mut Self, instructions: &Instructions) {
        while !self.is_finished(instructions) {
            instructions[self.pointer].execute(self);
        }
    }
}

trait Instruction {
    fn execute(self: &Self, computer: &mut Computer);
}

#[derive(Debug, Copy, Clone)]
struct Half {
    register: char,
}

impl Instruction for Half {
    fn execute(self: &Self, computer: &mut Computer) {
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
    fn execute(self: &Self, computer: &mut Computer) {
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
    fn execute(self: &Self, computer: &mut Computer) {
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
    fn execute(self: &Self, computer: &mut Computer) {
        computer.pointer = (computer.pointer as isize + self.offset) as usize;
    }
}

#[derive(Debug, Copy, Clone)]
struct JumpIfEven {
    register: char,
    offset: isize,
}

impl Instruction for JumpIfEven {
    fn execute(self: &Self, computer: &mut Computer) {
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
    fn execute(self: &Self, computer: &mut Computer) {
        let register = if self.register == 'a' { computer.register_a } else { computer.register_b };
        if register == 1 {
            computer.pointer = (computer.pointer as isize + self.offset) as usize;
        } else {
            computer.pointer += 1;
        }
    }
}

fn read_input() -> Vec<Box<dyn Instruction>> {
    fs::read_to_string(FILENAME)
        .expect("error reading")
        .trim()
        .lines()
        .map(line_to_instruction)
        .collect()
}

fn line_to_instruction(line: &str) -> Box<dyn Instruction> {
    let half_regex: Regex = Regex::new(r"hlf (\w+)").unwrap();
    let triple_regex: Regex = Regex::new(r"tpl (\w+)").unwrap();
    let increment_regex: Regex = Regex::new(r"inc (\w+)").unwrap();
    let jump_regex: Regex = Regex::new(r"jmp ([+-]\d+)").unwrap();
    let jump_if_even_regex: Regex = Regex::new(r"jie (\w+), ([+-]\d+)").unwrap();
    let jump_if_one_regex: Regex = Regex::new(r"jio (\w+), ([+-]\d+)").unwrap();

    let mut captures = half_regex.captures(line);
    if captures.is_some() {
        return Box::new(Half { register: captures.unwrap().get(1).unwrap().as_str().chars().next().unwrap() });
    }

    captures = triple_regex.captures(line);
    if captures.is_some() {
        return Box::new(Triple { register: captures.unwrap().get(1).unwrap().as_str().chars().next().unwrap() });
    }

    captures = increment_regex.captures(line);
    if captures.is_some() {
        return Box::new(Increment { register: captures.unwrap().get(1).unwrap().as_str().chars().next().unwrap() });
    }

    captures = jump_regex.captures(line);
    if captures.is_some() {
        return Box::new(Jump { offset: captures.unwrap().get(1).unwrap().as_str().parse().unwrap() });
    }

    captures = jump_if_even_regex.captures(line);
    if captures.is_some() {
        let c = captures.unwrap();
        return Box::new(JumpIfEven { register: c.get(1).unwrap().as_str().chars().next().unwrap(), offset: c.get(2).unwrap().as_str().parse().unwrap() });
    }

    captures = jump_if_one_regex.captures(line);
    if captures.is_some() {
        let c = captures.unwrap();
        return Box::new(JumpIfOne { register: c.get(1).unwrap().as_str().chars().next().unwrap(), offset: c.get(2).unwrap().as_str().parse().unwrap() });
    }

    panic!("unknown instruction {}", line);
}

pub fn part1() -> usize {
    let instructions = read_input();
    let mut computer = Computer::new();
    computer.run(&instructions);
    computer.register_b as usize
}

pub fn part2() -> usize {
    let instructions = read_input();
    let mut computer = Computer::new();
    computer.register_a = 1;
    computer.run(&instructions);
    computer.register_b as usize
}
