use crate::solver::AdventSolver;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from(c: char) -> Direction {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("unknown"),
        }
    }
}

struct Shape {
    height: usize,
    width: usize,
    occupies: Vec<(usize, usize)>,
}

pub struct Advent2022Day17Solver {
    jet_pattern: Vec<Direction>,
    shapes: Vec<Shape>,
}

impl Advent2022Day17Solver {
    pub fn new(input: String) -> Self {
        Self {
            jet_pattern: input
                .chars()
                .map(Direction::from)
                .collect(),
            shapes: vec!(
                Shape { height: 1, width: 4, occupies: vec!((0, 0), (0, 1), (0, 2), (0, 3)) },
                Shape { height: 3, width: 3, occupies: vec!((0, 1), (1, 0), (1, 1), (1, 2), (2, 1)) },
                Shape { height: 3, width: 3, occupies: vec!((0, 2), (1, 2), (2, 0), (2, 1), (2, 2)) },
                Shape { height: 4, width: 1, occupies: vec!((0, 0), (1, 0), (2, 0), (3, 0)) },
                Shape { height: 2, width: 2, occupies: vec!((0, 0), (0, 1), (1, 0), (1, 1)) },
            ),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum State {
    Empty,
    Falling,
    Resting,
}

impl State {
    fn _print(&self) -> char {
        match self {
            State::Empty => '.',
            State::Falling => '@',
            State::Resting => '#',
        }
    }
}

struct Chamber {
    rows: Vec<[State; 7]>,
    jet_pattern: Vec<Direction>,
    current_jet: usize,
    truncated_rows: usize,
}

impl Chamber {
    fn new(jet_pattern: &Vec<Direction>) -> Self {
        Self {
            rows: vec!([State::Resting; 7]),
            jet_pattern: jet_pattern.clone(),
            current_jet: jet_pattern.len() - 1,
            truncated_rows: 0,
        }
    }

    fn next_jet(&mut self) -> Direction {
        self.current_jet += 1;
        if self.current_jet == self.jet_pattern.len() {
            self.current_jet = 0;
        }
        self.jet_pattern[self.current_jet]
    }

    fn extend(&mut self, count: usize) -> usize {
        (0..(count + 3)).for_each(|_| self.rows.push([State::Empty; 7]));
        self.rows.len() - 1
    }

    fn shrink(&mut self) -> () {
        while self.rows.last().unwrap().iter().all(|s| *s == State::Empty) {
            self.rows.pop();
        }
    }

    fn _pp(&self) -> () {
        for row in self.rows.iter().rev() {
            println!("{}", row.iter().map(State::_print).collect::<String>());
        }
        println!();
    }

    fn switch_state(&mut self, shape: &Shape, top: usize, left: usize, state: State) -> () {
        shape.occupies
            .iter()
            .map(|(r, c)| (top - r, c + left))
            .for_each(|(r, c)| self.rows[r][c] = state);
    }

    fn move_sideways(&mut self, shape: &Shape, top: usize, left: usize) -> usize {
        match self.next_jet() {
            Direction::Left => {
                if left == 0 { return left; }
                if self.rests(shape, top, left - 1) { return left; }
                self.switch_state(shape, top, left, State::Empty);
                self.switch_state(shape, top, left - 1, State::Falling);
                left - 1
            }
            Direction::Right => {
                if left + shape.width == 7 { return left; }
                if self.rests(shape, top, left + 1) { return left; }
                self.switch_state(shape, top, left, State::Empty);
                self.switch_state(shape, top, left + 1, State::Falling);
                left + 1
            }
        }
    }

    fn rests(&self, shape: &Shape, top: usize, left: usize) -> bool {
        shape.occupies
            .iter()
            .map(|(r, c)| (top - r, c + left))
            .any(|(r, c)| self.rows[r][c] == State::Resting)
    }

    fn drop(&mut self, shape: &Shape) -> () {
        let mut top = self.extend(shape.height);
        let mut left = 2;
        self.switch_state(shape, top, left, State::Falling);
        loop {
            left = self.move_sideways(shape, top, left);
            if self.rests(shape, top - 1, left) {
                self.switch_state(shape, top, left, State::Resting);
                self.shrink();
                break;
            }
            self.switch_state(shape, top, left, State::Empty);
            top -= 1;
            self.switch_state(shape, top, left, State::Falling);
        }
    }
}

impl AdventSolver for Advent2022Day17Solver {
    fn solve_part1(&self) -> usize {
        let mut chamber = Chamber::new(&self.jet_pattern);
        let mut shape_index = 0;
        for _ in 0..2022 {
            chamber.drop(&self.shapes[shape_index]);
            shape_index += 1;
            if shape_index == self.shapes.len() {
                shape_index = 0;
            }
        }
        chamber.rows.len() - 1
    }

    fn solve_part2(&self) -> usize {
        const LIMIT: usize = 1000000000000;
        let mut chamber = Chamber::new(&self.jet_pattern);
        let mut shape_index = 0;
        let mut i = 0;
        let mut found = false;
        let mut indexes: Vec<(usize, usize, usize, usize)> = vec!();
        while i < LIMIT {
            chamber.drop(&self.shapes[shape_index]);
            match indexes.iter().position(|is| is.0 == shape_index && is.1 == chamber.current_jet) {
                None => {}
                Some(px) => {
                    let x = indexes[px];
                    if !found && (1..=5).all(|t| indexes[px - t].0 == indexes[i - t].0 && indexes[px - t].1 == indexes[i - t].1)
                    {
                        found = true;
                        let length = i - x.3;
                        let row_length = chamber.rows.len() - x.2;
                        let repetitions = (LIMIT - i) / length - 1;
                        i += length * repetitions;
                        chamber.truncated_rows = row_length * repetitions;
                    }
                }
            }
            indexes.push((shape_index, chamber.current_jet, chamber.rows.len(), i));
            shape_index += 1;
            if shape_index == self.shapes.len() {
                shape_index = 0;
            }
            i += 1;
        }
        chamber.rows.len() - 1 + chamber.truncated_rows
    }
}
