use crate::solver::AdventSolver;
use crate::year2019::intcode::{Computer, Value};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Write};

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
        let mut program = self.program.clone();
        program[0] = 2;
        let computer = Computer::new(program);
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
        while self.computer.is_running() {
            self.computer.run();
            while let Some(x) = self.computer.receive_output() {
                let y = self.computer.receive_output().unwrap();
                let z = self.computer.receive_output().unwrap();
                if x == -1 && y == 0 {
                    self.score = z;
                } else {
                    let t = Tile::from(z);
                    match t {
                        Tile::Ball => { self.ball = (x, y); }
                        Tile::Paddle => { self.paddle = (x, y); }
                        _ => {}
                    }
                    self.tiles.insert((x, y), t);
                }
            }
            if self.computer.is_waiting() {
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
