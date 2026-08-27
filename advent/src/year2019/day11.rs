use crate::solver::AdventSolver;
use crate::year2019::intcode::{Computer, Value};
use itertools::Itertools;
use std::collections::HashMap;

type Pos = (Value, Value);
pub struct Advent2019Day11Solver {
    program: Vec<Value>,
}

impl Advent2019Day11Solver {
    pub fn new(input: &str) -> Self {
        Self {
            program: input.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }
}

impl AdventSolver for Advent2019Day11Solver {
    fn solve_part1(&self) -> usize {
        let computer = Computer::new(self.program.clone());
        let mut robot = Robot::new(computer);
        robot.run();
        robot.grid.len()
    }

    fn solve_part2_string(&self) -> String {
        let computer = Computer::new(self.program.clone());
        let mut robot = Robot::new(computer);
        robot.grid.insert(robot.pos, true);
        robot.run();
        let (min_x, max_x) = robot
            .grid
            .keys()
            .map(|&(x, _)| x)
            .minmax()
            .into_option()
            .unwrap();
        let (min_y, max_y) = robot
            .grid
            .keys()
            .map(|&(_, y)| y)
            .minmax()
            .into_option()
            .unwrap();
        let mut registration = String::new();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                registration.push(
                    robot
                        .grid
                        .get(&(x, y))
                        .map_or(' ', |&v| if v { '#' } else { ' ' }),
                );
            }
            registration.push('\n');
        }
        registration
    }
}

struct Robot {
    computer: Computer,
    grid: HashMap<Pos, bool>,
    pos: Pos,
    direction: Direction,
}

impl Robot {
    fn new(computer: Computer) -> Self {
        Self {
            computer,
            grid: HashMap::new(),
            pos: (0, 0),
            direction: Direction::Up,
        }
    }

    fn run(&mut self) {
        while self.computer.is_running() {
            if let Some(&value) = self.grid.get(&self.pos) {
                self.computer.send_input(if value { 1 } else { 0 });
            } else {
                self.computer.send_input(0);
            }
            self.computer.run();
            let new_color = self.computer.receive_output().unwrap();
            self.grid.insert(self.pos, new_color == 1);
            let turn = self.computer.receive_output().unwrap();
            if turn == 0 {
                self.turn_left();
            } else {
                self.turn_right();
            }
            self.move_forward();
        }
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => self.pos.1 -= 1,
            Direction::Right => self.pos.0 += 1,
            Direction::Down => self.pos.1 += 1,
            Direction::Left => self.pos.0 -= 1,
        }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}
