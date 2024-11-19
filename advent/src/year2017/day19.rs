use crate::solver::AdventSolver;

pub struct Advent2017Day19Solver {
    map: Map,
}

impl Advent2017Day19Solver {
    pub fn new(input: &str) -> Self {
        Self {
            map: Map::new(input.lines().map(|l| l.chars().collect()).collect()),
        }
    }
}

impl AdventSolver for Advent2017Day19Solver {
    fn solve_part1_string(&self) -> String {
        let mut packet = Packet::new(&self.map);
        packet.run();
        packet.seen.iter().collect()
    }

    fn solve_part2(&self) -> usize {
        let mut packet = Packet::new(&self.map);
        packet.run();
        packet.steps + 1
    }
}

struct Packet<'a> {
    map: &'a Map,
    seen: Vec<char>,
    pos: Pos,
    steps: usize,
}

impl<'a> Packet<'a> {
    fn new(map: &'a Map) -> Self {
        Self {
            map,
            seen: Vec::new(),
            pos: (map.start_x, 0, Down),
            steps: 0,
        }
    }

    fn run(&mut self) {
        while let Some((pos, character)) = self.map.next(&self.pos) {
            self.pos = pos;
            if let Some(c) = character {
                self.seen.push(c);
            }
            self.steps += 1;
        }
    }
}

type Pos = (usize, usize, Direction);

use Direction::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Map {
    paths: Vec<Vec<char>>,
    start_x: usize,
}

impl Map {
    fn new(paths: Vec<Vec<char>>) -> Self {
        Self {
            start_x: paths[0].iter().position(|c| c == &'|').unwrap(),
            paths,
        }
    }

    fn next(&self, pos: &Pos) -> Option<(Pos, Option<char>)> {
        match pos.2 {
            Up => match self.paths[pos.1 - 1][pos.0] {
                ' ' => None,
                '|' | '-' => Some(((pos.0, pos.1 - 1, Up), None)),
                '+' => Some((
                    (
                        pos.0,
                        pos.1 - 1,
                        if pos.0 == 0 || self.paths[pos.1 - 1][pos.0 - 1] == ' ' {
                            Right
                        } else {
                            Left
                        },
                    ),
                    None,
                )),
                c => Some(((pos.0, pos.1 - 1, Up), Some(c))),
            },
            Down => match self.paths[pos.1 + 1][pos.0] {
                ' ' => None,
                '|' | '-' => Some(((pos.0, pos.1 + 1, Down), None)),
                '+' => Some((
                    (
                        pos.0,
                        pos.1 + 1,
                        if pos.0 == 0 || self.paths[pos.1 + 1][pos.0 - 1] == ' ' {
                            Right
                        } else {
                            Left
                        },
                    ),
                    None,
                )),
                c => Some(((pos.0, pos.1 + 1, Down), Some(c))),
            },
            Left => match self.paths[pos.1][pos.0 - 1] {
                ' ' => None,
                '|' | '-' => Some(((pos.0 - 1, pos.1, Left), None)),
                '+' => Some((
                    (
                        pos.0 - 1,
                        pos.1,
                        if pos.1 == 1 || self.paths[pos.1 - 1][pos.0 - 1] == ' ' {
                            Down
                        } else {
                            Up
                        },
                    ),
                    None,
                )),
                c => Some(((pos.0 - 1, pos.1, Left), Some(c))),
            },
            Right => match self.paths[pos.1][pos.0 + 1] {
                ' ' => None,
                '|' | '-' => Some(((pos.0 + 1, pos.1, Right), None)),
                '+' => Some((
                    (
                        pos.0 + 1,
                        pos.1,
                        if pos.1 == 1 || self.paths[pos.1 - 1][pos.0 + 1] == ' ' {
                            Down
                        } else {
                            Up
                        },
                    ),
                    None,
                )),
                c => Some(((pos.0 + 1, pos.1, Right), Some(c))),
            },
        }
    }
}
