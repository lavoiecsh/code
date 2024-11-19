use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Display, Formatter};

use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2018Day15Solver {
    map: Map,
}

impl Advent2018Day15Solver {
    pub fn new(input: &str) -> Self {
        let mut grid = Vec::new();
        let mut units = Vec::new();
        let rows: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        for y in 0..rows.len() {
            let mut grid_row = Vec::new();
            for x in 0..rows[y].len() {
                grid_row.push(match rows[y][x] {
                    '#' => false,
                    '.' => true,
                    'E' => {
                        units.push(Unit::elf(Pos { x, y }));
                        true
                    }
                    'G' => {
                        units.push(Unit::goblin(Pos { x, y }));
                        true
                    }
                    c => panic!("unknown character {c}"),
                });
            }
            grid.push(grid_row);
        }
        Self {
            map: Map::new(grid, units),
        }
    }
}

impl AdventSolver for Advent2018Day15Solver {
    fn solve_part1(&self) -> usize {
        let mut map = self.map.clone();
        map.run();
        map.outcome()
    }

    fn solve_part2(&self) -> usize {
        let mut increase = 0;
        let mut winning_outcome = None;
        while winning_outcome.is_none() {
            let mut map = self.map.clone();
            map.buff_elves(increase);
            map.run();
            if map.elves_win_without_loss() {
                winning_outcome = Some(map.outcome());
            }
            increase += 1;
        }
        winning_outcome.unwrap()
    }
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<bool>>,
    units: Vec<Unit>,
    rounds: usize,
}

impl Map {
    fn new(grid: Vec<Vec<bool>>, units: Vec<Unit>) -> Self {
        Self {
            grid,
            units,
            rounds: 0,
        }
    }

    fn buff_elves(&mut self, increase: usize) {
        for i in 0..self.units.len() {
            if self.units[i].is_elf {
                self.units[i].damage += increase;
            }
        }
    }

    fn outcome(&self) -> usize {
        self.rounds * self.units.iter().map(|u| u.health).sum::<usize>()
    }

    fn elves_win_without_loss(&self) -> bool {
        self.units.iter().all(|u| u.is_elf ^ u.is_dead())
    }

    fn run(&mut self) {
        while !self.iterate() && !self.is_done() {
            self.rounds += 1;
        }
    }

    fn is_done(&self) -> bool {
        self.units
            .iter()
            .filter(|u| !u.is_dead())
            .map(|u| u.is_elf)
            .all_equal()
    }

    fn iterate(&mut self) -> bool {
        let ordered: Vec<usize> = (0..self.units.len())
            .filter(|u| !self.units[*u].is_dead())
            .sorted_by_key(|u| self.units[*u].pos)
            .collect();

        for i in 0..ordered.len() {
            if self.units[ordered[i]].is_dead() {
                continue;
            }
            self.turn(ordered[i]);
            if self.is_done() && i != ordered.len() - 1 {
                return true;
            }
        }
        false
    }

    fn turn(&mut self, unit: usize) {
        if let Some(target) = self.target(unit) {
            return self.attack(unit, target);
        }

        let opponents: Vec<&Unit> = self
            .units
            .iter()
            .filter(|u| self.units[unit].is_enemy_of(u) && !u.is_dead())
            .collect();
        let adjacents: Vec<Pos> = opponents
            .iter()
            .flat_map(|o| o.pos.adjacents())
            .filter(|p| self.unit_at(p).is_none())
            .collect();
        if let Some(chosen) = self.choose_destination(self.units[unit].pos, &adjacents) {
            if let Some(next) = self.choose_next(self.units[unit].pos, chosen) {
                self.units[unit].pos = next;
            }
        }

        if let Some(target) = self.target(unit) {
            self.attack(unit, target)
        }
    }

    fn choose_destination(&self, pos: Pos, possible: &[Pos]) -> Option<Pos> {
        let mut distances: HashMap<Pos, usize> = HashMap::new();
        let mut queue: VecDeque<(Pos, usize)> = VecDeque::new();
        let mut remaining: Vec<Pos> = possible.to_owned();

        queue.push_back((pos, 0));
        while let Some(current) = queue.pop_front() {
            let d = distances.get(&current.0);
            if d.is_some() {
                continue;
            } else {
                distances.insert(current.0, current.1);
            }

            queue.extend(
                current
                    .0
                    .adjacents()
                    .into_iter()
                    .filter(|a| self.grid[a.y][a.x])
                    .filter(|a| self.unit_at(a).is_none())
                    .filter(|a| !distances.contains_key(a))
                    .map(|a| (a, current.1 + 1)),
            );

            if let Some(i) = remaining.iter().position(|r| r == &current.0) {
                remaining.swap_remove(i);
            }

            if remaining.is_empty() {
                break;
            }
        }

        possible
            .iter()
            .filter_map(|p| distances.get(p).map(|d| (p, *d)))
            .sorted_by(|l, r| l.1.cmp(&r.1).then(l.0.cmp(r.0)))
            .next()
            .map(|(p, _)| *p)
    }

    fn choose_next(&self, unit: Pos, destination: Pos) -> Option<Pos> {
        let mut distances: HashMap<Pos, usize> = HashMap::new();
        let mut queue: VecDeque<(Pos, usize)> = VecDeque::new();
        let mut remaining: Vec<Pos> = unit.adjacents();

        queue.push_back((destination, 0));
        while let Some(current) = queue.pop_front() {
            let d = distances.get(&current.0);
            if d.is_some() {
                continue;
            } else {
                distances.insert(current.0, current.1);
            }

            queue.extend(
                current
                    .0
                    .adjacents()
                    .into_iter()
                    .filter(|a| self.grid[a.y][a.x])
                    .filter(|a| self.unit_at(a).is_none())
                    .filter(|a| !distances.contains_key(a))
                    .map(|a| (a, current.1 + 1)),
            );

            if let Some(i) = remaining.iter().position(|r| r == &current.0) {
                remaining.swap_remove(i);
            }

            if remaining.is_empty() {
                break;
            }
        }

        unit.adjacents()
            .into_iter()
            .filter_map(|p| distances.get(&p).map(|d| (p, *d)))
            .sorted_by(|l, r| l.1.cmp(&r.1).then_with(|| l.0.cmp(&r.0)))
            .next()
            .map(|(p, _)| p)
    }

    fn target(&self, unit: usize) -> Option<usize> {
        self.units[unit]
            .pos
            .adjacents()
            .iter()
            .filter_map(|p| self.unit_at(p))
            .filter(|u| self.units[unit].is_enemy_of(&self.units[*u]))
            .sorted_by(|l, r| {
                let left = &self.units[*l];
                let right = &self.units[*r];
                left.health
                    .cmp(&right.health)
                    .then(left.pos.cmp(&right.pos))
            })
            .next()
    }

    fn attack(&mut self, unit: usize, target: usize) {
        if self.units[target].health < self.units[unit].damage {
            self.units[target].health = 0;
        } else {
            self.units[target].health -= self.units[unit].damage;
        }
    }

    fn unit_at(&self, pos: &Pos) -> Option<usize> {
        self.units
            .iter()
            .position(|u| !u.is_dead() && u.pos == *pos)
    }
}

#[derive(Clone)]
struct Unit {
    is_elf: bool,
    pos: Pos,
    damage: usize,
    health: usize,
}

impl Unit {
    fn elf(pos: Pos) -> Self {
        Self {
            is_elf: true,
            pos,
            damage: 3,
            health: 200,
        }
    }

    fn goblin(pos: Pos) -> Self {
        Self {
            is_elf: false,
            pos,
            damage: 3,
            health: 200,
        }
    }

    fn is_enemy_of(&self, other: &Unit) -> bool {
        self.is_elf != other.is_elf
    }

    fn is_dead(&self) -> bool {
        self.health == 0
    }
}

impl Debug for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}({:3}) at ({:3},{:3})",
            if self.is_elf { "E" } else { "G" },
            self.health,
            self.pos.x,
            self.pos.y
        ))
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}({:3}) at ({:3},{:3})",
            if self.is_elf { "E" } else { "G" },
            self.health,
            self.pos.x,
            self.pos.y
        ))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn adjacents(&self) -> Vec<Pos> {
        vec![
            Pos {
                x: self.x,
                y: self.y - 1,
            },
            Pos {
                x: self.x - 1,
                y: self.y,
            },
            Pos {
                x: self.x + 1,
                y: self.y,
            },
            Pos {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }
}
