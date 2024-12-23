use crate::solver::AdventSolver;
use std::hash::{Hash, Hasher};

pub struct Advent2021Day24Solver {
    instructions: Vec<Instruction>,
}

impl Advent2021Day24Solver {
    pub fn new(input: &str) -> Self {
        Self {
            instructions: input.lines().map(Instruction::new).collect(),
        }
    }
}

impl AdventSolver for Advent2021Day24Solver {
    fn solve_part1(&self) -> usize {
        let mut small_alus = vec![Alu::new()];
        for _ in 0..14 {
            let mut small_alus2 = Vec::new();
            for alu in &small_alus {
                for d in (1..=9).rev() {
                    let mut tmp = alu.copy();
                    tmp.execute(&self.instructions, d);
                    small_alus2.push(tmp);
                }
            }
            let min = small_alus2
                .iter()
                .map(|alu| alu.registers[3])
                .min()
                .unwrap()
                * 10;
            let max = small_alus2
                .iter()
                .map(|alu| alu.registers[3])
                .max()
                .unwrap();
            if min < max {
                small_alus = small_alus2
                    .iter()
                    .filter(|alu| alu.registers[3] <= min)
                    .map(|alu| alu.copy())
                    .collect::<Vec<Alu>>();
            } else {
                small_alus = small_alus2;
            }
        }
        small_alus.first().unwrap().input_as_usize()
    }

    fn solve_part2(&self) -> usize {
        let mut small_alus = vec![Alu::new()];
        for _ in 0..14 {
            let mut small_alus2 = Vec::new();
            for alu in &small_alus {
                for d in 1..=9 {
                    let mut tmp = alu.copy();
                    tmp.execute(&self.instructions, d);
                    small_alus2.push(tmp);
                }
            }
            let min = small_alus2
                .iter()
                .map(|alu| alu.registers[3])
                .min()
                .unwrap()
                * 10;
            let max = small_alus2
                .iter()
                .map(|alu| alu.registers[3])
                .max()
                .unwrap();
            if min < max {
                small_alus = small_alus2
                    .iter()
                    .filter(|alu| alu.registers[3] <= min)
                    .map(|alu| alu.copy())
                    .collect::<Vec<Alu>>();
            } else {
                small_alus = small_alus2;
            }
        }
        small_alus.first().unwrap().input_as_usize()
    }
}

#[derive(Eq)]
struct Alu {
    registers: [i64; 4],
    ip: usize,
    input: Vec<i64>,
}

impl PartialEq for Alu {
    fn eq(&self, other: &Self) -> bool {
        self.registers[2] == other.registers[2]
    }
}

impl Hash for Alu {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.registers[2].hash(state);
    }
}

impl Alu {
    fn new() -> Self {
        Self {
            registers: [0; 4],
            ip: 0,
            input: Vec::new(),
        }
    }

    fn copy(&self) -> Self {
        Self {
            registers: [
                self.registers[0],
                self.registers[1],
                self.registers[2],
                self.registers[3],
            ],
            ip: self.ip,
            input: self.input.to_vec(),
        }
    }

    fn execute(&mut self, instructions: &[Instruction], input: i64) {
        let mut input_used = false;
        while self.ip < instructions.len() {
            match instructions[self.ip] {
                Instruction::Inp(a) => {
                    if input_used {
                        return;
                    }
                    self.registers[a] = input;
                    self.input.push(input);
                    input_used = true;
                }
                Instruction::AddReg(a, b) => {
                    self.registers[a] += self.registers[b];
                }
                Instruction::AddInt(a, b) => {
                    self.registers[a] += b;
                }
                Instruction::MulReg(a, b) => {
                    self.registers[a] *= self.registers[b];
                }
                Instruction::MulInt(a, b) => {
                    self.registers[a] *= b;
                }
                Instruction::DivReg(a, b) => {
                    self.registers[a] /= self.registers[b];
                }
                Instruction::DivInt(a, b) => {
                    self.registers[a] /= b;
                }
                Instruction::ModReg(a, b) => {
                    self.registers[a] %= self.registers[b];
                }
                Instruction::ModInt(a, b) => {
                    self.registers[a] %= b;
                }
                Instruction::EqlReg(a, b) => {
                    self.registers[a] = if self.registers[a] == self.registers[b] {
                        1
                    } else {
                        0
                    };
                }
                Instruction::EqlInt(a, b) => {
                    self.registers[a] = if self.registers[a] == b { 1 } else { 0 };
                }
            }
            self.ip += 1;
        }
    }

    fn input_as_usize(&self) -> usize {
        self.input.iter().fold(0, |acc, c| acc * 10 + *c as usize)
    }
}

enum Instruction {
    Inp(usize),
    AddReg(usize, usize),
    AddInt(usize, i64),
    MulReg(usize, usize),
    MulInt(usize, i64),
    DivReg(usize, usize),
    DivInt(usize, i64),
    ModReg(usize, usize),
    ModInt(usize, i64),
    EqlReg(usize, usize),
    EqlInt(usize, i64),
}

impl Instruction {
    fn new(inst: &str) -> Instruction {
        let mut split = inst.split(" ");
        let inst = split.next().unwrap();
        let reg = Instruction::reg_letter_to_index(split.next().unwrap());
        if inst == "inp" {
            return Instruction::Inp(reg);
        }
        let last = split.next().unwrap();
        let last_is_reg = Instruction::is_reg(last);
        match (inst, last_is_reg) {
            ("add", true) => Instruction::AddReg(reg, Instruction::reg_letter_to_index(last)),
            ("add", false) => Instruction::AddInt(reg, last.parse().unwrap()),
            ("mul", true) => Instruction::MulReg(reg, Instruction::reg_letter_to_index(last)),
            ("mul", false) => Instruction::MulInt(reg, last.parse().unwrap()),
            ("div", true) => Instruction::DivReg(reg, Instruction::reg_letter_to_index(last)),
            ("div", false) => Instruction::DivInt(reg, last.parse().unwrap()),
            ("mod", true) => Instruction::ModReg(reg, Instruction::reg_letter_to_index(last)),
            ("mod", false) => Instruction::ModInt(reg, last.parse().unwrap()),
            ("eql", true) => Instruction::EqlReg(reg, Instruction::reg_letter_to_index(last)),
            ("eql", false) => Instruction::EqlInt(reg, last.parse().unwrap()),
            _ => panic!("unknown instruction"),
        }
    }

    fn reg_letter_to_index(letter: &str) -> usize {
        letter.chars().next().unwrap() as usize - 119
    }

    fn is_reg(input: &str) -> bool {
        input == "w" || input == "x" || input == "y" || input == "z"
    }
}
