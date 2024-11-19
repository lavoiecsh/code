use std::fmt::{Debug, Formatter};

use crate::solver::AdventSolver;

pub struct Advent2022Day22Solver {
    map: Vec<Vec<char>>,
    face_size: usize,
    instructions: Vec<Instruction>,
}

struct Instruction {
    distance: Option<usize>,
    direction: Option<char>,
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (self.distance, self.direction) {
            (Some(d), None) => write!(f, "Instruction: Move forward {d}"),
            (None, Some(t)) => write!(f, "Instruction: Turn {t}"),
            _ => panic!("unknown instruction"),
        }
    }
}

impl Advent2022Day22Solver {
    pub fn new(input: &str) -> Self {
        let face_size = 50;
        let mut instructions = vec![];
        let mut number = String::new();
        for c in input.lines().last().unwrap().chars() {
            match c {
                'L' | 'R' => {
                    instructions.push(Instruction {
                        distance: Some(number.parse().unwrap()),
                        direction: None,
                    });
                    instructions.push(Instruction {
                        distance: None,
                        direction: Some(c),
                    });
                    number = String::new();
                }
                d => number.push(d),
            }
        }
        instructions.push(Instruction {
            distance: Some(number.parse().unwrap()),
            direction: None,
        });
        Self {
            map: input
                .lines()
                .take_while(|l| !l.is_empty())
                .map(|l| l.chars().collect())
                .collect(),
            face_size,
            instructions,
        }
    }
}

impl AdventSolver for Advent2022Day22Solver {
    fn solve_part1(&self) -> usize {
        let map = Map::new(&self.map);
        let mut character = Character::new(&map);
        for instruction in &self.instructions {
            character = character.execute(instruction, &map);
        }
        (character.position.0 + 1) * 1000 + (character.position.1 + 1) * 4 + character.direction
    }

    fn solve_part2(&self) -> usize {
        let cube = Cube::new(&self.map, self.face_size);
        let mut character = Character::new(&cube);
        for instruction in &self.instructions {
            character = character.execute(instruction, &cube);
        }
        (character.position.0 + 1) * 1000 + (character.position.1 + 1) * 4 + character.direction
    }
}

type Direction = usize;

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;

fn inc_direction(direction: Direction) -> Direction {
    if direction == 3 {
        0
    } else {
        direction + 1
    }
}

fn dec_direction(direction: Direction) -> Direction {
    if direction == 0 {
        3
    } else {
        direction - 1
    }
}

trait Surface {
    fn starting_position(&self) -> Position;
    fn move_forward(&self, character: &Character) -> Character;
    fn _pp(&self, character: &Character);
}

struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn new(map: &[Vec<char>]) -> Self {
        let longest_row = map.iter().map(|row| row.len()).max().unwrap();
        Self {
            map: map
                .iter()
                .map(|row| {
                    let mut row2 = row.clone();
                    while row2.len() != longest_row {
                        row2.push(' ');
                    }
                    row2
                })
                .collect(),
        }
    }
}

impl Surface for Map {
    fn starting_position(&self) -> Position {
        (0, self.map[0].iter().position(|c| c == &'.').unwrap())
    }

    fn move_forward(&self, character: &Character) -> Character {
        let (row, col) = match character.direction {
            RIGHT => {
                let row = character.position.0;
                let mut col = character.position.1 + 1;
                if col == self.map[row].len() || self.map[row][col] == ' ' {
                    col = 0;
                    while self.map[row][col] == ' ' {
                        col += 1;
                    }
                }
                (row, col)
            }
            DOWN => {
                let mut row = character.position.0 + 1;
                let col = character.position.1;
                if row == self.map.len() || self.map[row][col] == ' ' {
                    row = 0;
                    while self.map[row][col] == ' ' {
                        row += 1;
                    }
                }
                (row, col)
            }
            LEFT => {
                let row = character.position.0;
                let mut col = character.position.1 - 1;
                if character.position.1 == 0 || self.map[row][col] == ' ' {
                    col = self.map[row].len() - 1;
                    while self.map[row][col] == ' ' {
                        col -= 1;
                    }
                }
                (row, col)
            }
            UP => {
                let mut row = character.position.0 - 1;
                let col = character.position.1;
                if character.position.0 == 0 || self.map[row][col] == ' ' {
                    row = self.map.len() - 1;
                    while self.map[row][col] == ' ' {
                        row -= 1;
                    }
                }
                (row, col)
            }
            _ => panic!("unknown direction"),
        };
        if self.map[row][col] == '#' {
            *character
        } else {
            Character {
                position: (row, col),
                direction: character.direction,
            }
        }
    }

    fn _pp(&self, character: &Character) {
        _pp(&self.map, character);
    }
}

fn _pp(map: &[Vec<char>], character: &Character) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if character.position == (i, j) {
                print!(
                    "{}",
                    match character.direction {
                        RIGHT => '>',
                        DOWN => 'v',
                        LEFT => '<',
                        UP => '^',
                        _ => panic!("unknown direction"),
                    }
                );
            } else {
                print!("{}", map[i][j]);
            }
        }
        println!();
    }
    println!();
}

struct Cube {
    map: Vec<Vec<char>>,
    faces: Vec<Face>,
}

#[derive(Copy, Clone)]
struct Face {
    index: usize,
    top_left: Position,
    bottom_right: Position,
    relations: [Option<(usize, Direction)>; 4],
}

impl Debug for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Face {}: [({},{}) -> ({},{})] -> {} {} {} {}",
            self.index,
            self.top_left.0,
            self.top_left.1,
            self.bottom_right.0,
            self.bottom_right.1,
            display_face_relation(&self.relations[RIGHT]),
            display_face_relation(&self.relations[DOWN]),
            display_face_relation(&self.relations[LEFT]),
            display_face_relation(&self.relations[UP])
        )
    }
}

fn display_face_relation(relation: &Option<(usize, Direction)>) -> String {
    match relation {
        None => String::from("None"),
        Some((f, d)) => format!("({f}, {d})"),
    }
}

impl Cube {
    fn new(map: &[Vec<char>], face_size: usize) -> Self {
        let mut faces = vec![];
        let mut face_index = 0;
        for i in 0..(map.len() / face_size) {
            let true_i = i * face_size;
            for j in 0..(map[true_i].len() / face_size) {
                let true_j = j * face_size;
                if map[true_i][true_j] != ' ' {
                    faces.push(Face {
                        index: face_index,
                        top_left: (true_i, true_j),
                        bottom_right: (true_i + face_size - 1, true_j + face_size - 1),
                        relations: [None; 4],
                    });
                    face_index += 1;
                }
            }
        }
        faces = build_face_relation_1(&faces, face_size);
        while faces.iter().any(|f| f.relations.contains(&None)) {
            faces = build_face_relation_2(&faces);
        }
        Self {
            map: map.to_owned(),
            faces,
        }
    }

    fn face_index(&self, position: &Position) -> Option<usize> {
        self.faces.iter().position(|f| {
            f.top_left.0 <= position.0
                && f.top_left.1 <= position.1
                && f.bottom_right.0 >= position.0
                && f.bottom_right.1 >= position.1
        })
    }
}

fn build_face_relation_1(faces: &[Face], face_size: usize) -> Vec<Face> {
    let mut new_faces: Vec<Face> = faces.to_owned();
    for i in 0..faces.len() {
        for j in 0..faces.len() {
            if i == j {
                continue;
            }
            if faces[i].relations[RIGHT].is_none()
                && faces[j].relations[LEFT].is_none()
                && faces[j].top_left.0 == faces[i].top_left.0
                && faces[j].top_left.1 == faces[i].top_left.1 + face_size
            {
                new_faces[i].relations[RIGHT] = Some((j, RIGHT));
                new_faces[j].relations[LEFT] = Some((i, LEFT));
                continue;
            }
            if faces[i].relations[DOWN].is_none()
                && faces[j].relations[UP].is_none()
                && faces[j].top_left.1 == faces[i].top_left.1
                && faces[j].top_left.0 == faces[i].top_left.0 + face_size
            {
                new_faces[i].relations[DOWN] = Some((j, DOWN));
                new_faces[j].relations[UP] = Some((i, UP));
                continue;
            }
        }
    }
    new_faces
}

fn build_face_relation_2(faces: &[Face]) -> Vec<Face> {
    let mut new_faces: Vec<Face> = faces.to_owned();
    for i in 0..faces.len() {
        for d in 0..=3 {
            if new_faces[i].relations[d].is_none() && faces[i].relations[dec_direction(d)].is_some()
            {
                let starting = faces[i].relations[dec_direction(d)].unwrap();
                new_faces[i].relations[d] = faces[starting.0].relations[inc_direction(starting.1)]
                    .map(|f| (f.0, dec_direction(f.1)));
            }
            if new_faces[i].relations[d].is_none() && faces[i].relations[inc_direction(d)].is_some()
            {
                let starting = faces[i].relations[inc_direction(d)].unwrap();
                new_faces[i].relations[d] = faces[starting.0].relations[dec_direction(starting.1)]
                    .map(|f| (f.0, inc_direction(f.1)));
            }
        }
    }
    new_faces
}

impl Surface for Cube {
    fn starting_position(&self) -> Position {
        (0, self.map[0].iter().position(|c| c == &'.').unwrap())
    }

    fn move_forward(&self, character: &Character) -> Character {
        let mut moving = *character;
        let initial_face = self.face_index(&moving.position);
        if initial_face.is_none() {
            panic!("initial face none");
        }
        match character.direction {
            RIGHT => {
                moving.position.1 += 1;
            }
            DOWN => {
                moving.position.0 += 1;
            }
            LEFT => {
                moving.position.1 -= 1;
            }
            UP => {
                moving.position.0 -= 1;
            }
            _ => panic!("unknown direction"),
        }
        if self.face_index(&moving.position) == initial_face {
            return if self.map[moving.position.0][moving.position.1] == '#' {
                *character
            } else {
                moving
            };
        }
        let face = &self.faces[initial_face.unwrap()];
        let (next_face_index, next_direction) = face.relations[character.direction].unwrap();
        let next_face = &self.faces[next_face_index];
        moving.direction = next_direction;
        match character.direction {
            RIGHT => {
                let distance = face.bottom_right.0 - character.position.0;
                moving.position = match next_direction {
                    RIGHT => (character.position.0, character.position.1 + 1),
                    DOWN => (next_face.top_left.0, next_face.top_left.1 + distance),
                    LEFT => (next_face.top_left.0 + distance, next_face.bottom_right.1),
                    UP => (
                        next_face.bottom_right.0,
                        next_face.bottom_right.1 - distance,
                    ),
                    _ => panic!("unknown direction"),
                };
            }
            DOWN => {
                let distance = face.bottom_right.1 - character.position.1;
                moving.position = match next_direction {
                    RIGHT => (next_face.top_left.0 + distance, next_face.top_left.1),
                    DOWN => (next_face.top_left.0, next_face.bottom_right.1 - distance),
                    LEFT => (
                        next_face.bottom_right.0 - distance,
                        next_face.bottom_right.1,
                    ),
                    UP => (next_face.bottom_right.0, next_face.top_left.1 + distance),
                    _ => panic!("unknown direction"),
                };
            }
            LEFT => {
                let distance = face.bottom_right.0 - character.position.0;
                moving.position = match next_direction {
                    RIGHT => (next_face.top_left.0 + distance, next_face.top_left.1),
                    DOWN => (next_face.top_left.0, next_face.bottom_right.1 - distance),
                    LEFT => (character.position.0, character.position.1 - 1),
                    UP => (next_face.bottom_right.0, next_face.top_left.1 + distance),
                    _ => panic!("unknown direction"),
                };
            }
            UP => {
                let distance = character.position.1 - face.top_left.1;
                moving.position = match next_direction {
                    RIGHT => (next_face.top_left.0 + distance, next_face.top_left.1),
                    DOWN => (
                        next_face.bottom_right.0,
                        next_face.bottom_right.1 - distance,
                    ),
                    LEFT => (
                        next_face.bottom_right.0 - distance,
                        next_face.bottom_right.1,
                    ),
                    UP => (next_face.bottom_right.0, next_face.top_left.1 + distance),
                    _ => panic!("unknown direction"),
                };
            }
            _ => panic!("unknown direction"),
        }
        if self.map[moving.position.0][moving.position.1] == '#' {
            *character
        } else {
            moving
        }
    }

    fn _pp(&self, character: &Character) {
        _pp(&self.map, character);
    }
}

type Position = (usize, usize);

#[derive(Copy, Clone, Eq, PartialEq)]
struct Character {
    position: (usize, usize),
    direction: Direction,
}

impl Debug for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Character [({}, {}) => {}]",
            self.position.0, self.position.1, self.direction
        )
    }
}

impl Character {
    fn new(surface: &dyn Surface) -> Self {
        Self {
            position: surface.starting_position(),
            direction: 0,
        }
    }

    fn execute(&self, instruction: &Instruction, surface: &dyn Surface) -> Self {
        match instruction.direction {
            Some('L') => {
                return Self {
                    position: self.position,
                    direction: dec_direction(self.direction),
                };
            }
            Some('R') => {
                return Self {
                    position: self.position,
                    direction: inc_direction(self.direction),
                };
            }
            None => {}
            _ => panic!("unknown direction"),
        }
        let mut character = *self;
        for _ in 0..instruction.distance.unwrap() {
            let tmp_character = surface.move_forward(&character);
            if tmp_character == character {
                break;
            }
            character = tmp_character;
        }
        character
    }
}
