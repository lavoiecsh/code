use crate::solver::AdventSolver;
use itertools::Itertools;
use regex::{Match, Regex};
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};

pub struct Advent2018Day17Solver {
    map: Map,
}

impl Advent2018Day17Solver {
    pub fn new(input: String) -> Self {
        let x_re = Regex::new(r"x=(\d+), y=(\d+)\.\.(\d+)").unwrap();
        let y_re = Regex::new(r"y=(\d+), x=(\d+)\.\.(\d+)").unwrap();
        let as_num = |m: Option<Match>| m.unwrap().as_str().parse::<usize>().unwrap();
        let mut map = Map::new(input.lines()
                .flat_map(|l| {
                    if let Some(cap) = x_re.captures(l) {
                        let x = as_num(cap.get(1));
                        (as_num(cap.get(2))..=as_num(cap.get(3)))
                            .map(|y| (y, x))
                            .collect_vec()
                    } else if let Some(cap) = y_re.captures(l) {
                        let y = as_num(cap.get(1));
                        (as_num(cap.get(2))..=as_num(cap.get(3)))
                            .map(|x| (y, x))
                            .collect_vec()
                    } else {
                        panic!("invalid line")
                    }
                })
                .collect());
        while !map.is_done() { map.step(); }
        Self { map }
    }
}

impl AdventSolver for Advent2018Day17Solver {
    fn solve_part1(&self) -> usize {
        self.map.count_water()
    }

    fn solve_part2(&self) -> usize {
        self.map.count_resting()
    }
}

type Pos = (usize, usize);

#[derive(Debug, Clone)]
enum State {
    Clay,
    Rest,
    Flow,
    Empty,
}

struct Map {
    map: Vec<Vec<State>>,
    flowing: Vec<Pos>,
    min_y: usize,
    max_y: usize,
}

impl Map {
    fn new(clay: HashSet<Pos>) -> Self {
        let (&min_y, &max_y) = clay.iter().map(|(y, _)| y).minmax().into_option().unwrap();
        let &max_x = clay.iter().map(|(_, x)| x).max().unwrap();
        let mut map = vec![vec![State::Empty; max_x + 2]; max_y + 2];
        clay.iter()
            .for_each(|&(y, x)| map[y][x] = State::Clay);
        map[0][500] = State::Flow;
        Self { map, flowing: vec![(0, 500)], min_y, max_y }
    }

    fn step(&mut self) {
        if let Some((y, x)) = self.flowing.pop() {
            if y >= self.map.len() - 1 {
                return;
            }
            if matches!(self.map[y + 1][x], State::Flow) {
                return;
            }
            if matches!(self.map[y + 1][x], State::Empty) {
                self.map[y + 1][x] = State::Flow;
                self.flowing.push((y, x));
                self.flowing.push((y + 1, x));
                return;
            }
            let mut xl = x - 1;
            while matches!(self.map[y+1][xl], State::Clay | State::Rest) && matches!(self.map[y][xl], State::Empty) {
                xl -= 1;
            }
            let mut xr = x + 1;
            while matches!(self.map[y+1][xr], State::Clay | State::Rest) && matches!(self.map[y][xr], State::Empty) {
                xr += 1;
            }
            match (&self.map[y][xl], &self.map[y][xr]) {
                (State::Empty, State::Empty) => {
                    (xl..=xr).for_each(|x| self.map[y][x] = State::Flow);
                    self.flowing.push((y, xl));
                    self.flowing.push((y, xr));
                }
                (State::Empty, State::Clay) => {
                    (xl..=xr-1).for_each(|x| self.map[y][x] = State::Flow);
                    self.flowing.push((y, xl));
                }
                (State::Clay, State::Empty) => {
                    (xl+1..=xr).for_each(|x| self.map[y][x] = State::Flow);
                    self.flowing.push((y, xr));
                }
                (State::Clay, State::Clay) => {
                    (xl+1..=xr-1).for_each(|x| self.map[y][x] = State::Rest);
                }
                (State::Flow, State::Empty) => {
                    (xl+1..=xr).for_each(|x| self.map[y][x] = State::Flow);
                    self.flowing.push((y, xr));
                }
                (State::Empty, State::Flow) => {
                    (xl..=xr-1).for_each(|x| self.map[y][x] = State::Flow);
                    self.flowing.push((y, xl));
                }
                (State::Flow, State::Clay) => {
                    (xl+1..=xr-1).for_each(|x| self.map[y][x] = State::Flow);
                }
                (State::Clay, State::Flow) => {
                    (xl+1..=xr-1).for_each(|x| self.map[y][x] = State::Flow);
                }
                (State::Flow, State::Flow) => {
                    (xl+1..=xr-1).for_each(|x| self.map[y][x] = State::Flow);
                }
                (left, right) => {
                    panic!("invalid state {:?} {:?}", left, right);
                }
            }
            let mut wall = None;
            for x in 0..self.map[y].len() {
                if wall.is_none() {
                    if matches!(self.map[y][x], State::Clay) {
                        wall = Some(x);
                    }
                    continue;
                }
                if matches!(self.map[y][x], State::Flow) && matches!(self.map[y+1][x], State::Rest | State::Clay) {
                    
                } else if matches!(self.map[y][x], State::Clay) {
                    (wall.unwrap()+1..x).for_each(|x| self.map[y][x] = State::Rest);
                    wall = None;
                } else {
                    wall = None;
                }
            }
        }
    }
    
    fn is_done(&self) -> bool {
        self.flowing.is_empty()
    }

    fn count_water(&self) -> usize {
        (self.min_y..=self.max_y)
            .map(|y| self.map[y].iter().filter(|&s| matches!(s, State::Flow | State::Rest)).count())
            .sum()
    }
    
    fn count_resting(&self) -> usize {
        (self.min_y..=self.max_y)
            .map(|y| self.map[y].iter().filter(|&s| matches!(s, State::Rest)).count())
            .sum()
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x_start = self.map.iter().map(|row| row.iter().position(|s| !matches!(s, State::Empty)).unwrap_or(row.len())).min().unwrap() - 1;
        let mut board = String::with_capacity((self.map.len() + 4) * (self.map[0].len() + 10));
        board.push('\n');
        for (row_index, row)  in self.map.iter().enumerate() {
            board.push_str(&format!("{:4}  ", row_index));
            for col in row.iter().skip(x_start) {
                board.push(match col {
                    State::Clay => '#',
                    State::Rest => '~',
                    State::Flow => '|',
                    State::Empty => '.',
                });
            }
            board.push('\n');
        }
        board.push_str(&format!("Flowing: {} -> ", self.flowing.len()));
        self.flowing.iter()
            .for_each(|(y,x)| board.push_str(&format!("({},{}), ", y, x)));
        board.push('\n');
        f.write_str(&board)
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::AdventSolver;
    use crate::year2018::day17::Advent2018Day17Solver;

    fn example() -> Advent2018Day17Solver {
        Advent2018Day17Solver::new(String::from("\
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
"))
    }

    #[test]
    fn finds_number_of_water() {
        assert_eq!(example().solve_part1(), 57);
    }
}
