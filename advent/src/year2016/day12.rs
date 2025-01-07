use crate::solver::AdventSolver;

pub struct Advent2016Day12Solver {
    instructions: Vec<Instruction>,
}

impl Advent2016Day12Solver {
    pub fn new(input: &str) -> Self {
        let to_index = |r: &str| r.chars().next().unwrap() as usize - 'a' as usize;
        Self {
            instructions: input
                .lines()
                .map(|l| {
                    let s = l.split(' ').collect::<Vec<&str>>();
                    match s[0] {
                        "cpy" => s[1].parse().map_or_else(
                            |_| Instruction::CopyRegister(to_index(s[1]), to_index(s[2])),
                            |v| Instruction::CopyValue(v, to_index(s[2])),
                        ),
                        "inc" => Instruction::Increment(to_index(s[1])),
                        "dec" => Instruction::Decrement(to_index(s[1])),
                        "jnz" => s[1].parse().map_or_else(
                            |_| {
                                Instruction::JumpNotZeroRegister(
                                    to_index(s[1]),
                                    s[2].parse().unwrap(),
                                )
                            },
                            |v| Instruction::JumpNotZeroValue(v, s[2].parse().unwrap()),
                        ),
                        i => panic!("unknown instruction {i} in line {l}"),
                    }
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2016Day12Solver {
    fn solve_part1(&self) -> usize {
        let mut computer = Computer::new(&self.instructions);
        computer.run();
        computer.registers[0] as usize
    }

    fn solve_part2(&self) -> usize {
        let mut computer = Computer::new(&self.instructions);
        computer.registers[2] = 1;
        computer.run();
        computer.registers[0] as usize
    }
}

struct Computer<'a> {
    registers: [i32; 4],
    pointer: i32,
    instructions: &'a Vec<Instruction>,
}

impl<'a> Computer<'a> {
    fn new(instructions: &'a Vec<Instruction>) -> Self {
        Self {
            registers: [0; 4],
            pointer: 0,
            instructions,
        }
    }

    fn run(&mut self) {
        while !self.is_completed() {
            self.execute();
            self.pointer += 1;
        }
    }

    fn is_completed(&self) -> bool {
        self.pointer as usize >= self.instructions.len()
    }

    fn execute(&mut self) {
        match self.instructions[self.pointer as usize] {
            Instruction::CopyValue(v, o) => self.registers[o] = v,
            Instruction::CopyRegister(i, o) => self.registers[o] = self.registers[i],
            Instruction::Increment(r) => self.registers[r] += 1,
            Instruction::Decrement(r) => self.registers[r] -= 1,
            Instruction::JumpNotZeroRegister(r, p) => {
                if self.registers[r] != 0 {
                    self.pointer += p - 1;
                }
            }
            Instruction::JumpNotZeroValue(v, p) => {
                if v != 0 {
                    self.pointer += p - 1;
                }
            }
        };
    }
}

enum Instruction {
    CopyValue(i32, usize),
    CopyRegister(usize, usize),
    Increment(usize),
    Decrement(usize),
    JumpNotZeroRegister(usize, i32),
    JumpNotZeroValue(i32, i32),
}
