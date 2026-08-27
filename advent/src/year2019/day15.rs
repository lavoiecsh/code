use crate::solver::AdventSolver;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter, Write};
use itertools::Itertools;
use num_traits::Inv;
use crate::year2019::intcode::{Computer, Value};

type Pos = (Value, Value);
pub struct Advent2019Day15Solver {
    remote_control: RemoteControl,
}

impl Advent2019Day15Solver {
    pub fn new(input: &str) -> Self {
        let program = input.split(',').map(|n| n.parse().unwrap()).collect();
        let computer = Computer::new(program);
        let mut remote_control = RemoteControl::new(computer);
        remote_control.build_map();
        Self { remote_control }
    }
}

impl AdventSolver for Advent2019Day15Solver {
    fn solve_part1(&self) -> usize {
        self.remote_control.shortest_path_to_oxygen()
    }

    fn solve_part2(&self) -> usize {
        self.remote_control.time_to_fill_space()
    }
}

struct RemoteControl {
    computer: Computer,
    map: HashMap<Pos, Tile>,
    droid: Pos,
    oxygen: Option<Pos>,
}

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Oxygen,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl RemoteControl {
    fn new(computer: Computer) -> Self {
        let mut map = HashMap::new();
        map.insert((0, 0), Tile::Empty);
        Self {
            computer,
            map,
            droid: (0, 0),
            oxygen: None,
        }
    }

    fn build_map(&mut self) {
        let mut to_visit = VecDeque::new();
        to_visit.extend(self.next_directions());
        while let Some((pos, direction)) = to_visit.pop_front() {
            if self.map.contains_key(&direction.move_pos(pos)) {
                continue;
            }
            self.move_to(pos);
            if self.move_droid(direction) {
                to_visit.extend(self.next_directions());
            }
        }
    }

    fn next_directions(&self) -> impl Iterator<Item = (Pos, Direction)> {
        Direction::iter()
            .filter(|d| !self.map.contains_key(&d.move_pos(self.droid)))
            .map(|d| (self.droid, d))
    }

    fn move_to(&mut self, pos: Pos) {
        self.shortest_path_to(pos)
            .into_iter()
            .for_each(|d| { self.move_droid(d); })
    }

    fn move_droid(&mut self, direction: Direction) -> bool {
        let next_pos = direction.move_pos(self.droid);
        self.computer.send_input(direction.into());
        self.computer.run();
        match self.computer.receive_output().unwrap() {
            0 => {
                self.map.insert(next_pos, Tile::Wall);
                false
            },
            1 => {
                self.map.insert(next_pos, Tile::Empty);
                self.droid = next_pos;
                true
            },
            2 => {
                self.map.insert(next_pos, Tile::Oxygen);
                self.droid = next_pos;
                self.oxygen = Some(next_pos);
                true
            },
            _status => unreachable!("unknown status code {_status}"),
        }
    }

    fn shortest_path_to_oxygen(&self) -> usize {
        *self.distance_map_from((0, 0))
            .get(&self.oxygen.unwrap())
            .unwrap()
    }

    fn time_to_fill_space(&self) -> usize {
        *self.distance_map_from(self.oxygen.unwrap())
            .values()
            .max()
            .unwrap()
    }

    fn distance_map_from(&self, start: Pos) -> HashMap<Pos, usize> {
        let mut distances = HashMap::new();
        distances.insert(start, 0);
        let mut queue = VecDeque::new();
        queue.extend(self.walkable_tiles_around(start).map(|p| (p, 1)));
        while let Some((p, d)) = queue.pop_front() {
            if let Some(c) = distances.get(&p) && c < &d {
                continue;
            }
            distances.insert(p, d);
            queue.extend(self.walkable_tiles_around(p).map(|n| (n, d + 1)));
        }
        distances
    }

    fn shortest_path_to(&self, pos: Pos) -> Vec<Direction> {
        let distances = self.distance_map_from(self.droid);
        let mut path = vec![];
        let mut current_pos = pos;
        let mut current_distance = *distances.get(&current_pos).unwrap();
        while current_distance != 0 {
            current_distance -= 1;
            let direction = Direction::iter()
                .find(|d| distances.get(&d.move_pos(current_pos))
                    .map_or(false, |&d| d == current_distance))
                .unwrap();
            current_pos = direction.move_pos(current_pos);
            path.push(direction.inv());
        }
        path.reverse();
        path
    }

    fn walkable_tiles_around(&self, pos: Pos) -> impl Iterator<Item = Pos> {
        Direction::iter()
            .map(move |d| d.move_pos(pos))
            .filter(|d| self.map.get(&d).map_or(false, Tile::is_walkable))
    }
}

impl Direction {
    fn move_pos(&self, pos: Pos) -> Pos {
        match self {
            Direction::North => (pos.0, pos.1 - 1),
            Direction::South => (pos.0, pos.1 + 1),
            Direction::East => (pos.0 - 1, pos.1),
            Direction::West => (pos.0 + 1, pos.1),
        }
    }

    fn iter() -> impl Iterator<Item = Direction> {
        vec![Direction::North, Direction::East, Direction::South, Direction::West].into_iter()
    }
}

impl Tile {
    fn is_walkable(&self) -> bool {
        matches!(self, Tile::Empty | Tile::Oxygen)
    }
}

impl Inv for Direction {
    type Output = Direction;

    fn inv(self) -> Self::Output {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl Into<Value> for Direction {
    fn into(self) -> Value {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Direction::North => 'N',
            Direction::South => 'S',
            Direction::East => 'E',
            Direction::West => 'W',
        })
    }
}

impl From<&Tile> for char {
    fn from(value: &Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Oxygen => '0',
        }
    }
}

impl Debug for RemoteControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x) = self.map.keys().map(|&(x,_)| x).minmax().into_option().unwrap();
        let (min_y, max_y) = self.map.keys().map(|&(_,y)| y).minmax().into_option().unwrap();
        f.write_char('\n')?;
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.droid == (x, y) {
                    f.write_char('D')?;
                    continue;
                }
                f.write_char(self.map.get(&(x, y)).map_or(' ', char::from))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}
