use crate::solver::AdventSolver;
use crate::year2018::day19::Operation::*;
use std::fmt::{Debug, Formatter};

pub struct Advent2018Day19Solver {
    ip: Register,
    program: Vec<Operation>,
}

impl Advent2018Day19Solver {
    pub fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let ip = lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();
        Self {
            ip,
            program: lines.map(Operation::from).collect(),
        }
    }
}

impl AdventSolver for Advent2018Day19Solver {
    fn solve_part1(&self) -> usize {
        let mut computer = Computer::new(self.ip);
        computer.execute_program(&self.program);
        computer.registers[0]
    }

    fn solve_part2(&self) -> usize {
        // sum of divisors of number in register 1 when register 0 is not 1
        let mut computer = Computer::new(self.ip);
        computer.registers[0] = 1;
        computer.execute_program_until(&self.program, |c| c.registers[0] != 1);
        let target = computer.registers[1];
        (1..=target).filter(|i| target % i == 0).sum()
    }
}

#[derive(Eq, PartialEq)]
struct Computer {
    registers: [Value; 6],
    ip: Register,
}

impl Debug for Computer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.registers))
    }
}

impl Computer {
    fn new(ip: Register) -> Self {
        Self {
            registers: [0, 0, 0, 0, 0, 0],
            ip,
        }
    }
}

impl Computer {
    fn execute_program(&mut self, program: &[Operation]) {
        while self.registers[self.ip] < program.len() {
            self.execute_operation(&program[self.registers[self.ip]]);
            self.registers[self.ip] += 1;
        }
        self.registers[self.ip] -= 1;
    }

    fn execute_program_until(
        &mut self,
        program: &[Operation],
        stop_condition: impl Fn(&Self) -> bool,
    ) {
        while self.registers[self.ip] < program.len() && !stop_condition(self) {
            self.execute_operation(&program[self.registers[self.ip]]);
            self.registers[self.ip] += 1;
        }
        self.registers[self.ip] -= 1;
    }

    fn execute_operation(&mut self, operation: &Operation) {
        match *operation {
            Addr(a, b, c) => self.registers[c] = self.registers[a] + self.registers[b],
            Addi(a, b, c) => self.registers[c] = self.registers[a] + b,
            Mulr(a, b, c) => self.registers[c] = self.registers[a] * self.registers[b],
            Muli(a, b, c) => self.registers[c] = self.registers[a] * b,
            Banr(a, b, c) => self.registers[c] = self.registers[a] & self.registers[b],
            Bani(a, b, c) => self.registers[c] = self.registers[a] & b,
            Borr(a, b, c) => self.registers[c] = self.registers[a] | self.registers[b],
            Bori(a, b, c) => self.registers[c] = self.registers[a] | b,
            Setr(a, c) => self.registers[c] = self.registers[a],
            Seti(a, c) => self.registers[c] = a,
            Gtir(a, b, c) => self.registers[c] = if a > self.registers[b] { 1 } else { 0 },
            Gtri(a, b, c) => self.registers[c] = if self.registers[a] > b { 1 } else { 0 },
            Gtrr(a, b, c) => {
                self.registers[c] = if self.registers[a] > self.registers[b] {
                    1
                } else {
                    0
                }
            }
            Eqir(a, b, c) => self.registers[c] = if a == self.registers[b] { 1 } else { 0 },
            Eqri(a, b, c) => self.registers[c] = if self.registers[a] == b { 1 } else { 0 },
            Eqrr(a, b, c) => {
                self.registers[c] = if self.registers[a] == self.registers[b] {
                    1
                } else {
                    0
                }
            }
        }
    }
}

type Register = usize;
type Value = usize;

#[derive(PartialEq, Debug)]
enum Operation {
    Addr(Register, Register, Register),
    Addi(Register, Value, Register),
    Mulr(Register, Register, Register),
    Muli(Register, Value, Register),
    Banr(Register, Register, Register),
    Bani(Register, Value, Register),
    Borr(Register, Register, Register),
    Bori(Register, Value, Register),
    Setr(Register, Register),
    Seti(Value, Register),
    Gtir(Value, Register, Register),
    Gtri(Register, Value, Register),
    Gtrr(Register, Register, Register),
    Eqir(Value, Register, Register),
    Eqri(Register, Value, Register),
    Eqrr(Register, Register, Register),
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        let split = value.split(' ').collect::<Vec<_>>();
        let values = split
            .iter()
            .skip(1)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        match split[0] {
            "addr" => Addr(values[0], values[1], values[2]),
            "addi" => Addi(values[0], values[1], values[2]),
            "mulr" => Mulr(values[0], values[1], values[2]),
            "muli" => Muli(values[0], values[1], values[2]),
            "banr" => Banr(values[0], values[1], values[2]),
            "bani" => Bani(values[0], values[1], values[2]),
            "borr" => Borr(values[0], values[1], values[2]),
            "bori" => Bori(values[0], values[1], values[2]),
            "setr" => Setr(values[0], values[2]),
            "seti" => Seti(values[0], values[2]),
            "gtir" => Gtir(values[0], values[1], values[2]),
            "gtri" => Gtri(values[0], values[1], values[2]),
            "gtrr" => Gtrr(values[0], values[1], values[2]),
            "eqir" => Eqir(values[0], values[1], values[2]),
            "eqri" => Eqri(values[0], values[1], values[2]),
            "eqrr" => Eqrr(values[0], values[1], values[2]),
            op => unreachable!("operation not found: {}", op),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
";

    #[test]
    fn executes_program() {
        let solver = Advent2018Day19Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 6);
    }
}
