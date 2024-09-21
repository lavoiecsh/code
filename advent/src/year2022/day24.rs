use std::collections::HashSet;

use crate::solver::AdventSolver;

pub struct Advent2022Day24Solver {
    map: Vec<Vec<Square>>,
}

#[derive(Clone)]
struct Map {
    map: Vec<Vec<Square>>,
}

impl Map {
    fn new(map: &[Vec<Square>]) -> Self {
        Self { map: map.to_owned() }
    }

    fn entrance(&self) -> (usize, usize) {
        (0, self.map[0].iter().position(|s| !s.wall).unwrap())
    }

    fn exit(&self) -> (usize, usize) {
        (self.map.len() - 1, self.map[self.map.len() - 1].iter().position(|s| !s.wall).unwrap())
    }

    fn is_open(&self, row: usize, col: usize) -> bool {
        if row >= self.map.len() || col >= self.map[row].len() { return false; }
        let square = &self.map[row][col];
        !square.wall && square.blizzards.is_empty()
    }

    fn iterate(&self) -> Self {
        let mut next = Self { map: vec!() };
        for row in 0..self.map.len() {
            next.map.push(vec!());
            for col in 0..self.map[row].len() {
                next.map[row].push(Square { wall: self.map[row][col].wall, blizzards: vec!() });
            }
        }
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                if self.map[row][col].wall { continue; }
                for b in &self.map[row][col].blizzards {
                    let mut next_row = row;
                    let mut next_col = col;
                    match b.direction {
                        '>' => {
                            next_col += 1;
                            if self.map[next_row][next_col].wall {
                                next_col = self.map[next_row].iter().position(|s| !s.wall).unwrap();
                            }
                        }
                        'v' => {
                            next_row += 1;
                            if self.map[next_row][next_col].wall {
                                next_row = self.map.iter().position(|r| !r[next_col].wall).unwrap();
                            }
                        }
                        '<' => {
                            next_col -= 1;
                            if self.map[next_row][next_col].wall {
                                next_col = self.map[next_row].iter().rposition(|s| !s.wall).unwrap();
                            }
                        }
                        '^' => {
                            next_row -= 1;
                            if self.map[next_row][next_col].wall {
                                next_row = self.map.iter().rposition(|r| !r[next_col].wall).unwrap();
                            }
                        }
                        _ => panic!("unknown direction"),
                    }
                    next.map[next_row][next_col].blizzards.push(b.clone());
                }
            }
        }
        next
    }

    fn _pp(&self) {
        for row in 0..self.map.len() {
            for col in 0..self.map[row].len() {
                let square = &self.map[row][col];
                if square.wall {
                    print!("#");
                } else if square.blizzards.is_empty() {
                    print!(".");
                } else if square.blizzards.len() > 1 {
                    print!("{}", square.blizzards.len());
                } else {
                    print!("{}", square.blizzards[0].direction);
                }
            }
            println!();
        }
        println!();
    }
}

#[derive(Clone)]
struct Square {
    wall: bool,
    blizzards: Vec<Blizzard>,
}

impl Square {
    fn from(c: char) -> Square {
        Square {
            wall: c == '#',
            blizzards: match c {
                '>' | 'v' | '<' | '^' => vec!(Blizzard { direction: c }),
                _ => vec!(),
            },
        }
    }
}

#[derive(Clone)]
struct Blizzard {
    direction: char,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Character {
    row: usize,
    col: usize,
}

impl Character {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn next_positions(&self, map: &Map) -> Vec<Self> {
        [
            Character { row: self.row, col: self.col },
            Character { row: self.row - 1, col: self.col },
            Character { row: self.row + 1, col: self.col },
            Character { row: self.row, col: self.col - 1 },
            Character { row: self.row, col: self.col + 1 },
        ]
            .iter()
            .filter(|c| map.is_open(c.row, c.col)).cloned()
            .collect()
    }
}

impl Advent2022Day24Solver {
    pub fn new(input: String) -> Self {
        Self {
            map: input
                .lines()
                .map(|l| l.chars().map(Square::from).collect())
                .collect()
        }
    }
}

impl AdventSolver for Advent2022Day24Solver {
    fn solve_part1(&self) -> usize {
        let mut map = Map::new(&self.map);
        let entrance = map.entrance();
        let exit = map.exit();
        let mut characters: HashSet<Character> = HashSet::new();
        characters.insert(Character::new(entrance.0, entrance.1));
        let mut i = 0;
        while !characters.iter().any(|c| c.row == exit.0 && c.col == exit.1) {
            i += 1;
            map = map.iterate();
            characters = characters.iter().flat_map(|c| c.next_positions(&map)).collect();
        }
        i
    }

    fn solve_part2(&self) -> usize {
        let mut map = Map::new(&self.map);
        let entrance = map.entrance();
        let exit = map.exit();
        let mut characters: HashSet<Character> = HashSet::new();
        characters.insert(Character::new(entrance.0, entrance.1));
        let mut i = 0;
        while !characters.iter().any(|c| c.row == exit.0 && c.col == exit.1) {
            i += 1;
            map = map.iterate();
            characters = characters.iter().flat_map(|c| c.next_positions(&map)).collect();
        }
        characters.clear();
        characters.insert(Character::new(exit.0, exit.1));
        while !characters.iter().any(|c| c.row == entrance.0 && c.col == entrance.1) {
            i += 1;
            map = map.iterate();
            characters = characters.iter().flat_map(|c| c.next_positions(&map)).collect();
        }
        characters.clear();
        characters.insert(Character::new(entrance.0, entrance.1));
        while !characters.iter().any(|c| c.row == exit.0 && c.col == exit.1) {
            i += 1;
            map = map.iterate();
            characters = characters.iter().flat_map(|c| c.next_positions(&map)).collect();
        }
        i
    }
}
