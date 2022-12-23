use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;
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
    pub fn new() -> Self {
        let text = read_to_string("src/year2022/day22.txt").unwrap();
        let face_size = 50;
        let mut instructions = vec!();
        let mut number = String::new();
        for c in text.lines().last().unwrap().chars() {
            match c {
                'L' | 'R' => {
                    instructions.push(Instruction { distance: Some(number.parse().unwrap()), direction: None });
                    instructions.push(Instruction { distance: None, direction: Some(c) });
                    number = String::new();
                }
                d => number.push(d),
            }
        }
        instructions.push(Instruction { distance: Some(number.parse().unwrap()), direction: None });
        Self {
            map: text.lines().take_while(|l| !l.is_empty())
                .map(|l| l.chars().collect())
                .collect(),
            face_size,
            instructions,
        }
    }
}

impl AdventSolver for Advent2022Day22Solver {
    fn day(&self) -> usize { 22 }
    fn year(&self) -> usize { 2022 }

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
        // return 0;
        let mut character = Character::new(&cube);
        dbg!(&character);
        for instruction in &self.instructions {
            dbg!(&instruction);
            character = character.execute(instruction, &cube);
            dbg!(&character);
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
    if direction == 3 { 0 } else { direction + 1 }
}

fn dec_direction(direction: Direction) -> Direction {
    if direction == 0 { 3 } else { direction - 1 }
}

trait Surface {
    fn starting_position(&self) -> Position;
    fn move_forward(&self, character: &Character) -> Character;
    fn pp(&self, character: &Character) -> ();
}

struct Map {
    map: Vec<Vec<char>>,
}

impl Map {
    fn new(map: &Vec<Vec<char>>) -> Self {
        let longest_row = map.iter().map(|row| row.len()).max().unwrap();
        Self {
            map: map.iter()
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
            character.clone()
        } else {
            Character { position: (row, col), direction: character.direction.clone() }
        }
    }

    fn pp(&self, character: &Character) -> () {
        pp(&self.map, &character);
    }
}

fn pp(map: &Vec<Vec<char>>, character: &Character) -> () {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if character.position == (i, j) {
                print!("{}", match character.direction {
                    RIGHT => '>',
                    DOWN => 'v',
                    LEFT => '<',
                    UP => '^',
                    _ => panic!("unknown direction"),
                });
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
    face_size: usize,
    faces: Vec<Face>,
}

struct Face {
    index: usize,
    top_left: Position,
    bottom_right: Position,
    right: Option<(usize, Direction)>,
    down: Option<(usize, Direction)>,
    left: Option<(usize, Direction)>,
    up: Option<(usize, Direction)>,
}

impl Debug for Face {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Face {}: [({},{}) -> ({},{})] -> {} {} {} {}",
               self.index,
               self.top_left.0, self.top_left.1, self.bottom_right.0, self.bottom_right.1,
               display_face_relation(&self.right),
               display_face_relation(&self.down),
               display_face_relation(&self.left),
               display_face_relation(&self.up)
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
    fn new(map: &Vec<Vec<char>>, face_size: usize) -> Self {
        let mut faces = vec!();
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
                        right: None,
                        down: None,
                        left: None,
                        up: None,
                    });
                    face_index += 1;
                }
            }
        }
        faces[0].right = Some((1,0));
        faces[0].down = Some((2,1));
        faces[0].left = Some((3,0));
        faces[0].up = Some((5,0));
        faces[1].right = Some((4,2));
        faces[1].down = Some((2,2));
        faces[1].left = Some((0,2));
        faces[1].up = Some((5,3));
        faces[2].right = Some((1,3));
        faces[2].down = Some((4,1));
        faces[2].left = Some((3,1));
        faces[2].up = Some((0,3));
        faces[3].right = Some((4,0));
        faces[3].down = Some((5,1));
        faces[3].left = Some((0,0));
        faces[3].up = Some((2,0));
        faces[4].right = Some((1,2));
        faces[4].down = Some((5,2));
        faces[4].left = Some((3,2));
        faces[4].up = Some((2,3));
        faces[5].right = Some((4,3));
        faces[5].down = Some((1,1));
        faces[5].left = Some((0,1));
        faces[5].up = Some((3,3));
        // dbg!(&faces);
        // for i in 0..faces.len() {
        //     for j in 0..faces.len() {
        //         if i == j { continue; }
        //         if faces[i].right.is_none()
        //             && faces[j].left.is_none()
        //             && faces[j].top_left.0 == faces[i].top_left.0
        //             && faces[j].top_left.1 == faces[i].top_left.1 + face_size {
        //             faces[i].right = Some((j, RIGHT));
        //             faces[j].left = Some((i, LEFT));
        //             continue;
        //         }
        //         if faces[i].down.is_none()
        //             && faces[j].up.is_none()
        //             && faces[j].top_left.1 == faces[i].top_left.1
        //             && faces[j].top_left.0 == faces[i].top_left.0 + face_size {
        //             faces[i].down = Some((j, DOWN));
        //             faces[j].up = Some((i, UP));
        //             continue;
        //         }
        //     }
        // }
        // dbg!(&faces);
        // for i in 0..faces.len() {
        //     if faces[i].right.is_none() {
        //         if faces[i].up.is_some() && faces[faces[i].up.unwrap().0].right.is_some() && faces[faces[i].up.unwrap().0].right.unwrap().0 != i {
        //             faces[i].right = Some((faces[faces[i].up.unwrap().0].right.unwrap().0, UP));
        //         } else if faces[i].down.is_some() && faces[faces[i].down.unwrap().0].right.is_some() && faces[faces[i].down.unwrap().0].right.unwrap().0 != i {
        //             faces[i].right = Some((faces[faces[i].down.unwrap().0].right.unwrap().0, DOWN));
        //         }
        //     }
        //     if faces[i].down.is_none() {
        //         if faces[i].right.is_some() && faces[faces[i].right.unwrap().0].down.is_some() && faces[faces[i].right.unwrap().0].down.unwrap().0 != i {
        //             faces[i].down = Some((faces[faces[i].right.unwrap().0].down.unwrap().0, RIGHT));
        //         } else if faces[i].left.is_some() && faces[faces[i].left.unwrap().0].down.is_some() && faces[faces[i].left.unwrap().0].down.unwrap().0 != i {
        //             faces[i].down = Some((faces[faces[i].left.unwrap().0].down.unwrap().0, LEFT));
        //         }
        //     }
        //     if faces[i].left.is_none() {
        //         if faces[i].up.is_some() && faces[faces[i].up.unwrap().0].left.is_some() && faces[faces[i].up.unwrap().0].left.unwrap().0 != i {
        //             faces[i].left = Some((faces[faces[i].up.unwrap().0].left.unwrap().0, UP));
        //         } else if faces[i].down.is_some() && faces[faces[i].down.unwrap().0].left.is_some() && faces[faces[i].down.unwrap().0].left.unwrap().0 != i {
        //             faces[i].left = Some((faces[faces[i].down.unwrap().0].left.unwrap().0, DOWN));
        //         }
        //     }
        //     if faces[i].up.is_none() {
        //         if faces[i].right.is_some() && faces[faces[i].right.unwrap().0].up.is_some() && faces[faces[i].right.unwrap().0].up.unwrap().0 != i {
        //             faces[i].up = Some((faces[faces[i].right.unwrap().0].up.unwrap().0, RIGHT));
        //         } else if faces[i].left.is_some() && faces[faces[i].left.unwrap().0].up.is_some() && faces[faces[i].left.unwrap().0].up.unwrap().0 != i {
        //             faces[i].up = Some((faces[faces[i].left.unwrap().0].up.unwrap().0, LEFT));
        //         }
        //     }
        // }
        // dbg!(&faces);
        // for i in 0..faces.len() {
        //     if faces[i].right.is_none() {
        //         if faces[i].down.is_some()
        //             && faces[faces[i].down.unwrap().0].down.is_some()
        //             && faces[faces[faces[i].down.unwrap().0].down.unwrap().0].right.is_some()
        //             && faces[faces[faces[i].down.unwrap().0].down.unwrap().0].right.unwrap().0 != i {
        //             faces[i].right = Some((faces[faces[faces[i].down.unwrap().0].down.unwrap().0].right.unwrap().0, LEFT));
        //         } else if faces[i].up.is_some()
        //             && faces[faces[i].up.unwrap().0].up.is_some()
        //             && faces[faces[faces[i].up.unwrap().0].up.unwrap().0].right.is_some()
        //             && faces[faces[faces[i].up.unwrap().0].up.unwrap().0].right.unwrap().0 != i {
        //             faces[i].right = Some((faces[faces[faces[i].up.unwrap().0].up.unwrap().0].right.unwrap().0, LEFT));
        //         } else if faces[i].left.is_some()
        //             && faces[faces[i].left.unwrap().0].down.is_some()
        //             && faces[faces[faces[i].left.unwrap().0].down.unwrap().0].down.is_some()
        //             && faces[faces[faces[i].left.unwrap().0].down.unwrap().0].down.unwrap().0 != i {
        //             faces[i].right = Some((faces[faces[faces[i].left.unwrap().0].down.unwrap().0].down.unwrap().0, LEFT));
        //         }
        //     }
        //     if faces[i].down.is_none() {
        //         if faces[i].right.is_some()
        //             && faces[faces[i].right.unwrap().0].right.is_some()
        //             && faces[faces[faces[i].right.unwrap().0].right.unwrap().0].down.is_some()
        //             && faces[faces[faces[i].right.unwrap().0].right.unwrap().0].down.unwrap().0 != i {
        //             faces[i].down = Some((faces[faces[faces[i].right.unwrap().0].right.unwrap().0].down.unwrap().0, UP));
        //         } else if faces[i].left.is_some()
        //             && faces[faces[i].left.unwrap().0].left.is_some()
        //             && faces[faces[faces[i].left.unwrap().0].left.unwrap().0].down.is_some()
        //             && faces[faces[faces[i].left.unwrap().0].left.unwrap().0].down.unwrap().0 != i {
        //             faces[i].down = Some((faces[faces[faces[i].left.unwrap().0].left.unwrap().0].down.unwrap().0, UP));
        //         } else if faces[i].up.is_some()
        //             && faces[faces[i].up.unwrap().0].left.is_some()
        //             && faces[faces[faces[i].up.unwrap().0].left.unwrap().0].left.is_some()
        //             && faces[faces[faces[i].up.unwrap().0].left.unwrap().0].left.unwrap().0 != i {
        //             faces[i].down = Some((faces[faces[faces[i].up.unwrap().0].left.unwrap().0].left.unwrap().0, UP));
        //         }
        //     }
        //     if faces[i].left.is_none() {
        //         if faces[i].down.is_some()
        //             && faces[faces[i].down.unwrap().0].down.is_some()
        //             && faces[faces[faces[i].down.unwrap().0].down.unwrap().0].left.is_some()
        //             && faces[faces[faces[i].down.unwrap().0].down.unwrap().0].left.unwrap().0 != i {
        //             faces[i].left = Some((faces[faces[faces[i].down.unwrap().0].down.unwrap().0].left.unwrap().0, RIGHT));
        //         } else if faces[i].up.is_some()
        //             && faces[faces[i].up.unwrap().0].up.is_some()
        //             && faces[faces[faces[i].up.unwrap().0].up.unwrap().0].left.is_some()
        //             && faces[faces[faces[i].up.unwrap().0].up.unwrap().0].left.unwrap().0 != i {
        //             faces[i].left = Some((faces[faces[faces[i].up.unwrap().0].up.unwrap().0].left.unwrap().0, RIGHT));
        //         } else if faces[i].right.is_some()
        //             && faces[faces[i].right.unwrap().0].up.is_some()
        //             && faces[faces[faces[i].right.unwrap().0].up.unwrap().0].up.is_some()
        //             && faces[faces[faces[i].right.unwrap().0].up.unwrap().0].up.unwrap().0 != i {
        //             faces[i].left = Some((faces[faces[faces[i].right.unwrap().0].up.unwrap().0].up.unwrap().0, RIGHT));
        //         }
        //     }
        //     if faces[i].up.is_none() {
        //         if faces[i].right.is_some()
        //             && faces[faces[i].right.unwrap().0].right.is_some()
        //             && faces[faces[faces[i].right.unwrap().0].right.unwrap().0].up.is_some()
        //             && faces[faces[faces[i].right.unwrap().0].right.unwrap().0].up.unwrap().0 != i {
        //             faces[i].up = Some((faces[faces[faces[i].right.unwrap().0].right.unwrap().0].up.unwrap().0, DOWN));
        //         } else if faces[i].left.is_some()
        //             && faces[faces[i].left.unwrap().0].left.is_some()
        //             && faces[faces[faces[i].left.unwrap().0].left.unwrap().0].up.is_some()
        //             && faces[faces[faces[i].left.unwrap().0].left.unwrap().0].up.unwrap().0 != i {
        //             faces[i].up = Some((faces[faces[faces[i].left.unwrap().0].left.unwrap().0].up.unwrap().0, DOWN));
        //         } else if faces[i].down.is_some()
        //             && faces[faces[i].down.unwrap().0].down.is_some()
        //             && faces[faces[faces[i].down.unwrap().0].down.unwrap().0].down.is_some()
        //             && faces[faces[faces[i].down.unwrap().0].down.unwrap().0].down.unwrap().0 != i {
        //             faces[i].up = Some((faces[faces[faces[i].down.unwrap().0].down.unwrap().0].down.unwrap().0, DOWN));
        //         } else if faces[i].down.is_some()
        //             && faces[faces[i].down.unwrap().0].left.is_some()
        //             && faces[faces[faces[i].down.unwrap().0].left.unwrap().0].left.is_some()
        //             && faces[faces[faces[i].down.unwrap().0].left.unwrap().0].left.unwrap().0 != i {
        //             faces[i].up = Some((faces[faces[faces[i].down.unwrap().0].left.unwrap().0].left.unwrap().0, DOWN));
        //         } else if faces[i].down.is_some()
        //             && faces[faces[i].down.unwrap().0].right.is_some()
        //             && faces[faces[faces[i].down.unwrap().0].right.unwrap().0].right.is_some()
        //             && faces[faces[faces[i].down.unwrap().0].right.unwrap().0].right.unwrap().0 != i {
        //             faces[i].up = Some((faces[faces[faces[i].down.unwrap().0].right.unwrap().0].right.unwrap().0, DOWN));
        //         }
        //     }
        // }
        dbg!(&faces);
        Self {
            map: map.clone(),
            face_size,
            faces,
        }
    }

    fn face_index(&self, position: &Position) -> Option<usize> {
        self.faces.iter().position(|f|
            f.top_left.0 <= position.0 && f.top_left.1 <= position.1 &&
                f.bottom_right.0 >= position.0 && f.bottom_right.1 >= position.1)
    }
}

impl Surface for Cube {
    fn starting_position(&self) -> Position {
        (0, self.map[0].iter().position(|c| c == &'.').unwrap())
    }

    fn move_forward(&self, character: &Character) -> Character {
        let mut moving = character.clone();
        let initial_face = self.face_index(&moving.position);
        if initial_face.is_none() {
            panic!("initial face none");
        }
        match character.direction {
            RIGHT => { moving.position.1 += 1; }
            DOWN => { moving.position.0 += 1; }
            LEFT => { moving.position.1 -= 1; }
            UP => { moving.position.0 -= 1; }
            _ => panic!("unknown direction"),
        }
        if self.face_index(&moving.position) == initial_face {
            return if self.map[moving.position.0][moving.position.1] == '#' { character.clone() } else { moving };
        }
        let face = &self.faces[initial_face.unwrap()];
        let (next_face_index, next_direction) = match character.direction {
            RIGHT => face.right,
            DOWN => face.down,
            LEFT => face.left,
            UP => face.up,
            _ => panic!("unknown direction"),
        }.unwrap();
        let next_face = &self.faces[next_face_index];
        moving.direction = next_direction;
        match character.direction {
            RIGHT => {
                let distance = face.bottom_right.0 - character.position.0;
                dbg!("right", distance);
                moving.position = match next_direction {
                    RIGHT => (character.position.0, character.position.1 + 1),
                    DOWN => (next_face.top_left.0, next_face.top_left.1 + distance),
                    LEFT => (next_face.top_left.0 + distance, next_face.bottom_right.1),
                    UP => (next_face.bottom_right.0, next_face.bottom_right.1 - distance),
                    _ => panic!("unknown direction"),
                };
            }
            DOWN => {
                let distance = face.bottom_right.1 - character.position.1;
                dbg!("down", distance);
                moving.position = match next_direction {
                    RIGHT => (next_face.top_left.0 + distance, next_face.top_left.1),
                    DOWN => (next_face.top_left.0, next_face.bottom_right.1 - distance),
                    LEFT => (next_face.bottom_right.0 - distance, next_face.bottom_right.1),
                    UP => (next_face.bottom_right.0, next_face.top_left.1 + distance),
                    _ => panic!("unknown direction"),
                };
            }
            LEFT => {
                let distance = face.bottom_right.0 - character.position.0;
                dbg!("left", distance);
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
                dbg!("up", distance);
                dbg!(face.index, next_face.index, next_direction);
                moving.position = match next_direction {
                    RIGHT => (next_face.top_left.0 + distance, next_face.top_left.1),
                    DOWN => (next_face.bottom_right.0, next_face.bottom_right.1 - distance),
                    LEFT => (next_face.bottom_right.0 - distance, next_face.bottom_right.1),
                    UP => (next_face.bottom_right.0, next_face.top_left.1 + distance),
                    _ => panic!("unknown direction"),
                };
            }
            _ => panic!("unknown direction"),
        }
        if self.map[moving.position.0][moving.position.1] == '#' { character.clone() } else { moving }
    }

    fn pp(&self, character: &Character) -> () {
        pp(&self.map, &character);
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
        write!(f, "Character [({}, {}) => {}]", self.position.0, self.position.1, self.direction)
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
            Some('L') => { return Self { position: self.position, direction: dec_direction(self.direction) }; }
            Some('R') => { return Self { position: self.position, direction: inc_direction(self.direction) }; }
            None => {}
            _ => panic!("unknown direction"),
        }
        let mut character = self.clone();
        // surface.pp(&character);
        for _ in 0..instruction.distance.unwrap() {
            let tmp_character = surface.move_forward(&character);
            if tmp_character == character { break; }
            character = tmp_character;
            // surface.pp(&character);
        }
        character
    }
}
