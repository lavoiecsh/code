use std::cmp::Ordering;
use Direction::*;

use crate::solver::AdventSolver;

pub struct Advent2023Day10Solver {
    grid: Grid,
}

impl Advent2023Day10Solver {
    pub fn new(input: String) -> Self {
        Self {
            grid: Grid::new(input.lines().map(|l| l.chars().collect()).collect())
        }
    }
}

impl AdventSolver for Advent2023Day10Solver {
    fn solve_part1(&self) -> usize {
        self.grid.farthest_distance_from_start()
    }

    fn solve_part2(&self) -> usize {
        self.grid.included_in_loop()
    }
}

type Pos = (usize, usize);

#[derive(Debug)]
enum Direction { North, East, South, West }

struct Grid {
    tiles: Vec<Vec<char>>,
    starting_position: Pos,
}

impl Grid {
    fn new(tiles: Vec<Vec<char>>) -> Self {
        Self {
            starting_position: tiles.iter()
                .enumerate()
                .filter_map(|(y, row)| row.iter().position(|&t| t == 'S').map(|x| (x, y)))
                .next()
                .unwrap(),
            tiles,
        }
    }

    fn farthest_distance_from_start(&self) -> usize {
        self.path().len() / 2
    }

    fn included_in_loop(&self) -> usize {
        let path = self.path();

        let start = path[0];
        let next = path[1];
        let prev = path[path.len() - 1];
        let from = match start.0.cmp(&prev.0) {
            Ordering::Equal => if start.1 > prev.1 { 'a' } else { 'b' },
            Ordering::Greater => 'l',
            Ordering::Less => 'r',
        };
        let to = match start.0.cmp(&next.0) {
            Ordering::Equal => if start.1 > next.1 { 'a' } else { 'b' },
            Ordering::Greater => 'l',
            Ordering::Less => 'r',
        };
        let start_char = match (from, to) {
            ('a', 'b') => '|',
            ('a', 'l') => 'J',
            ('a', 'r') => 'L',
            ('b', 'a') => '|',
            ('b', 'l') => '7',
            ('b', 'r') => 'F',
            ('l', 'r') => '-',
            ('l', 'a') => 'J',
            ('l', 'b') => '7',
            ('r', 'l') => '-',
            ('r', 'a') => 'L',
            ('r', 'b') => 'F',
            _ => panic!(),
        };
        let mut tiles: Vec<Vec<char>> = Vec::new();
        for y in 0..self.tiles.len() {
            let mut row1 = Vec::new();
            let mut row2 = Vec::new();
            let mut row3 = Vec::new();
            for x in 0..self.tiles[y].len() {
                if let Some(i) = path.iter().position(|&p| p == (x, y)) {
                    let c = if i == 0 { start_char } else { self.tiles[y][x] };
                    match c {
                        '-' => {
                            row1.extend("...".chars());
                            row2.extend("---".chars());
                            row3.extend("...".chars());
                        }
                        '|' => {
                            row1.extend(".|.".chars());
                            row2.extend(".|.".chars());
                            row3.extend(".|.".chars());
                        }
                        'L' => {
                            row1.extend(".|.".chars());
                            row2.extend(".L-".chars());
                            row3.extend("...".chars());
                        }
                        'F' => {
                            row1.extend("...".chars());
                            row2.extend(".F-".chars());
                            row3.extend(".|.".chars());
                        }
                        '7' => {
                            row1.extend("...".chars());
                            row2.extend("-7.".chars());
                            row3.extend(".|.".chars());
                        }
                        'J' => {
                            row1.extend(".|.".chars());
                            row2.extend("-J.".chars());
                            row3.extend("...".chars());
                        }
                        _ => panic!(),
                    }
                } else {
                    row1.extend("...".chars());
                    row2.extend("...".chars());
                    row3.extend("...".chars());
                }
            }
            tiles.push(row1);
            tiles.push(row2);
            tiles.push(row3);
        }
        let mut queue = vec!((0, 0));
        while let Some(current) = queue.pop() {
            if tiles[current.1][current.0] != '.' { continue; }
            tiles[current.1][current.0] = 'O';
            queue.extend([
                if current.0 == 0 { None } else { Some((current.0 - 1, current.1)) },
                if current.0 == tiles[current.1].len() - 1 { None } else { Some((current.0 + 1, current.1)) },
                if current.1 == 0 { None } else { Some((current.0, current.1 - 1)) },
                if current.1 == tiles.len() - 1 { None } else { Some((current.0, current.1 + 1)) },
            ].iter().filter_map(|&a| a))
        }
        (0..self.tiles.len())
            .map(|y| (0..self.tiles[y].len()).filter(|x| tiles[y * 3 + 1][x * 3 + 1] == '.').count())
            .sum()
    }

    fn path(&self) -> Vec<Pos> {
        let mut path = vec!(self.starting_position);
        let mut current = self.after_starting();
        while current.0 != self.starting_position {
            path.push(current.0);
            current = self.next(current);
        }
        path
    }

    fn next(&self, current: (Pos, Direction)) -> (Pos, Direction) {
        let pos = current.0;
        let direction = current.1;
        let next_pos = match direction {
            North => (pos.0, pos.1 - 1),
            East => (pos.0 + 1, pos.1),
            South => (pos.0, pos.1 + 1),
            West => (pos.0 - 1, pos.1),
        };
        match (self.tiles[next_pos.1][next_pos.0], direction) {
            ('|', North) => (next_pos, North),
            ('|', South) => (next_pos, South),
            ('-', East) => (next_pos, East),
            ('-', West) => (next_pos, West),
            ('L', South) => (next_pos, East),
            ('L', West) => (next_pos, North),
            ('J', South) => (next_pos, West),
            ('J', East) => (next_pos, North),
            ('7', North) => (next_pos, West),
            ('7', East) => (next_pos, South),
            ('F', North) => (next_pos, East),
            ('F', West) => (next_pos, South),
            ('S', d) => (next_pos, d),
            _ => panic!("no next position found"),
        }
    }

    fn after_starting(&self) -> (Pos, Direction) {
        if self.starting_position.1 != 0 {
            let pos = (self.starting_position.0, self.starting_position.1 - 1);
            match self.tiles[pos.1][pos.0] {
                '|' => return (pos, North),
                'F' => return (pos, East),
                '7' => return (pos, West),
                _ => {}
            }
        }
        if self.starting_position.0 != self.tiles[self.starting_position.1].len() - 1 {
            let pos = (self.starting_position.0 + 1, self.starting_position.1);
            match self.tiles[pos.1][pos.0] {
                '-' => return (pos, East),
                'J' => return (pos, North),
                '7' => return (pos, South),
                _ => {}
            }
        }
        if self.starting_position.1 != self.tiles.len() - 1 {
            let pos = (self.starting_position.0, self.starting_position.1 + 1);
            match self.tiles[pos.1][pos.0] {
                '|' => return (pos, South),
                'J' => return (pos, West),
                'L' => return (pos, East),
                _ => {}
            }
        }
        if self.starting_position.0 != 0 {
            let pos = (self.starting_position.0 - 1, self.starting_position.1);
            match self.tiles[pos.1][pos.0] {
                '-' => return (pos, West),
                'L' => return (pos, North),
                'F' => return (pos, South),
                _ => {}
            }
        }
        panic!("no position after starting found")
    }
}

#[test]
fn farthest_in_loop_1() {
    let solver = Advent2023Day10Solver::new(String::from("\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"));
    assert_eq!(solver.solve_part1(), 4);
}

#[test]
fn farthest_in_loop_2() {
    let solver = Advent2023Day10Solver::new(String::from("\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"));
    assert_eq!(solver.solve_part1(), 8);
}

#[test]
fn included_in_loop_1() {
    let solver = Advent2023Day10Solver::new(String::from("\
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"));
    assert_eq!(solver.solve_part2(), 4);
}

#[test]
fn included_in_loop_2() {
    let solver = Advent2023Day10Solver::new(String::from("\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"));
    assert_eq!(solver.solve_part2(), 8);
}

#[test]
fn included_in_loop_3() {
    let solver = Advent2023Day10Solver::new(String::from("\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"));
    assert_eq!(solver.solve_part2(), 10);
}
