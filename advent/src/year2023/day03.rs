use std::collections::HashMap;

use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2023Day03Solver {
    schematic: Schematic,
}

impl Advent2023Day03Solver {
    pub fn new(input: String) -> Self {
        Self { schematic: Schematic::from(input.as_str()) }
    }
}

impl AdventSolver for Advent2023Day03Solver {
    fn solve_part1(&self) -> usize {
        self.schematic.list_parts().iter().sum()
    }

    fn solve_part2(&self) -> usize {
        self.schematic.gear_ratios().iter().sum()
    }
}

struct Schematic {
    digits: HashMap<Pos, usize>,
    symbols: HashMap<Pos, char>,
}

impl Schematic {
    fn gear_ratios(&self) -> Vec<usize> {
        let gears: Vec<&Pos> = self.symbols.iter()
            .filter(|(_, &c)| c == '*')
            .map(|(p, _)| p)
            .collect();
        let mut gear_ratios = Vec::new();
        for gear in gears {
            let adjacents: Vec<Pos> = gear.adjacents().into_iter().filter(|a| self.digits.contains_key(a)).collect();
            if adjacents.len() < 2 { continue; }
            let parts: Vec<(usize, Vec<Pos>)> = adjacents.iter().map(|a| self.extract_part(a)).unique().collect();
            if parts.len() != 2 { continue; }
            gear_ratios.push(parts[0].0 * parts[1].0);
        }
        gear_ratios
    }

    fn list_parts(&self) -> Vec<usize> {
        let mut positions: Vec<(&Pos, char)> = self.symbols.iter()
            .flat_map(|(p, c)| p.adjacents().iter().cloned().map(|a| (a, *c)).collect::<Vec<(Pos, char)>>())
            .filter_map(|(p, c)| self.digits.get_key_value(&p).map(|kv| (kv.0, c)))
            .collect();
        let mut parts = Vec::new();
        while let Some(p) = positions.pop() {
            let (part, digit_positions) = self.extract_part(p.0);
            parts.push(part);
            digit_positions.iter()
                .for_each(|dp| {
                    if let Some(index) = positions.iter().position(|&p| p.0 == dp) {
                        positions.swap_remove(index);
                    }
                });
        }
        parts
    }

    fn extract_part(&self, pos: &Pos) -> (usize, Vec<Pos>) {
        let mut seeker = self.digits.get_key_value(pos).unwrap();
        while let Some(left) = seeker.0.left().and_then(|l| self.digits.get_key_value(&l)) {
            seeker = left;
        }
        let mut seen_positions = vec!(seeker.0.clone());
        let mut number = *seeker.1;
        while let Some(right) = seeker.0.right().and_then(|r| self.digits.get_key_value(&r)) {
            seen_positions.push(right.0.clone());
            seeker = right;
            number *= 10;
            number += seeker.1;
        }
        (number, seen_positions)
    }
}

impl From<&str> for Schematic {
    fn from(value: &str) -> Self {
        let input: Vec<Vec<char>> = value.lines().map(|l| l.chars().collect()).collect();
        let mut digits = HashMap::new();
        let mut symbols = HashMap::new();
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x].is_ascii_digit() {
                    digits.insert(Pos { x, y }, input[y][x].to_digit(10).unwrap() as usize);
                } else if input[y][x] != '.' {
                    symbols.insert(Pos { x, y }, input[y][x]);
                }
            }
        }
        Self { digits, symbols }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug, Ord, PartialOrd)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn adjacents(&self) -> Vec<Pos> {
        let min_x = if self.x == 0 { 0 } else { self.x - 1 };
        let min_y = if self.y == 0 { 0 } else { self.y - 1 };
        (min_y..=self.y + 1)
            .flat_map(|y| (min_x..=self.x + 1).map(move |x| Pos { x, y }))
            .collect()
    }

    fn left(&self) -> Option<Pos> {
        if self.x == 0 { None } else { Some(Pos { x: self.x - 1, y: self.y }) }
    }

    fn right(&self) -> Option<Pos> {
        Some(Pos { x: self.x + 1, y: self.y })
    }
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day03Solver {
    Advent2023Day03Solver::new(String::from("\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"))
}

#[test]
fn extracts_parts() {
    let solver = test_solver_1();
    let mut part_list = solver.schematic.list_parts();
    part_list.sort();
    assert_eq!(part_list, vec!(35, 467, 592, 598, 617, 633, 664, 755));
    assert_eq!(solver.solve_part1(), 4361);
}

#[test]
fn extracts_gear_ratios() {
    let solver = test_solver_1();
    let mut gear_ratios = solver.schematic.gear_ratios();
    gear_ratios.sort();
    assert_eq!(gear_ratios, vec!(16345, 451490));
    assert_eq!(solver.solve_part2(), 467835);
}
