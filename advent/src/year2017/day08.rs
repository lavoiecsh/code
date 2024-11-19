use crate::solver::AdventSolver;
use regex::Regex;
use std::collections::HashMap;

pub struct Advent2017Day08Solver {
    instructions: Vec<Instruction>,
}

impl Advent2017Day08Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"(\w+) (inc|dec) (-?\d+) if (\w+) (!=|>=|==|>|<|<=) (-?\d+)").unwrap();
        Self {
            instructions: input
                .lines()
                .map(|l| re.captures(l).unwrap())
                .map(|c| Instruction {
                    register: c.get(1).unwrap().as_str().to_string(),
                    operation: match c.get(2).unwrap().as_str() {
                        "inc" => move |r, v| r + v,
                        "dec" => move |r, v| r - v,
                        _ => panic!("unknown operation"),
                    },
                    value: c.get(3).unwrap().as_str().parse().unwrap(),
                    condition: Condition {
                        register: c.get(4).unwrap().as_str().to_string(),
                        operation: match c.get(5).unwrap().as_str() {
                            "==" => move |r, v| r == v,
                            "!=" => move |r, v| r != v,
                            ">" => move |r, v| r > v,
                            ">=" => move |r, v| r >= v,
                            "<" => move |r, v| r < v,
                            "<=" => move |r, v| r <= v,
                            _ => panic!("unknown operator"),
                        },
                        value: c.get(6).unwrap().as_str().parse().unwrap(),
                    },
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2017Day08Solver {
    fn solve_part1(&self) -> usize {
        let mut computer = Computer::new();
        self.instructions.iter().for_each(|i| computer.execute(i));
        computer.highest_value() as usize
    }

    fn solve_part2(&self) -> usize {
        let mut computer = Computer::new();
        self.instructions.iter().fold(0, |highest, instruction| {
            computer.execute(instruction);
            usize::max(highest, computer.highest_value() as usize)
        })
    }
}

type Value = i32;

struct Instruction {
    register: String,
    operation: fn(Value, Value) -> Value,
    value: Value,
    condition: Condition,
}

impl Instruction {
    fn new_value(&self, register: Value) -> Value {
        (self.operation)(register, self.value)
    }
}

struct Condition {
    register: String,
    operation: fn(Value, Value) -> bool,
    value: Value,
}

impl Condition {
    fn passes(&self, register: Value) -> bool {
        (self.operation)(register, self.value)
    }

    fn fails(&self, register: Value) -> bool {
        !self.passes(register)
    }
}

struct Computer {
    registers: HashMap<String, Value>,
}

impl Computer {
    fn new() -> Self {
        Self {
            registers: HashMap::new(),
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        if instruction
            .condition
            .fails(self.get_value(&instruction.condition.register))
        {
            return;
        }

        let previous = self.get_value(&instruction.register);
        self.registers.insert(
            instruction.register.clone(),
            instruction.new_value(previous),
        );
    }

    fn get_value(&self, register: &String) -> Value {
        *self.registers.get(register).unwrap_or(&0)
    }

    fn highest_value(&self) -> Value {
        *self.registers.values().max().unwrap_or(&0)
    }
}
