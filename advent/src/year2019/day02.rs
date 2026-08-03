use crate::solver::AdventSolver;

pub struct Advent2019Day02Solver {
    program: Program,
}

impl Advent2019Day02Solver {
    pub fn new(input: &str) -> Self {
        Self {
            program: Program::from(input),
        }
    }
}

impl AdventSolver for Advent2019Day02Solver {
    fn solve_part1(&self) -> usize {
        let mut program = self.program.clone();
        program.integers[1] = 12;
        program.integers[2] = 2;
        program.execute();
        program.integers[0] as usize
    }

    fn solve_part2(&self) -> usize {
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut program = self.program.clone();
                program.integers[1] = noun;
                program.integers[2] = verb;
                program.execute();
                if program.integers[0] == 19690720 {
                    return 100usize * noun as usize + verb as usize;
                }
            }
        }
        unreachable!("no solution found")
    }
}

#[derive(Clone)]
struct Program {
    integers: Vec<u32>,
    instruction_pointer: usize,
}

impl Program {
    fn execute(&mut self) {
        while self.is_running() {
            self.execute_instruction();
            self.instruction_pointer += 4;
        }
    }

    fn is_running(&self) -> bool {
        self.integers[self.instruction_pointer] != 99
    }

    fn execute_instruction(&mut self) {
        let p = self.instruction_pointer;
        let a = self.integers[self.instruction_pointer + 1] as usize;
        let b = self.integers[self.instruction_pointer + 2] as usize;
        let c = self.integers[self.instruction_pointer + 3] as usize;
        match self.integers[p] {
            1 => self.integers[c] = self.integers[a] + self.integers[b],
            2 => self.integers[c] = self.integers[a] * self.integers[b],
            _x => unreachable!("unknown opcode {_x} at position {}", p),
        }
    }
}

impl From<&str> for Program {
    fn from(value: &str) -> Self {
        Self {
            integers: value.split(',').map(|x| x.parse().unwrap()).collect(),
            instruction_pointer: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "1,9,10,3,2,3,11,0,99,30,40,50";

    #[test]
    fn executes_simple_programs() {
        let mut program_1 = Program::from(EXAMPLE_1);
        program_1.execute();
        assert_eq!(program_1.integers[3], 70);
        assert_eq!(program_1.integers[0], 3500);
    }
}