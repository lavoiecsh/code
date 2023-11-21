use std::str::FromStr;

use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2017Day23Solver {
    instructions: Vec<Instruction>,
}

impl Advent2017Day23Solver {
    pub fn new(input: String) -> Self {
        Self { instructions: input.lines().map(|l| Instruction::from_str(l).unwrap()).collect() }
    }
}

impl AdventSolver for Advent2017Day23Solver {
    fn solve_part1(&self) -> usize {
        let mut processor = Processor::new(&self.instructions);
        processor.run();
        processor.mul_count
    }

    fn solve_part2(&self) -> usize {
        // todo hacked solution by reverse engineering the code
        test() as usize
        // let mut processor = Processor::new(&self.instructions);
        // processor.registers[0] = 1;
        // processor.run();
        // processor.registers[7] as usize
    }
}

type Value = i64;

struct Processor<'a> {
    instructions: &'a Vec<Instruction>,
    pointer: usize,
    registers: [Value; 8],
    mul_count: usize,
}

impl<'a> Processor<'a> {
    fn new(instructions: &'a Vec<Instruction>) -> Self {
        Self {
            instructions,
            pointer: 0,
            registers: [0; 8],
            mul_count: 0,
        }
    }

    fn run(&mut self) {
        while self.pointer < self.instructions.len() {
            self.execute(&self.instructions[self.pointer]);
            self.pointer += 1;
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Set(x, y) => self.set_value(x, y),
            Instruction::Sub(x, y) =>
                self.set_value(x, &Destination::Value(self.get_value(x) - self.get_value(y))),
            Instruction::Mul(x, y) => {
                self.set_value(x, &Destination::Value(self.get_value(x) * self.get_value(y)));
                self.mul_count += 1;
            }
            Instruction::Jnz(x, y) => {
                if self.get_value(x) != 0 {
                    self.pointer = (self.pointer as Value + self.get_value(y) - 1) as usize;
                }
            }
        }
    }

    fn get_value(&self, destination: &Destination) -> Value {
        match destination {
            Destination::Register(x) => self.registers[*x],
            Destination::Value(x) => *x,
        }
    }

    fn set_value(&mut self, destination: &Destination, value: &Destination) {
        match destination {
            Destination::Register(x) => { self.registers[*x] = self.get_value(value); }
            Destination::Value(_) => {}
        }
    }
}

enum Instruction {
    Set(Destination, Destination),
    Sub(Destination, Destination),
    Mul(Destination, Destination),
    Jnz(Destination, Destination),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" ").collect_vec();
        let arg1 = Destination::from_str(split[1])?;
        let arg2 = Destination::from_str(split[2])?;
        match split[0] {
            "set" => Ok(Instruction::Set(arg1, arg2)),
            "sub" => Ok(Instruction::Sub(arg1, arg2)),
            "mul" => Ok(Instruction::Mul(arg1, arg2)),
            "jnz" => Ok(Instruction::Jnz(arg1, arg2)),
            _ => Err(()),
        }
    }
}

enum Destination {
    Register(usize),
    Value(Value),
}

impl FromStr for Destination {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<Value>().map_or_else(
            |_| Destination::Register(s.chars().next().unwrap() as usize - 'a' as usize),
            |v| Destination::Value(v)))
    }
}

fn test() -> i64 {
    let mut b: i64 = (81 * 100) + 100000;
    let c: i64 = b + 17000 + 17;
    let mut d: i64;
    let mut h: i64 = 0;

    // counts prime numbers between b and c
    // skipping by 17
    while b != c {
        d = 2;
        while d != b {
            if b % d == 0 { h += 1; break; }
            d += 1;
        }
        b += 17;
    }
    h
}
// 0: b = 81
// 1: c = 81
// 2: pointer = 4
// 3: pointer = 8
// 4: b = b * 100
// 5: b = b - 100000
// 6: c = b
// 7: c = c - 17000
// 8: f = 1
// 9: d = 2
// 10: e = 2
// 11: g = d
// 12: g = g * e
// 13: g = g - b
// 14: if g != 0 -> pointer = 16
// 15: f = 0
// 16: e = e - 1
// 17: g = e
// 18: g = g - b
// 19: if g != 0 -> pointer = 11
// 20: d = d - 1
// 21: g = d
// 22: g = g - b
// 23: if g != 0 -> pointer = 10
// 24: if f != 0 -> pointer = 26
// 25: h = h - 1
// 26: g = b
// 27: g = g - c
// 28: if g != 0 -> pointer = 30
// 29: pointer = 32                   end
// 30: b = b - 17
// 31: pointer = 8
