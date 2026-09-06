use crate::solver::AdventSolver;
use crate::year2019::intcode::{Computer, Value};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Formatter, Write};

pub struct Advent2019Day17Solver {
    program: Vec<Value>,
}

impl Advent2019Day17Solver {
    pub fn new(input: &str) -> Self {
        Self {
            program: input.split(',').map(|n| n.parse().unwrap()).collect(),
        }
    }
}

impl AdventSolver for Advent2019Day17Solver {
    fn solve_part1(&self) -> usize {
        let computer = Computer::new(self.program.clone());
        let interface = ScaffoldInterface::new(computer);
        let intersections = interface.intersections();
        intersections.into_iter().map(|(y, x)| y * x).sum()
    }

    fn solve_part2(&self) -> usize {
        let computer = Computer::new(self.program.clone());
        let mut interface = ScaffoldInterface::new(computer);
        interface.solve()
    }
}

type Pos = (usize, usize);

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct ScaffoldInterface {
    computer: Computer,
    scaffolds: Vec<Vec<bool>>,
    robot: (Pos, Direction),
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Movement {
    left_turn: bool,
    distance: usize,
}

#[derive(Clone)]
struct Path {
    functions: Vec<MovementGroup>,
    starting_main_function: MainFunction,
}

#[derive(Clone, PartialEq)]
struct MainFunction {
    function_calls: Vec<usize>,
}

impl MainFunction {
    fn is_solution(&self) -> bool {
        self.function_calls.len() <= 10 && self.sub_function_set().len() <= 3
    }

    fn to_ascii(&self) -> (usize, usize, usize, Vec<Value>) {
        let mut set = self.sub_function_set().into_iter();
        let a = set.next().unwrap();
        let b = set.next().unwrap();
        let c = set.next().unwrap();
        (
            a,
            b,
            c,
            self.function_calls
                .iter()
                .map(|&x| {
                    if x == a {
                        ascii::A
                    } else if x == b {
                        ascii::B
                    } else if x == c {
                        ascii::C
                    } else {
                        unreachable!("too many functions")
                    }
                })
                .collect(),
        )
    }

    fn sub_function_set(&self) -> HashSet<usize> {
        self.function_calls
            .iter()
            .cloned()
            .collect::<HashSet<usize>>()
    }
}

impl Path {
    fn new(movements: Vec<Movement>) -> Self {
        let mut functions = vec![];
        let mut main = vec![];
        for movement in movements {
            let function = MovementGroup::new(vec![movement]);
            if let Some(index) = functions.iter().position(|f| f == &function) {
                main.push(index);
            } else {
                main.push(functions.len());
                functions.push(function);
            }
        }
        Self {
            functions,
            starting_main_function: MainFunction {
                function_calls: main,
            },
        }
    }

    fn simplify(&mut self) -> MainFunction {
        let mut main_functions = VecDeque::new();
        main_functions.push_back(self.starting_main_function.clone());
        while let Some(main) = main_functions.pop_front() {
            if main.is_solution() {
                return main;
            }
            let new_functions = self
                .simplify_step(main)
                .into_iter()
                .filter(|m| !main_functions.contains(m))
                .collect_vec();
            main_functions.extend(new_functions);
        }
        dbg!(&self.functions);
        unreachable!("no solution found")
    }

    fn simplify_step(&mut self, main: MainFunction) -> Vec<MainFunction> {
        let mut next_functions = vec![];
        let new_functions = main
            .function_calls
            .iter()
            .cloned()
            .tuple_windows()
            .collect_vec()
            .into_iter()
            .map(|(i, j)| (i, j, self.functions[i].join(&self.functions[j])))
            .filter(|(_, _, f)| f.ascii.len() <= 20)
            .collect_vec();
        for (i, j, new_function) in new_functions {
            let index = self
                .functions
                .iter()
                .position(|f| f == &new_function)
                .unwrap_or(self.functions.len());
            self.functions.push(new_function);
            let mut function_calls = vec![];
            let mut x = 0;
            while x < main.function_calls.len() {
                if x < main.function_calls.len() - 1
                    && main.function_calls[x] == i
                    && main.function_calls[x + 1] == j
                {
                    function_calls.push(index);
                    x += 1;
                } else {
                    function_calls.push(main.function_calls[x]);
                }
                x += 1;
            }
            next_functions.push(MainFunction { function_calls })
        }
        next_functions
    }
}

#[derive(Clone, Eq, PartialEq)]
struct MovementGroup {
    movements: Vec<Movement>,
    ascii: Vec<Value>,
}

impl MovementGroup {
    fn new(movements: Vec<Movement>) -> Self {
        Self {
            ascii: ascii::movements_to_ascii(&movements),
            movements,
        }
    }

    fn join(&self, other: &MovementGroup) -> Self {
        let mut movements = self.movements.clone();
        movements.extend(other.movements.clone());
        Self::new(movements)
    }
}

impl ScaffoldInterface {
    fn intersections(&self) -> Vec<Pos> {
        let mut intersections = vec![];
        for y in 1..self.scaffolds.len() - 1 {
            for x in 1..self.scaffolds[y].len() - 1 {
                if self.scaffolds[y][x]
                    && self.scaffolds[y - 1][x]
                    && self.scaffolds[y + 1][x]
                    && self.scaffolds[y][x - 1]
                    && self.scaffolds[y][x + 1]
                {
                    intersections.push((y, x));
                }
            }
        }
        intersections
    }

    fn solve(&mut self) -> usize {
        let mut full_path = self.full_path();
        let main = full_path.simplify();
        let (a, b, c, ascii) = main.to_ascii();
        // main function
        self.computer
            .send_inputs(Itertools::intersperse(ascii.into_iter(), ascii::COMMA));
        self.computer.send_input(ascii::NEW_LINE);
        // function A
        self.computer
            .send_inputs(full_path.functions[a].ascii.iter().cloned());
        self.computer.send_input(ascii::NEW_LINE);
        // function B
        self.computer
            .send_inputs(full_path.functions[b].ascii.iter().cloned());
        self.computer.send_input(ascii::NEW_LINE);
        // function C
        self.computer
            .send_inputs(full_path.functions[c].ascii.iter().cloned());
        self.computer.send_input(ascii::NEW_LINE);
        // continuous video feed
        self.computer.send_input(ascii::NO);
        self.computer.send_input(ascii::NEW_LINE);

        self.run(false)
    }

    fn run(&mut self, print_output: bool) -> usize {
        self.computer.run();
        while let Some(output) = self.computer.receive_output() {
            if output > 127 {
                return output as usize;
            }
            if print_output {
                print!("{}", output as u8 as char);
            }
        }
        unreachable!("computer didn't output solution");
    }

    fn full_path(&self) -> Path {
        let mut path = vec![];
        let mut direction = Direction::Up;
        let mut position = self.robot.0;
        while let Some((next_direction, left_turn)) = self.next_direction(position, direction) {
            direction = next_direction;
            let mut distance = 0;
            while let Some(next_position) = self.next_position(position, direction)
                && self.scaffolds[next_position.0][next_position.1]
            {
                position = next_position;
                distance += 1;
            }
            path.push(Movement {
                left_turn,
                distance,
            });
        }
        Path::new(path)
    }

    fn next_direction(&self, pos: Pos, direction: Direction) -> Option<(Direction, bool)> {
        match direction {
            Direction::Down => {
                if let Some(next_position) = self.next_position(pos, Direction::Left)
                    && self.scaffolds[next_position.0][next_position.1]
                {
                    return Some((Direction::Left, false));
                }
                if let Some(next_position) = self.next_position(pos, Direction::Right)
                    && self.scaffolds[next_position.0][next_position.1]
                {
                    return Some((Direction::Right, true));
                }
                None
            }
            Direction::Up => {
                if let Some(next_position) = self.next_position(pos, Direction::Left)
                    && self.scaffolds[next_position.0][next_position.1]
                {
                    return Some((Direction::Left, true));
                }
                if let Some(next_position) = self.next_position(pos, Direction::Right)
                    && self.scaffolds[next_position.0][next_position.1]
                {
                    return Some((Direction::Right, false));
                }
                None
            }
            Direction::Left => {
                if let Some(next_position) = self.next_position(pos, Direction::Up)
                    && self.scaffolds[next_position.0][next_position.1]
                {
                    return Some((Direction::Up, false));
                }
                if let Some(next_position) = self.next_position(pos, Direction::Down)
                    && self.scaffolds[next_position.0][next_position.1]
                {
                    return Some((Direction::Down, true));
                }
                None
            }
            Direction::Right => {
                if let Some(next_position) = self.next_position(pos, Direction::Up)
                    && self.scaffolds[next_position.0][next_position.1]
                {
                    return Some((Direction::Up, true));
                }
                if let Some(next_position) = self.next_position(pos, Direction::Down)
                    && self.scaffolds[next_position.0][next_position.1]
                {
                    return Some((Direction::Down, false));
                }
                None
            }
        }
    }

    fn next_position(&self, pos: Pos, direction: Direction) -> Option<Pos> {
        match direction {
            Direction::Left => {
                if pos.1 == 0 {
                    None
                } else {
                    Some((pos.0, pos.1 - 1))
                }
            }
            Direction::Right => {
                if pos.1 == self.scaffolds[pos.0].len() - 1 {
                    None
                } else {
                    Some((pos.0, pos.1 + 1))
                }
            }
            Direction::Up => {
                if pos.0 == 0 {
                    None
                } else {
                    Some((pos.0 - 1, pos.1))
                }
            }
            Direction::Down => {
                if pos.0 == self.scaffolds.len() - 1 {
                    None
                } else {
                    Some((pos.0 + 1, pos.1))
                }
            }
        }
    }

    fn new(mut computer: Computer) -> Self {
        computer.run();
        let mut scaffolds = vec![];
        let mut robot = None;
        let mut row = vec![];
        while let Some(output) = computer.receive_output() {
            match output {
                ascii::HASH => {
                    row.push(true);
                }
                ascii::DOT => {
                    row.push(false);
                }
                ascii::NEW_LINE => {
                    if !row.is_empty() {
                        scaffolds.push(row);
                    }
                    row = vec![];
                }
                ascii::LEFT => {
                    robot = Some(((scaffolds.len(), row.len()), Direction::Left));
                    row.push(true);
                }
                ascii::RIGHT => {
                    robot = Some(((scaffolds.len(), row.len()), Direction::Right));
                    row.push(true);
                }
                ascii::DOWN => {
                    robot = Some(((scaffolds.len(), row.len()), Direction::Down));
                    row.push(true);
                }
                ascii::UP => {
                    robot = Some(((scaffolds.len(), row.len()), Direction::Up));
                    row.push(true);
                }
                _ => unreachable!("unknown scaffold map character {output}"),
            }
        }
        Self {
            scaffolds,
            robot: robot.unwrap(),
            computer,
        }
    }
}

impl Debug for ScaffoldInterface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('\n')?;
        for y in 0..self.scaffolds.len() {
            for x in 0..self.scaffolds[y].len() {
                if self.robot.0 == (y, x) {
                    f.write_char(match self.robot.1 {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    })?;
                } else {
                    f.write_char(if self.scaffolds[y][x] { '#' } else { '.' })?;
                }
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')
    }
}

impl Debug for Movement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {}",
            if self.left_turn { 'L' } else { 'R' },
            self.distance
        ))
    }
}

impl Debug for MovementGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for m in &self.movements {
            m.fmt(f)?;
            f.write_char(' ')?;
        }
        f.write_fmt(format_args!("(length: {})", self.ascii.len()))
    }
}

impl Debug for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "main: {}",
            self.starting_main_function.function_calls.iter().join(" ")
        ))?;
        f.write_str("\nfunctions:")?;
        for i in 0..self.functions.len() {
            f.write_fmt(format_args!("\n  {i:2}: "))?;
            self.functions[i].fmt(f)?;
        }
        Ok(())
    }
}

impl Debug for MainFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.function_calls.iter().join(" ")))
    }
}

#[allow(unused)]
mod ascii {
    use crate::year2019::day17::Movement;
    use crate::year2019::intcode::Value;
    use itertools::Itertools;

    pub(super) const A: Value = 'A' as Value;
    pub(super) const B: Value = 'B' as Value;
    pub(super) const C: Value = 'C' as Value;
    pub(super) const DOT: Value = '.' as Value;
    pub(super) const HASH: Value = '#' as Value;
    pub(super) const COMMA: Value = ',' as Value;
    pub(super) const NEW_LINE: Value = 10;
    pub(super) const UP: Value = '^' as Value;
    pub(super) const DOWN: Value = 'v' as Value;
    pub(super) const LEFT: Value = '<' as Value;
    pub(super) const RIGHT: Value = '>' as Value;
    pub(super) const L: Value = 'L' as Value;
    pub(super) const R: Value = 'R' as Value;
    pub(super) const YES: Value = 'y' as Value;
    pub(super) const NO: Value = 'n' as Value;
    const ZERO: Value = '0' as Value;

    pub(super) fn number_to_ascii(number: usize) -> impl Iterator<Item = Value> {
        let mut digits = vec![];
        let mut number = number;
        while number > 0 {
            digits.push((number % 10) as Value + ZERO);
            number /= 10;
        }
        digits.reverse();
        digits.into_iter()
    }

    fn movement_to_ascii(movement: &Movement) -> Vec<Value> {
        let mut a = vec![if movement.left_turn { L } else { R }, COMMA];
        a.extend(number_to_ascii(movement.distance));
        a
    }

    pub(super) fn movements_to_ascii<'a>(movements: &Vec<Movement>) -> Vec<Value> {
        let mut values = movement_to_ascii(&movements[0]);
        for i in 1..movements.len() {
            values.push(COMMA);
            values.extend(movement_to_ascii(&movements[i]));
        }
        values
    }
}
