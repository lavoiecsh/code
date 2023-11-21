use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use itertools::Itertools;

use Direction::*;

use crate::solver::AdventSolver;

pub struct Advent2017Day22Solver {
    grid: Vec<Vec<bool>>,
}

impl Advent2017Day22Solver {
    pub fn new(input: String) -> Self {
        Self {
            grid: input.lines()
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect()
        }
    }
}

impl AdventSolver for Advent2017Day22Solver {
    fn solve_part1(&self) -> usize {
        let mut carrier: Carrier<BiState> = Carrier::new(&self.grid);
        (0..10000).for_each(|_| carrier.iterate());
        carrier.infected_count
    }

    fn solve_part2(&self) -> usize {
        let mut carrier: Carrier<QuadState> = Carrier::new(&self.grid);
        (0..10000000).for_each(|_| carrier.iterate());
        carrier.infected_count
    }
}

type Pos = (i32, i32);

trait GridState {
    fn clean() -> Self;
    fn infected() -> Self;
    fn is_infected(&self) -> bool;
    fn next_state(&self) -> Self;
    fn next_direction(&self, direction: &Direction) -> Direction;
    fn dbg(&self) -> char;
}

enum BiState {
    Clean,
    Infected,
}

impl GridState for BiState {
    fn clean() -> Self {
        BiState::Clean
    }

    fn infected() -> Self {
        BiState::Infected
    }

    fn is_infected(&self) -> bool {
        match self {
            BiState::Infected => true,
            _ => false,
        }
    }

    fn next_state(&self) -> Self {
        match self {
            BiState::Clean => BiState::Infected,
            BiState::Infected => BiState::Clean,
        }
    }

    fn next_direction(&self, direction: &Direction) -> Direction {
        match self {
            BiState::Clean => direction.left(),
            BiState::Infected => direction.right(),
        }
    }

    fn dbg(&self) -> char {
        match self {
            BiState::Clean => '.',
            BiState::Infected => '#',
        }
    }
}

enum QuadState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl GridState for QuadState {
    fn clean() -> Self {
        QuadState::Clean
    }

    fn infected() -> Self {
        QuadState::Infected
    }

    fn is_infected(&self) -> bool {
        match self {
            QuadState::Infected => true,
            _ => false,
        }
    }

    fn next_state(&self) -> Self {
        match self {
            QuadState::Clean => QuadState::Weakened,
            QuadState::Weakened => QuadState::Infected,
            QuadState::Infected => QuadState::Flagged,
            QuadState::Flagged => QuadState::Clean,
        }
    }

    fn next_direction(&self, direction: &Direction) -> Direction {
        match self {
            QuadState::Clean => direction.left(),
            QuadState::Weakened => direction.same(),
            QuadState::Infected => direction.right(),
            QuadState::Flagged => direction.reverse(),
        }
    }

    fn dbg(&self) -> char {
        match self {
            QuadState::Clean => '.',
            QuadState::Weakened => 'W',
            QuadState::Infected => '#',
            QuadState::Flagged => 'F',
        }
    }
}

struct Carrier<State: GridState> {
    states: HashMap<Pos, State>,
    pos: Pos,
    direction: Direction,
    infected_count: usize,
}

impl<State: GridState> Carrier<State> {
    fn new(grid: &Vec<Vec<bool>>) -> Self {
        let mut states = HashMap::new();
        let half_size: i32 = (grid.len() as i32) / 2;
        for row in 0..grid.len() {
            for col in 0..grid.len() {
                if grid[row][col] {
                    states.insert((col as i32 - half_size, row as i32 - half_size), State::infected());
                }
            }
        }
        Self {
            states,
            pos: (0, 0),
            direction: Up,
            infected_count: 0,
        }
    }

    fn iterate(&mut self) {
        self.update_direction();
        self.update_state();
        self.update_position();
    }

    fn update_direction(&mut self) {
        self.direction = self.states.get(&self.pos).unwrap_or(&State::clean())
            .next_direction(&self.direction);
    }

    fn update_state(&mut self) {
        let state = self.states.entry(self.pos).or_insert_with(State::clean);
        *state = state.next_state();
        if state.is_infected() {
            self.infected_count += 1;
        }
    }

    fn update_position(&mut self) {
        self.pos = match self.direction {
            Up => (self.pos.0, self.pos.1 - 1),
            Right => (self.pos.0 + 1, self.pos.1),
            Down => (self.pos.0, self.pos.1 + 1),
            Left => (self.pos.0 - 1, self.pos.1),
        };
    }
}

impl<State: GridState> Debug for Carrier<State> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let xs: Vec<i32> = self.states.iter().filter(|(_, s)| s.is_infected()).map(|(p, _)| p.0).collect();
        let ys: Vec<i32> = self.states.iter().filter(|(_, s)| s.is_infected()).map(|(p, _)| p.1).collect();
        let x_bounds = xs.iter().minmax().into_option().unwrap();
        let y_bounds = ys.iter().minmax().into_option().unwrap();
        let mut map: Vec<String> = Vec::new();
        for y in *y_bounds.0..=*y_bounds.1 {
            let mut row: Vec<char> = Vec::new();
            for x in *x_bounds.0..=*x_bounds.1 {
                row.push(self.states.get(&(x, y)).unwrap_or(&State::clean()).dbg());
            }
            map.push(row.iter().join(""));
        }
        f.write_fmt(format_args!("({},{})\n{}", self.pos.0, self.pos.1, &map.iter().join("\n")))
    }
}

enum Direction { Up, Right, Down, Left }

impl Direction {
    fn right(&self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn left(&self) -> Self {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn same(&self) -> Self {
        match self {
            Up => Up,
            Left => Left,
            Down => Down,
            Right => Right,
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right,
        }
    }
}
