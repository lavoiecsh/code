use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::solver::AdventSolver;

pub struct Advent2023Day16Solver {
    contraption: Contraption,
}

impl Advent2023Day16Solver {
    pub fn new(input: String) -> Self {
        Self {
            contraption: Contraption::new(input.lines().map(|l| l.chars().collect()).collect())
        }
    }
}

impl AdventSolver for Advent2023Day16Solver {
    fn solve_part1(&self) -> usize {
        let mut contraption = self.contraption.clone();
        contraption.energized_squares((0, 0, Right))
    }

    fn solve_part2(&self) -> usize {
        let mut contraption = self.contraption.clone();
        let mut highest = 0;
        for y in 0..=contraption.max_y {
            let right = contraption.energized_squares((0, y, Right));
            if right > highest { highest = right; }
            let left = contraption.energized_squares((contraption.max_x, y, Left));
            if left > highest { highest = left; }
        }
        for x in 0..=contraption.max_x {
            let down = contraption.energized_squares((x, 0, Down));
            if down > highest { highest = down; }
            let up = contraption.energized_squares((x, contraption.max_y, Up));
            if up > highest { highest = up; }
        }
        highest
    }
}

#[derive(Clone)]
struct Contraption {
    grid: Vec<Vec<char>>,
    max_x: usize,
    max_y: usize,
    energized: HashMap<Beam, HashSet<Beam>>
}

impl Contraption {
    fn new(grid: Vec<Vec<char>>) -> Self {
        Self { max_x: grid[0].len() - 1, max_y: grid.len() - 1, grid, energized: HashMap::new() }
    }

    fn energized_squares(&mut self, beam: Beam) -> usize {
        let mut seen: HashSet<Beam> = HashSet::new();
        let mut beams: Vec<Beam> = vec!(beam);
        while let Some(current) = beams.pop() {
            if seen.iter().contains(&current) { continue; }
            seen.insert(current);
            beams.extend(self.calculate_next(&current));
        }
        let squares = seen.iter()
            .map(|b| (b.0, b.1))
            .unique()
            .count();
        squares
    }

    fn energized_squares_rec(&mut self, beam: &Beam, ignore_list: &HashSet<Beam>) -> Option<&HashSet<Beam>> {
        if self.energized.contains_key(beam) {
            return self.energized.get(beam);
        }
        if ignore_list.contains(&beam) {
            return None;
        }

        let next = self.calculate_next(&beam);
        if next.is_empty() {
            let mut hs = HashSet::new();
            hs.insert(*beam);
            self.energized.insert(*beam, hs);
            return self.energized.get(beam);
        }
        let mut next_ignore_list = ignore_list.clone();
        next_ignore_list.insert(beam.clone());
        if next.len() == 1 {
            if let Some(nhs) = self.energized_squares_rec(&next[0], &next_ignore_list) {
                let mut hs = nhs.clone();
                hs.insert(*beam);
                self.energized.insert(*beam, hs);
                return self.energized.get(beam);
            }
            return None;
        }
        let nhs1 = self.energized_squares_rec(&next[0], &next_ignore_list).map(|hs| hs.clone());
        let nhs2 = self.energized_squares_rec(&next[1], &next_ignore_list).map(|hs| hs.clone());
        if nhs1.is_none() || nhs2.is_some() {
            return None;
        }
        let mut hs = HashSet::new();
        if nhs1.is_some() {
            hs.extend(nhs1.unwrap());
        }
        if nhs2.is_some() {
            hs.extend(nhs2.unwrap());
        }
        hs.insert(*beam);
        self.energized.insert(*beam, hs);
        return self.energized.get(beam);
    }

    fn energized_squares_2(&mut self, beam: &Beam) -> usize {
        let solution = self.energized_squares_rec(beam, &HashSet::new()).unwrap();
        dbg!(beam, solution);
        solution.iter().map(|b| (b.0, b.1)).unique().count()
    }

    fn calculate_next(&self, (x, y, d): &Beam) -> Vec<Beam> {
        match (self.grid[*y][*x], *d) {
            ('.', Right) | ('-', Right) => if *x < self.max_x { vec!((x + 1, *y, Right)) } else { vec!() },
            ('.', Down) | ('|', Down) => if *y < self.max_y { vec!((*x, y + 1, Down)) } else { vec!() },
            ('.', Left) | ('-', Left) => if *x > 0 { vec!((x - 1, *y, Left)) } else { vec!() },
            ('.', Up) | ('|', Up) => if *y > 0 { vec!((*x, y - 1, Up)) } else { vec!() },
            ('|', Right) | ('|', Left) => {
                let mut next = vec!();
                if *y > 0 { next.push((*x, y - 1, Up)); }
                if *y < self.max_y { next.push((*x, y + 1, Down)); }
                next
            }
            ('-', Down) | ('-', Up) => {
                let mut next = vec!();
                if *x > 0 { next.push((x - 1, *y, Left)); }
                if *x < self.max_x { next.push((x + 1, *y, Right)); }
                next
            }
            ('/', Right) | ('\\', Left) => if *y > 0 { vec!((*x, y - 1, Up)) } else { vec!() }
            ('/', Up) | ('\\', Down) => if *x < self.max_x { vec!((x + 1, *y, Right)) } else { vec!() }
            ('/', Left) | ('\\', Right) => if *y < self.max_y { vec!((*x, y + 1, Down)) } else { vec!() }
            ('/', Down) | ('\\', Up) => if *x > 0 { vec!((x - 1, *y, Left)) } else { vec!() }
            _ => vec!()
        }
    }
}

type Beam = (usize, usize, Direction);

use Direction::*;

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day16Solver {
    Advent2023Day16Solver::new(String::from("\
.|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
"))
}

#[test]
fn tmp() {
    let solver = test_solver_1();
    let mut contraption = solver.contraption.clone();
    assert_eq!(contraption.energized_squares_2(&(0, 0, Right)), 46);
}

#[test]
fn counts_energized_squares() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part1(), 46);
}

#[test]
fn finds_best_energizing() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part2(), 51);
}
