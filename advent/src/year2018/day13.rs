use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2018Day13Solver {
    map: Map,
}

impl Advent2018Day13Solver {
    pub fn new(input: &str) -> Self {
        let lines: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut carts = Vec::new();
        for y in 0..lines.len() {
            let mut row = Vec::new();
            for x in 0..lines[y].len() {
                row.push(match lines[y][x] {
                    '<' => {
                        carts.push(Cart::new((x, y), Direction::Left));
                        '-'
                    }
                    '>' => {
                        carts.push(Cart::new((x, y), Direction::Right));
                        '-'
                    }
                    '^' => {
                        carts.push(Cart::new((x, y), Direction::Up));
                        '|'
                    }
                    'v' => {
                        carts.push(Cart::new((x, y), Direction::Down));
                        '|'
                    }
                    c => c,
                });
            }
            grid.push(row);
        }
        let mut map = Map::new(grid, carts);
        map.run();
        Self { map }
    }
}

impl AdventSolver for Advent2018Day13Solver {
    fn solve_part1_string(&self) -> String {
        let crash = &self.map.crashes[0];
        format!("{},{}", crash.0, crash.1)
    }

    fn solve_part2_string(&self) -> String {
        let cart = &self.map.carts[0];
        format!("{},{}", cart.position.0, cart.position.1)
    }
}

struct Map {
    grid: Vec<Vec<char>>,
    carts: Vec<Cart>,
    crashes: Vec<Pos>,
}

impl Map {
    fn new(grid: Vec<Vec<char>>, carts: Vec<Cart>) -> Self {
        Self {
            grid,
            carts,
            crashes: Vec::new(),
        }
    }

    fn get(&self, position: &Pos) -> char {
        self.grid[position.1][position.0]
    }

    fn run(&mut self) {
        while self.carts.len() > 1 {
            self.iterate();
        }
    }

    fn iterate(&mut self) {
        let ordered: Vec<Cart> = self
            .carts
            .iter()
            .sorted_by(|l, r| {
                l.position
                    .1
                    .cmp(&r.position.1)
                    .then(l.position.0.cmp(&r.position.0))
            })
            .cloned()
            .collect();
        self.carts.clear();
        let mut skip: Vec<usize> = Vec::new();
        for i in 0..ordered.len() {
            if skip.contains(&i) {
                continue;
            }
            let mut crash = false;
            let cart = ordered[i].next(self);
            for j in 0..self.carts.len() {
                if self.carts[j].position == cart.position {
                    crash = true;
                    self.carts.remove(j);
                }
            }
            for j in i + 1..ordered.len() {
                if ordered[j].position == cart.position {
                    crash = true;
                    skip.push(j);
                }
            }
            if crash {
                self.crashes.push(cart.position);
            } else {
                self.carts.push(cart);
            }
        }
    }
}

type Pos = (usize, usize);

#[derive(Clone, Debug)]
struct Cart {
    position: Pos,
    direction: Direction,
    next_intersection: IntersectionTurn,
}

impl Cart {
    fn new(position: Pos, direction: Direction) -> Self {
        Self {
            position,
            direction,
            next_intersection: IntersectionTurn::Left,
        }
    }

    fn next(&self, map: &Map) -> Self {
        let position = match self.direction {
            Direction::Up => (self.position.0, self.position.1 - 1),
            Direction::Down => (self.position.0, self.position.1 + 1),
            Direction::Left => (self.position.0 - 1, self.position.1),
            Direction::Right => (self.position.0 + 1, self.position.1),
        };
        let (direction, next_intersection) = match map.get(&position) {
            '-' | '|' => (self.direction.clone(), self.next_intersection.clone()),
            '\\' => (self.direction.backslash(), self.next_intersection.clone()),
            '/' => (self.direction.slash(), self.next_intersection.clone()),
            '+' => (
                self.direction.intersection(&self.next_intersection),
                self.next_intersection.next(),
            ),
            c => panic!("unknown map character '{c}'"),
        };
        Self {
            position,
            direction,
            next_intersection,
        }
    }
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn slash(&self) -> Self {
        match self {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn backslash(&self) -> Self {
        match self {
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn intersection(&self, intersection: &IntersectionTurn) -> Self {
        match (self, intersection) {
            (d, IntersectionTurn::Straight) => d.clone(),
            (Direction::Down, IntersectionTurn::Left) => Direction::Right,
            (Direction::Down, IntersectionTurn::Right) => Direction::Left,
            (Direction::Up, IntersectionTurn::Left) => Direction::Left,
            (Direction::Up, IntersectionTurn::Right) => Direction::Right,
            (Direction::Left, IntersectionTurn::Left) => Direction::Down,
            (Direction::Left, IntersectionTurn::Right) => Direction::Up,
            (Direction::Right, IntersectionTurn::Left) => Direction::Up,
            (Direction::Right, IntersectionTurn::Right) => Direction::Down,
        }
    }
}

#[derive(Clone, Debug)]
enum IntersectionTurn {
    Left,
    Straight,
    Right,
}

impl IntersectionTurn {
    fn next(&self) -> Self {
        match self {
            IntersectionTurn::Left => IntersectionTurn::Straight,
            IntersectionTurn::Straight => IntersectionTurn::Right,
            IntersectionTurn::Right => IntersectionTurn::Left,
        }
    }
}
