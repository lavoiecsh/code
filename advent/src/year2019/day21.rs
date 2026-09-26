use crate::solver::AdventSolver;
use crate::year2019::intcode::{Computer, Value, parse_program};

pub struct Advent2019Day21Solver {
    program: Vec<Value>,
}

impl Advent2019Day21Solver {
    pub fn new(input: &str) -> Self {
        Self {
            program: parse_program(input),
        }
    }
}

impl AdventSolver for Advent2019Day21Solver {
    fn solve_part1(&self) -> usize {
        let mut droid = SpringDroid::new(Computer::new(self.program.clone()));
        droid.walk()
    }

    fn solve_part2(&self) -> usize {
        let mut droid = SpringDroid::new(Computer::new(self.program.clone()));
        droid.run()
    }
}

struct SpringDroid {
    computer: Computer,
}

impl SpringDroid {
    fn new(computer: Computer) -> Self {
        Self {
            computer,
        }
    }

    fn walk(&mut self) -> usize {
        // reset J
        self.computer.send_ascii_instruction("NOT J T");
        self.computer.send_ascii_instruction("AND T J");
        // hole in the next three
        self.computer.send_ascii_instruction("AND A T");
        self.computer.send_ascii_instruction("AND B T");
        self.computer.send_ascii_instruction("AND C T");
        self.computer.send_ascii_instruction("NOT T J");
        // solid on the 4th
        self.computer.send_ascii_instruction("AND D J");
        // go
        self.computer.send_ascii_instruction("WALK");
        self.computer.run_script(false).unwrap() as usize
    }

    fn run(&mut self) -> usize {
        // reset J
        self.computer.send_ascii_instruction("NOT J T");
        self.computer.send_ascii_instruction("AND T J");
        // hole in the next three
        self.computer.send_ascii_instruction("AND A T");
        self.computer.send_ascii_instruction("AND B T");
        self.computer.send_ascii_instruction("AND C T");
        self.computer.send_ascii_instruction("NOT T J");
        // solid on the 4th
        self.computer.send_ascii_instruction("AND D J");
        // and other solid after
        self.computer.send_ascii_instruction("OR E T");
        self.computer.send_ascii_instruction("OR H T");
        self.computer.send_ascii_instruction("AND T J");
        // go
        self.computer.send_ascii_instruction("RUN");
        self.computer.run_script(false).unwrap() as usize
    }
}