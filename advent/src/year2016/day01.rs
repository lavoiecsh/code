use std::collections::HashSet;

use crate::solver::AdventSolver;

pub struct Advent2016Day01Solver {
    instructions: Vec<Instruction>,
}

impl Advent2016Day01Solver {
    pub fn new(input: String) -> Self {
        Self {
            instructions: input
                .trim()
                .split(", ")
                .map(|i| {
                    let mut chars = i.chars();
                    Instruction { direction: chars.next().unwrap(), distance: chars.collect::<String>().parse().unwrap() }
                })
                .collect()
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: char,
    distance: isize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0, direction: Direction::North }
    }

    fn execute(&self, instruction: &Instruction) -> Self {
        let direction = if instruction.direction == 'L' { self.direction.turn_left() } else { self.direction.turn_right() };
        match direction {
            Direction::North => Self { x: self.x, y: self.y - instruction.distance, direction },
            Direction::East => Self { x: self.x + instruction.distance, y: self.y, direction },
            Direction::South => Self { x: self.x, y: self.y + instruction.distance, direction },
            Direction::West => Self { x: self.x - instruction.distance, y: self.y, direction },
        }
    }

    fn distance_from_origin(&self) -> usize {
        let abs = |i| if i < 0 { (i * -1) as usize } else { i as usize };
        abs(self.x) + abs(self.y)
    }

    fn seen_positions(&self, previous: &Self) -> Vec<(isize, isize)> {
        if self.x == previous.x {
            if self.y < previous.y { self.y..previous.y } else { (previous.y + 1)..(self.y + 1) }
                .map(|y| (self.x, y))
                .collect()
        } else {
            if self.x < previous.x { self.x..previous.x } else { (previous.x + 1)..(self.x + 1) }
                .map(|x| (x, self.y))
                .collect()
        }
    }
}

impl AdventSolver for Advent2016Day01Solver {
    fn solve_part1(&self) -> usize {
        self.instructions
            .iter()
            .fold(Position::new(), |acc: Position, cur| acc.execute(cur))
            .distance_from_origin()
    }

    fn solve_part2(&self) -> usize {
        let mut seen_positions: HashSet<(isize, isize)> = HashSet::new();
        let mut position = Position::new();
        seen_positions.insert((position.x, position.y));
        for instruction in &self.instructions {
            let next_position = position.execute(&instruction);
            let next_seen_positions = next_position.seen_positions(&position);
            for nsp in next_seen_positions {
                if !seen_positions.insert(nsp) {
                    return Position { x: nsp.0, y: nsp.1, direction: Direction::North }.distance_from_origin();
                }
            }
            position = next_position;
        }
        0
    }
}
