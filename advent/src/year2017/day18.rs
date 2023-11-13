use std::collections::{HashMap, VecDeque};
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};

use crate::solver::AdventSolver;
use crate::year2017::day18::Instruction::*;

pub struct Advent2017Day18Solver {
    instructions: Vec<Instruction>,
}

impl Advent2017Day18Solver {
    pub fn new(input: String) -> Self {
        Self {
            instructions: input.lines()
                .map(|l| Instruction::from(l.split(" ").collect()))
                .collect()
        }
    }
}

impl AdventSolver for Advent2017Day18Solver {
    fn solve_part1(&self) -> usize {
        let (tx0, _rx0) = sync_channel(10000);
        let (_tx1, rx1) = sync_channel(10000);
        let mut computer = Computer::new(&self.instructions, tx0, rx1, 0);
        computer.run();
        computer.sent.pop_back().unwrap() as usize
    }

    fn solve_part2(&self) -> usize {
        let (tx0, rx0) = sync_channel(10000);
        let (tx1, rx1) = sync_channel(10000);
        let mut computer_0 = Computer::new(&self.instructions, tx0, rx1, 0);
        let mut computer_1 = Computer::new(&self.instructions, tx1, rx0, 1);
        let mut prev_0 = computer_0.sent.len();
        let mut prev_1 = computer_1.sent.len();
        while computer_0.is_running()
            || computer_1.is_running()
            || computer_0.sent.len() != prev_0
            || computer_1.sent.len() != prev_1
        {
            prev_0 = computer_0.sent.len();
            prev_1 = computer_1.sent.len();

            computer_0.run();
            computer_1.run();
        }
        computer_1.sent.len()
    }
}

type Value = isize;

struct Computer<'a> {
    instructions: &'a Vec<Instruction>,
    pointer: usize,
    registers: HashMap<char, Value>,
    tx: SyncSender<Value>,
    rx: Receiver<Value>,
    is_waiting: bool,
    sent: VecDeque<Value>,
}

impl<'a> Computer<'a> {
    fn new(instructions: &'a Vec<Instruction>, tx: SyncSender<Value>, rx: Receiver<Value>, p: Value) -> Self {
        let mut registers = HashMap::new();
        registers.insert('p', p);
        Self { instructions, pointer: 0, registers, tx, rx, is_waiting: false, sent: VecDeque::new() }
    }

    fn run(&mut self) {
        self.is_waiting = false;
        while !self.is_done() && !self.is_waiting() {
            self.execute();
            if !self.is_waiting {
                self.pointer += 1;
            }
        }
    }

    fn execute(&mut self) {
        match &self.instructions[self.pointer] {
            Send(d) => {
                let value = self.value(d);
                self.tx.send(value).unwrap();
                self.sent.push_back(value);
            }
            Set(d, v) => {
                self.set(d, v);
            }
            Add(d, v) => {
                let value = self.value(d) + self.value(v);
                self.set(d, &Destination::Value(value));
            }
            Mul(d, v) => {
                let value = self.value(d) * self.value(v);
                self.set(d, &Destination::Value(value));
            }
            Mod(d, v) => {
                let value = self.value(d) % self.value(v);
                self.set(d, &Destination::Value(value));
            }
            Receive(d) => {
                match self.rx.try_recv() {
                    Ok(value) => {
                        self.is_waiting = false;
                        self.set(d, &Destination::Value(value));
                    }
                    Err(_) => {
                        self.is_waiting = true;
                    }
                }
            }
            JumpGreaterZero(v, o) => {
                if self.value(v) <= 0 { return; }

                let offset = self.value(o);
                if offset < 0 {
                    self.pointer -= (offset * -1) as usize;
                } else {
                    self.pointer += offset as usize;
                }
                self.pointer -= 1;
            }
        }
    }

    fn is_done(&self) -> bool {
        self.pointer >= self.instructions.len()
    }

    fn is_waiting(&self) -> bool {
        self.is_waiting
    }

    fn is_running(&self) -> bool {
        !self.is_done() && !self.is_waiting()
    }

    fn value(&self, destination: &Destination) -> Value {
        match destination {
            Destination::Register(r) => *self.registers.get(r).unwrap_or(&0),
            Destination::Value(v) => *v,
        }
    }

    fn set(&mut self, destination: &Destination, value: &Destination) {
        match destination {
            Destination::Value(_) => {}
            Destination::Register(d) => {
                let v = self.value(value);
                self.registers.insert(*d, v);
            }
        }
    }
}

enum Instruction {
    Send(Destination),
    Set(Destination, Destination),
    Add(Destination, Destination),
    Mul(Destination, Destination),
    Mod(Destination, Destination),
    Receive(Destination),
    JumpGreaterZero(Destination, Destination),
}

impl Instruction {
    fn from(input: Vec<&str>) -> Self {
        match input[0] {
            "snd" => Send(Destination::from(input[1])),
            "set" => Set(Destination::from(input[1]), Destination::from(input[2])),
            "add" => Add(Destination::from(input[1]), Destination::from(input[2])),
            "mul" => Mul(Destination::from(input[1]), Destination::from(input[2])),
            "mod" => Mod(Destination::from(input[1]), Destination::from(input[2])),
            "rcv" => Receive(Destination::from(input[1])),
            "jgz" => JumpGreaterZero(Destination::from(input[1]), Destination::from(input[2])),
            _ => panic!("unknown instruction"),
        }
    }
}

enum Destination {
    Value(Value),
    Register(char),
}

impl Destination {
    fn from(input: &str) -> Self {
        input.parse::<Value>()
            .map_or_else(
                |_| Destination::Register(input.chars().next().unwrap()),
                |v| Destination::Value(v),
            )
    }
}
