use crate::solver::AdventSolver;
use crate::year2018::day16::Operation::{
    Bani, Banr, Bori, Borr, Eqir, Eqri, Eqrr, Gtir, Gtri, Gtrr, Muli, Mulr, Seti, Setr,
};
use itertools::Itertools;
use regex::{Captures, Regex};
use Operation::{Addi, Addr};

pub struct Advent2018Day16Solver {
    samples: Vec<Sample>,
    operations: Vec<[u32; 4]>,
}

impl Advent2018Day16Solver {
    pub fn new(input: &str) -> Self {
        let before_re = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
        let op_re = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
        let after_re = Regex::new(r"After:  \[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
        let mut reading_sample = false;
        let mut samples = vec![];
        let mut operations = vec![];
        let mut temp = TempSample::new();
        let to_array =
            |cap: Captures| [1, 2, 3, 4].map(|i| cap.get(i).unwrap().as_str().parse().unwrap());
        for line in input.lines() {
            if reading_sample {
                if let Some(cap) = op_re.captures(line) {
                    temp.operation = Some(to_array(cap));
                }
                if let Some(cap) = after_re.captures(line) {
                    temp.after = Some(to_array(cap));
                }
                if temp.is_complete() {
                    samples.push(temp.sample());
                    reading_sample = false;
                }
            } else if let Some(cap) = before_re.captures(line) {
                temp.before = Some(to_array(cap));
                reading_sample = true;
            } else if let Some(cap) = op_re.captures(line) {
                operations.push(to_array(cap));
            }
        }
        Self {
            samples,
            operations,
        }
    }
}

struct TempSample {
    before: Option<[u32; 4]>,
    after: Option<[u32; 4]>,
    operation: Option<[u32; 4]>,
}

impl TempSample {
    fn new() -> Self {
        Self {
            before: None,
            after: None,
            operation: None,
        }
    }

    fn is_complete(&self) -> bool {
        self.before.is_some() && self.after.is_some() && self.operation.is_some()
    }

    fn sample(&mut self) -> Sample {
        let s = Sample {
            before: self.before.unwrap(),
            after: self.after.unwrap(),
            operation: self.operation.unwrap(),
        };
        self.before = None;
        self.after = None;
        self.operation = None;
        s
    }
}

impl AdventSolver for Advent2018Day16Solver {
    fn solve_part1(&self) -> usize {
        self.samples
            .iter()
            .map(|s| s.possible_operations().len())
            .filter(|&c| c >= 3)
            .count()
    }

    fn solve_part2(&self) -> usize {
        let mut matches = OperationMatches::new();
        self.samples.iter().for_each(|s| matches.add_sample(s));
        let mut computer = Computer::new();
        self.operations
            .iter()
            .map(|&o| matches.operation_for(o))
            .for_each(|o| {
                computer = computer.execute(&o);
            });
        computer.registers[0] as usize
    }
}

#[derive(Debug)]
struct Sample {
    before: [u32; 4],
    after: [u32; 4],
    operation: [u32; 4],
}

impl Sample {
    fn op_code(&self) -> usize {
        self.operation[0] as usize
    }

    fn possible_operations(&self) -> Vec<Operation> {
        let before = Computer {
            registers: self.before,
        };
        let after = Computer {
            registers: self.after,
        };
        let a = self.operation[1];
        let b = self.operation[2];
        let c = self.operation[3] as Register;
        let mut possible = vec![];
        if a < 4 {
            possible.push(Addi(a as Register, b, c));
            possible.push(Muli(a as Register, b, c));
            possible.push(Bani(a as Register, b, c));
            possible.push(Bori(a as Register, b, c));
            possible.push(Setr(a as Register, c));
            possible.push(Gtri(a as Register, b, c));
            possible.push(Eqri(a as Register, b, c));
            if b < 4 {
                possible.push(Addr(a as Register, b as Register, c));
                possible.push(Mulr(a as Register, b as Register, c));
                possible.push(Banr(a as Register, b as Register, c));
                possible.push(Borr(a as Register, b as Register, c));
                possible.push(Gtrr(a as Register, b as Register, c));
                possible.push(Eqrr(a as Register, b as Register, c));
            }
        }
        if b < 4 {
            possible.push(Gtir(a, b as Register, c));
            possible.push(Eqir(a, b as Register, c));
        }
        possible.push(Seti(a, c));
        possible
            .into_iter()
            .filter(|o| before.execute(o) == after)
            .collect_vec()
    }
}

#[derive(Debug)]
struct OperationMatches {
    matches: Vec<Vec<OperationType>>,
}

impl OperationMatches {
    fn new() -> Self {
        let mut matches = vec![];
        matches.resize_with(16, Vec::new);
        Self { matches }
    }

    fn add_sample(&mut self, sample: &Sample) {
        let possible_operation_types = sample
            .possible_operations()
            .iter()
            .map(Operation::operation_type)
            .collect_vec();
        if self.matches[sample.op_code()].is_empty() {
            self.matches[sample.op_code()] = possible_operation_types;
        } else {
            self.matches[sample.op_code()] = self.matches[sample.op_code()]
                .iter()
                .filter(|o| possible_operation_types.contains(o))
                .cloned()
                .collect_vec();
        }
        if self.matches[sample.op_code()].len() == 1 {
            for i in 0..16 {
                if i == sample.op_code() {
                    continue;
                }
                if let Some(j) = self.matches[i]
                    .iter()
                    .position(|o| o == &self.matches[sample.op_code()][0])
                {
                    self.matches[i].remove(j);
                }
            }
        }
    }

    fn operation_for(&self, operation: [u32; 4]) -> Operation {
        match self.matches[operation[0] as usize][0] {
            OperationType::Addr => Addr(
                operation[1] as Register,
                operation[2] as Register,
                operation[3] as Register,
            ),
            OperationType::Addi => Addi(
                operation[1] as Register,
                operation[2],
                operation[3] as Register,
            ),
            OperationType::Mulr => Mulr(
                operation[1] as Register,
                operation[2] as Register,
                operation[3] as Register,
            ),
            OperationType::Muli => Muli(
                operation[1] as Register,
                operation[2],
                operation[3] as Register,
            ),
            OperationType::Banr => Banr(
                operation[1] as Register,
                operation[2] as Register,
                operation[3] as Register,
            ),
            OperationType::Bani => Bani(
                operation[1] as Register,
                operation[2],
                operation[3] as Register,
            ),
            OperationType::Borr => Borr(
                operation[1] as Register,
                operation[2] as Register,
                operation[3] as Register,
            ),
            OperationType::Bori => Bori(
                operation[1] as Register,
                operation[2],
                operation[3] as Register,
            ),
            OperationType::Setr => Setr(operation[1] as Register, operation[3] as Register),
            OperationType::Seti => Seti(operation[1], operation[3] as Register),
            OperationType::Gtir => Gtir(
                operation[1],
                operation[2] as Register,
                operation[3] as Register,
            ),
            OperationType::Gtri => Gtri(
                operation[1] as Register,
                operation[2],
                operation[3] as Register,
            ),
            OperationType::Gtrr => Gtrr(
                operation[1] as Register,
                operation[2] as Register,
                operation[3] as Register,
            ),
            OperationType::Eqir => Eqir(
                operation[1],
                operation[2] as Register,
                operation[3] as Register,
            ),
            OperationType::Eqri => Eqri(
                operation[1] as Register,
                operation[2],
                operation[3] as Register,
            ),
            OperationType::Eqrr => Eqrr(
                operation[1] as Register,
                operation[2] as Register,
                operation[3] as Register,
            ),
        }
    }
}

#[derive(Eq, PartialEq)]
struct Computer {
    registers: [u32; 4],
}

impl Computer {
    fn new() -> Self {
        Self {
            registers: [0, 0, 0, 0],
        }
    }
}

impl Computer {
    fn execute(&self, operation: &Operation) -> Self {
        let mut registers = self.registers;
        match *operation {
            Addr(a, b, c) => registers[c] = registers[a] + registers[b],
            Addi(a, b, c) => registers[c] = registers[a] + b,
            Mulr(a, b, c) => registers[c] = registers[a] * registers[b],
            Muli(a, b, c) => registers[c] = registers[a] * b,
            Banr(a, b, c) => registers[c] = registers[a] & registers[b],
            Bani(a, b, c) => registers[c] = registers[a] & b,
            Borr(a, b, c) => registers[c] = registers[a] | registers[b],
            Bori(a, b, c) => registers[c] = registers[a] | b,
            Setr(a, c) => registers[c] = registers[a],
            Seti(a, c) => registers[c] = a,
            Gtir(a, b, c) => registers[c] = if a > registers[b] { 1 } else { 0 },
            Gtri(a, b, c) => registers[c] = if registers[a] > b { 1 } else { 0 },
            Gtrr(a, b, c) => registers[c] = if registers[a] > registers[b] { 1 } else { 0 },
            Eqir(a, b, c) => registers[c] = if a == registers[b] { 1 } else { 0 },
            Eqri(a, b, c) => registers[c] = if registers[a] == b { 1 } else { 0 },
            Eqrr(a, b, c) => registers[c] = if registers[a] == registers[b] { 1 } else { 0 },
        }
        Self { registers }
    }
}

type Register = usize;
type Value = u32;

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

impl Operation {
    fn operation_type(&self) -> OperationType {
        match self {
            Addr(_, _, _) => OperationType::Addr,
            Addi(_, _, _) => OperationType::Addi,
            Mulr(_, _, _) => OperationType::Mulr,
            Muli(_, _, _) => OperationType::Muli,
            Banr(_, _, _) => OperationType::Banr,
            Bani(_, _, _) => OperationType::Bani,
            Borr(_, _, _) => OperationType::Borr,
            Bori(_, _, _) => OperationType::Bori,
            Setr(_, _) => OperationType::Setr,
            Seti(_, _) => OperationType::Seti,
            Gtir(_, _, _) => OperationType::Gtir,
            Gtri(_, _, _) => OperationType::Gtri,
            Gtrr(_, _, _) => OperationType::Gtrr,
            Eqir(_, _, _) => OperationType::Eqir,
            Eqri(_, _, _) => OperationType::Eqri,
            Eqrr(_, _, _) => OperationType::Eqrr,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum OperationType {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
    ";

    #[test]
    fn counts_matching_opcodes() {
        let solver = Advent2018Day16Solver::new(EXAMPLE);
        assert_eq!(
            solver.samples[0].possible_operations(),
            vec!(Addi(2, 1, 2), Mulr(2, 1, 2), Seti(2, 2),)
        );
    }
}
