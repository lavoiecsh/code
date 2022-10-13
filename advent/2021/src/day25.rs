#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

const FILENAME: &str = "inputs/day25.txt";

#[allow(dead_code)]
fn read_input() -> Map {
    Map::new(fs::read_to_string(FILENAME).expect("error reading")
             .trim()
             .lines()
             .map(|l| l.chars().collect())
             .collect())
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let mut map = read_input();
    debug!(&map);
    let mut next_map = map.iterate();
    let mut steps = 1;
    while !map.equals(&next_map) {
        map = next_map;
        next_map = map.iterate();
        steps += 1;
    }
    steps
}

#[allow(dead_code)]
pub fn part2() -> usize {
    0
}

type Coord = (usize, usize);

struct Map {
    east: Vec<Coord>,
    south: Vec<Coord>,
    size: Coord,
}

impl Map {
    fn new(input: Vec<Vec<char>>) -> Self {
        let size = (input.len(), input[0].len());
        let mut s = Self {
            east: Vec::new(),
            south: Vec::new(),
            size: size,
        };
        for row in 0..size.0 {
            for col in 0..size.1 {
                match input[row][col] {
                    '>' => { s.east.push((row, col)); },
                    'v' => { s.south.push((row, col)); },
                    '.' => {},
                    _ => panic!("unknown character"),
                }
            }
        }
        s
    }

    fn iterate(&self) -> Self {
        let mut next_east: Vec<Coord> = Vec::new();
        for east in &self.east {
            let next_coord = (east.0, if east.1 == self.size.1 - 1 { 0 } else { east.1 + 1 });
            if self.east.contains(&next_coord) || self.south.contains(&next_coord) {
                next_east.push(*east);
            } else {
                next_east.push(next_coord);
            }
        }
        let mut next_south: Vec<Coord> = Vec::new();
        for south in &self.south {
            let next_coord = (if south.0 == self.size.0 - 1 { 0 } else { south.0 + 1 }, south.1);
            if next_east.contains(&next_coord) || self.south.contains(&next_coord) {
                next_south.push(*south);
            } else {
                next_south.push(next_coord);
            }
            
        }
        Self {
            size: self.size,
            east: next_east,
            south: next_south,
        }
    }

    fn equals(&self, other: &Map) -> bool {
        self.east == other.east && self.south == other.south
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut lines = Vec::new();
        for row in 0..self.size.0 {
            let mut line = "".to_string();
            for col in 0..self.size.1 {
                if self.east.contains(&(row, col)) {
                    line += ">";
                } else if self.south.contains(&(row, col)) {
                    line += "v";
                } else {
                    line += ".";
                }
            }
            lines.push(line);
        }
        write!(f, "{:#?}", lines)
    }
}
