use crate::solver::AdventSolver;
use std::collections::HashMap;

pub struct Advent2017Day25Solver {
    start_state: char,
    steps: usize,
    states: HashMap<char, State>,
}

impl Advent2017Day25Solver {
    pub fn new(input: &str) -> Self {
        let mut lines = input.lines();
        let start_state = lines.next().unwrap().chars().nth(15).unwrap();
        let steps = lines
            .next()
            .unwrap()
            .split(" ")
            .nth(5)
            .unwrap()
            .parse()
            .unwrap();
        let mut states = HashMap::new();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                continue;
            }
            if line.starts_with("In state ") {
                let name = line.chars().nth(9).unwrap();
                lines.next();
                let false_write = lines.next().unwrap().chars().nth(22).unwrap() == '1';
                let false_right = lines.next().unwrap().split(" ").last().unwrap() == "right.";
                let false_state = lines.next().unwrap().chars().nth(26).unwrap();
                lines.next();
                let true_write = lines.next().unwrap().chars().nth(22).unwrap() == '1';
                let true_right = lines.next().unwrap().split(" ").last().unwrap() == "right.";
                let true_state = lines.next().unwrap().chars().nth(26).unwrap();
                states.insert(
                    name,
                    State {
                        false_write,
                        false_move: if false_right { 1 } else { -1 },
                        false_state,
                        true_write,
                        true_move: if true_right { 1 } else { -1 },
                        true_state,
                    },
                );
            }
        }

        Self {
            start_state,
            steps,
            states,
        }
    }
}

impl AdventSolver for Advent2017Day25Solver {
    fn solve_part1(&self) -> usize {
        let mut machine = TuringMachine::new(&self.states, self.start_state);
        machine.run(self.steps);
        machine.diagnostic_checksum()
    }
}

struct TuringMachine<'a> {
    states: &'a HashMap<char, State>,
    ribbon: HashMap<i64, bool>,
    cursor: i64,
    state: char,
}

impl<'a> TuringMachine<'a> {
    fn new(states: &'a HashMap<char, State>, starting_state: char) -> Self {
        Self {
            states,
            ribbon: HashMap::new(),
            cursor: 0,
            state: starting_state,
        }
    }

    fn run(&mut self, steps: usize) {
        (0..steps).for_each(|_| self.step());
    }

    fn step(&mut self) {
        let current_state = self.states.get(&self.state).unwrap();
        if *self.ribbon.get(&self.cursor).unwrap_or(&false) {
            self.ribbon.insert(self.cursor, current_state.true_write);
            self.cursor += current_state.true_move;
            self.state = current_state.true_state;
        } else {
            self.ribbon.insert(self.cursor, current_state.false_write);
            self.cursor += current_state.false_move;
            self.state = current_state.false_state;
        }
    }

    fn diagnostic_checksum(&self) -> usize {
        self.ribbon.values().filter(|v| **v).count()
    }
}

#[derive(Debug)]
struct State {
    false_write: bool,
    false_move: i64,
    false_state: char,
    true_write: bool,
    true_move: i64,
    true_state: char,
}
