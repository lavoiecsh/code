use num_traits::abs;
use crate::solver::AdventSolver;

pub struct Advent2017Day11Solver {
    path: Vec<Direction>,
}

impl Advent2017Day11Solver {
    pub fn new(input: String) -> Self {
        Self { path: input.split(",").map(Direction::from).collect() }
    }
}

impl AdventSolver for Advent2017Day11Solver {
    fn solve_part1(&self) -> usize {
        let mut process = Process::new();
        self.path.iter().for_each(|s| process.step(s));
        process.distance()
    }

    fn solve_part2(&self) -> usize {
        let mut process = Process::new();
        self.path.iter()
            .map(|s| { process.step(s); process.distance() })
            .max()
            .unwrap()
    }
}

struct Process {
    x: i32,
    y: i32,
}

impl Process {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::North => { self.y -= 1 },
            Direction::South => { self.y += 1 },
            Direction::NorthEast => { self.y -= 1; self.x += 1; },
            Direction::SouthWest => { self.y += 1; self.x -= 1; },
            Direction::NorthWest => { self.x -= 1 },
            Direction::SouthEast => { self.x += 1 },
        }
    }

    fn distance(&self) -> usize {
        let x = abs(self.x) as usize;
        let y = abs(self.y) as usize;
        usize::min(x,y) + (usize::max(x,y) - usize::min(x,y))
    }
}

enum Direction {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
}

impl Direction {
    fn from(direction: &str) -> Self {
        match direction {
            "n" => Direction::North,
            "ne" => Direction::NorthEast,
            "nw" => Direction::NorthWest,
            "s" => Direction::South,
            "se" => Direction::SouthEast,
            "sw" => Direction::SouthWest,
            _ => panic!("unknown direction"),
        }
    }
}
