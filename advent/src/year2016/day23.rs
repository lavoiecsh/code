use crate::solver::AdventSolver;

use Instruction::*;

pub struct Advent2016Day23Solver {
    instructions: Vec<Instruction>,
}

impl Advent2016Day23Solver {
    pub fn new(input: &str) -> Self {
        let to_index = |r: &str| r.chars().next().unwrap() as usize - 'a' as usize;
        Self {
            instructions: input
                .lines()
                .map(|l| {
                    let s = l.split(" ").collect::<Vec<&str>>();
                    match s[0] {
                        "cpy" => s[1].parse().map_or_else(
                            |_| CopyRegisterRegister(to_index(s[1]), to_index(s[2])),
                            |v| CopyValueRegister(v, to_index(s[2])),
                        ),
                        "inc" => IncrementRegister(to_index(s[1])),
                        "dec" => DecrementRegister(to_index(s[1])),
                        "jnz" => match (s[1].parse(), s[2].parse()) {
                            (Ok(a), Ok(b)) => JumpNotZeroValueValue(a, b),
                            (Ok(a), Err(_)) => JumpNotZeroValueRegister(a, to_index(s[2])),
                            (Err(_), Ok(b)) => JumpNotZeroRegisterValue(to_index(s[1]), b),
                            (Err(_), Err(_)) => {
                                JumpNotZeroRegisterRegister(to_index(s[1]), to_index(s[2]))
                            }
                        },
                        "tgl" => s[1]
                            .parse()
                            .map_or_else(|_| ToggleRegister(to_index(s[1])), ToggleValue),
                        i => panic!("unknown instruction {i} in line {l}"),
                    }
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2016Day23Solver {
    fn solve_part1(&self) -> usize {
        let mut computer = Computer::new(&self.instructions);
        computer.registers[0] = 7;
        computer.run();
        computer.registers[0] as usize
    }

    fn solve_part2(&self) -> usize {
        let mut computer = Computer::new(&self.instructions);
        computer.registers[0] = 12;
        computer.run();
        computer.registers[0] as usize
    }
}

type Value = i32;
type Register = usize;

struct Computer {
    registers: [Value; 4],
    pointer: usize,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn new(instructions: &[Instruction]) -> Self {
        Self {
            registers: [0; 4],
            pointer: 0,
            instructions: instructions.to_vec(),
        }
    }

    fn run(&mut self) {
        while !self.is_completed() {
            self.execute();
            self.pointer += 1;
        }
    }

    fn is_completed(&self) -> bool {
        self.pointer >= self.instructions.len()
    }

    fn execute(&mut self) {
        match self.instructions[self.pointer] {
            CopyRegisterRegister(i, o) => self.registers[o] = self.registers[i],
            CopyRegisterValue(_, _) => {}
            CopyValueRegister(v, o) => self.registers[o] = v,
            CopyValueValue(_, _) => {}
            IncrementRegister(r) => self.registers[r] += 1,
            IncrementValue(_) => {}
            DecrementRegister(r) => self.registers[r] -= 1,
            DecrementValue(_) => {}
            JumpNotZeroRegisterRegister(r, p) => {
                if self.registers[r] != 0 {
                    self.move_pointer(self.registers[p])
                }
            }
            JumpNotZeroRegisterValue(r, p) => {
                if self.registers[r] != 0 {
                    self.move_pointer(p)
                }
            }
            JumpNotZeroValueRegister(v, p) => {
                if v != 0 {
                    self.move_pointer(self.registers[p])
                }
            }
            JumpNotZeroValueValue(v, p) => {
                if v != 0 {
                    self.move_pointer(p)
                }
            }
            ToggleValue(v) => self.toggle(v),
            ToggleRegister(r) => self.toggle(self.registers[r]),
        }
    }

    fn move_pointer(&mut self, value: Value) {
        let next = self.pointer as Value + value - 1;
        if next < 0 || next as usize >= self.instructions.len() {
            return;
        }
        self.pointer = next as usize;
    }

    fn toggle(&mut self, value: Value) {
        let inst = self.pointer as Value + value;
        if inst < 0 || inst as usize >= self.instructions.len() {
            return;
        }
        self.instructions[inst as usize] = self.instructions[inst as usize].toggle();
    }
}

#[derive(Clone)]
enum Instruction {
    CopyRegisterRegister(Register, Register),
    CopyRegisterValue(Register, Value),
    CopyValueRegister(Value, Register),
    CopyValueValue(Value, Value),
    IncrementRegister(Register),
    IncrementValue(Value),
    DecrementRegister(Register),
    DecrementValue(Value),
    JumpNotZeroRegisterRegister(Register, Register),
    JumpNotZeroRegisterValue(Register, Value),
    JumpNotZeroValueRegister(Value, Register),
    JumpNotZeroValueValue(Value, Value),
    ToggleValue(Value),
    ToggleRegister(Register),
}

impl Instruction {
    fn toggle(&self) -> Self {
        match self {
            CopyRegisterRegister(r1, r2) => JumpNotZeroRegisterRegister(*r1, *r2),
            CopyRegisterValue(r, v) => JumpNotZeroRegisterValue(*r, *v),
            CopyValueRegister(v, r) => JumpNotZeroValueRegister(*v, *r),
            CopyValueValue(v1, v2) => JumpNotZeroValueValue(*v1, *v2),
            IncrementRegister(r) => DecrementRegister(*r),
            IncrementValue(v) => DecrementValue(*v),
            DecrementRegister(r) => IncrementRegister(*r),
            DecrementValue(v) => IncrementValue(*v),
            JumpNotZeroRegisterRegister(r1, r2) => CopyRegisterRegister(*r1, *r2),
            JumpNotZeroRegisterValue(r, v) => CopyRegisterValue(*r, *v),
            JumpNotZeroValueRegister(v, r) => CopyValueRegister(*v, *r),
            JumpNotZeroValueValue(v1, v2) => CopyValueValue(*v1, *v2),
            ToggleValue(v) => IncrementValue(*v),
            ToggleRegister(r) => IncrementRegister(*r),
        }
    }
}
