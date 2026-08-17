use std::cmp::Ordering;
use crate::solver::AdventSolver;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter, Write};

type Value = i64;
type Pos = (Value, Value);
pub struct Advent2019Day13Solver {
    program: Vec<Value>,
}

impl Advent2019Day13Solver {
    pub fn new(input: &str) -> Self {
        Self {
            program: input.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }
}

impl AdventSolver for Advent2019Day13Solver {
    fn solve_part1(&self) -> usize {
        let computer = Computer::new(self.program.clone());
        let mut game = Game::new(computer);
        game.play();
        game.count_blocks()
    }

    fn solve_part2(&self) -> usize {
        let mut computer = Computer::new(self.program.clone());
        computer.program[0] = 2;
        let mut game = Game::new(computer);
        game.play();
        game.score as usize
    }
}

struct Game {
    computer: Computer,
    tiles: HashMap<Pos, Tile>,
    score: Value,
    ball: Pos,
    paddle: Pos,
}

enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Game {
    fn new(computer: Computer) -> Self {
        Self {
            computer,
            score: 0,
            tiles: HashMap::new(),
            ball: (0, 0),
            paddle: (0, 0),
        }
    }

    fn play(&mut self) {
        while !matches!(self.computer.state, ComputerState::Halted) {
            self.computer.run();
            while self.computer.output.len() > 2 {
                let x = self.computer.receive_output();
                let y = self.computer.receive_output();
                if x == -1 && y == 0 {
                    self.score = self.computer.receive_output();
                } else {
                    let t = Tile::from(self.computer.receive_output());
                    match t {
                        Tile::Ball => { self.ball = (x, y); }
                        Tile::Paddle => { self.paddle = (x, y); }
                        _ => {}
                    }
                    self.tiles.insert((x, y), t);
                }
            }
            if matches!(self.computer.state, ComputerState::Waiting) {
                self.computer.send_input(match self.paddle.0.cmp(&self.ball.0) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                });
            }
        }
    }

    fn count_blocks(&self) -> usize {
        self.tiles.values()
            .filter(|v| matches!(v, Tile::Block))
            .count()
    }
}

impl Debug for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_x = self.tiles.keys().map(|&(x,_)| x).max().unwrap();
        let max_y = self.tiles.keys().map(|&(_,y)| y).max().unwrap();
        f.write_char('\n')?;
        for y in 0..=max_y {
            for x in 0..=max_x {
                f.write_char(self.tiles.get(&(x, y)).map_or(' ', Tile::into))?;
            }
            f.write_char('\n')?;
        }
        f.write_fmt(format_args!("score: {}", self.score))
    }
}

impl Tile {
    fn into(&self) -> char {
        match self {
            Tile::Empty => ' ',
            Tile::Block => '#',
            Tile::Ball => 'O',
            Tile::Paddle => '_',
            Tile::Wall => '|',
        }
    }
}

impl From<Value> for Tile {
    fn from(value: Value) -> Self {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => unreachable!("unknown tile {value}"),
        }
    }
}

struct Computer {
    program: Vec<Value>,
    pointer: usize,
    input: VecDeque<Value>,
    output: VecDeque<Value>,
    state: ComputerState,
    relative_base: Value,
}

enum ComputerState {
    Running,
    Waiting,
    Halted,
}

impl Computer {
    fn new(program: Vec<Value>) -> Self {
        Self {
            program,
            pointer: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: ComputerState::Running,
            relative_base: 0,
        }
    }

    fn send_input(&mut self, input: Value) {
        self.input.push_back(input);
    }

    fn read_input(&mut self) -> Value {
        self.input.pop_front().unwrap()
    }

    fn receive_output(&mut self) -> Value {
        self.output.pop_front().unwrap()
    }

    fn run(&mut self) {
        self.state = ComputerState::Running;
        while matches!(self.state, ComputerState::Running) {
            self.pointer += self.execute_operation();
        }
    }

    fn execute_operation(&mut self) -> usize {
        let opcode = self.program[self.pointer];
        let instruction = opcode % 100;
        match instruction {
            1 => {
                let a = self.get_value(1);
                let b = self.get_value(2);
                let position = self.get_address(3);
                self.set(position, a + b);
                4
            }
            2 => {
                let a = self.get_value(1);
                let b = self.get_value(2);
                let position = self.get_address(3);
                self.set(position, a * b);
                4
            }
            3 => {
                if self.input.is_empty() {
                    self.state = ComputerState::Waiting;
                    return 0;
                }
                let position = self.get_address(1);
                let value = self.read_input();
                self.set(position, value);
                2
            }
            4 => {
                let value = self.get_value(1);
                self.output.push_back(value);
                2
            }
            5 => {
                if self.get_value(1) != 0 {
                    self.pointer = self.get_value(2) as usize;
                    0
                } else {
                    3
                }
            }
            6 => {
                if self.get_value(1) == 0 {
                    self.pointer = self.get_value(2) as usize;
                    0
                } else {
                    3
                }
            }
            7 => {
                let position = self.get_address(3);
                let value = if self.get_value(1) < self.get_value(2) {
                    1
                } else {
                    0
                };
                self.set(position, value);
                4
            }
            8 => {
                let position = self.get_address(3);
                let value = if self.get_value(1) == self.get_value(2) {
                    1
                } else {
                    0
                };
                self.set(position, value);
                4
            }
            9 => {
                let value = self.get_value(1);
                self.relative_base += value;
                2
            }
            99 => {
                self.state = ComputerState::Halted;
                1
            }
            _ => unreachable!("unknown instruction {instruction}"),
        }
    }

    fn set(&mut self, position: usize, value: Value) {
        if position < self.program.len() {
            self.program[position] = value;
        } else {
            self.program.extend(vec![0; position - self.program.len()]);
            self.program.push(value);
        }
    }

    fn get_value(&self, offset: usize) -> Value {
        let div = (10 as Value).pow(offset as u32 + 1);
        let mode = (self.program[self.pointer] / div) % 10;
        let value = self.program[self.pointer + offset];
        match mode {
            0 => {
                let address = value as usize;
                if address >= self.program.len() {
                    0
                } else {
                    self.program[address]
                }
            }
            1 => value,
            2 => {
                let address = (value + self.relative_base) as usize;
                if address >= self.program.len() {
                    0
                } else {
                    self.program[address]
                }
            }
            _ => unreachable!("unknown mode {mode}"),
        }
    }

    fn get_address(&self, offset: u32) -> usize {
        let div = (10 as Value).pow(offset + 1);
        let mode = (self.program[self.pointer] / div) % 10;
        let value = self.program[self.pointer + offset as usize];
        match mode {
            0 => value as usize,
            1 => unreachable!("cannot get address for immediate mode"),
            2 => (value + self.relative_base) as usize,
            _ => unreachable!("unknown mode {mode}"),
        }
    }
}
