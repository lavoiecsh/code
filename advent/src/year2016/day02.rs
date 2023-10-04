use crate::solver::AdventSolver;

pub struct Advent2016Day02Solver {
    movements: Vec<String>,
}

impl Advent2016Day02Solver {
    pub fn new(input: String) -> Self {
        Self {
            movements: input.lines().map(|l| l.to_string()).collect()
        }
    }
}

impl AdventSolver for Advent2016Day02Solver {
    fn solve_part1_string(&self) -> String {
        let mut keypad = Keypad { current: '5', movement_fn: part1_keypad_movement };
        self.movements.iter()
            .map(|m| keypad.next_character(m))
            .collect()
    }

    fn solve_part2_string(&self) -> String {
        let mut keypad = Keypad { current: '5', movement_fn: part2_keypad_movement };
        self.movements.iter()
            .map(|m| keypad.next_character(m))
            .collect()
    }
}

struct Keypad {
    current: char,
    movement_fn: fn (char, char) -> char,
}

impl Keypad {
    fn next_character(&mut self, movements: &str) -> char {
        movements.chars().for_each(|c| self.current = (self.movement_fn)(self.current, c));
        self.current
    }
}

fn part1_keypad_movement(current: char, movement: char) -> char {
    match (current, movement) {
        ('1', 'D') => '4',
        ('1', 'R') => '2',
        ('2', 'D') => '5',
        ('2', 'L') => '1',
        ('2', 'R') => '3',
        ('3', 'D') => '6',
        ('3', 'L') => '2',
        ('4', 'U') => '1',
        ('4', 'D') => '7',
        ('4', 'R') => '5',
        ('5', 'U') => '2',
        ('5', 'D') => '8',
        ('5', 'L') => '4',
        ('5', 'R') => '6',
        ('6', 'U') => '3',
        ('6', 'D') => '9',
        ('6', 'L') => '5',
        ('7', 'U') => '4',
        ('7', 'R') => '8',
        ('8', 'U') => '5',
        ('8', 'L') => '7',
        ('8', 'R') => '9',
        ('9', 'U') => '6',
        ('9', 'L') => '8',
        _ => current,
    }
}

fn part2_keypad_movement(current: char, movement: char) -> char {
    match (current, movement) {
        ('1', 'D') => '3',
        ('2', 'D') => '6',
        ('2', 'R') => '3',
        ('3', 'U') => '1',
        ('3', 'D') => '7',
        ('3', 'L') => '2',
        ('3', 'R') => '4',
        ('4', 'D') => '8',
        ('4', 'L') => '3',
        ('5', 'R') => '6',
        ('6', 'U') => '2',
        ('6', 'D') => 'A',
        ('6', 'L') => '5',
        ('6', 'R') => '7',
        ('7', 'U') => '3',
        ('7', 'D') => 'B',
        ('7', 'L') => '6',
        ('7', 'R') => '8',
        ('8', 'U') => '4',
        ('8', 'D') => 'C',
        ('8', 'L') => '7',
        ('8', 'R') => '9',
        ('9', 'L') => '8',
        ('A', 'U') => '6',
        ('A', 'R') => 'B',
        ('B', 'U') => '7',
        ('B', 'D') => 'D',
        ('B', 'L') => 'A',
        ('B', 'R') => 'C',
        ('C', 'U') => '8',
        ('C', 'L') => 'B',
        ('D', 'U') => 'B',
        _ => current,
    }
}
