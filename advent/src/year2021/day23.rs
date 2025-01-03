use regex::{Captures, Regex};

use crate::solver::AdventSolver;

type Map = [Room; 27];

pub struct Advent2021Day23Solver {
    input: BurrowInit,
}

impl Advent2021Day23Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"([ABCD])").unwrap();
        let mut lines = input.trim().lines();
        lines.next();
        lines.next();
        let mut top_row_captures = re.captures_iter(lines.next().unwrap());
        let mut bot_row_captures = re.captures_iter(lines.next().unwrap());
        let first_char = |cap: Option<Captures>| {
            cap.unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap()
        };
        Self {
            input: (
                (
                    first_char(top_row_captures.next()),
                    first_char(bot_row_captures.next()),
                ),
                (
                    first_char(top_row_captures.next()),
                    first_char(bot_row_captures.next()),
                ),
                (
                    first_char(top_row_captures.next()),
                    first_char(bot_row_captures.next()),
                ),
                (
                    first_char(top_row_captures.next()),
                    first_char(bot_row_captures.next()),
                ),
            ),
        }
    }
}

impl AdventSolver for Advent2021Day23Solver {
    fn solve_part1(&self) -> usize {
        solve(Burrow::new_part1(self.input))
    }

    fn solve_part2(&self) -> usize {
        solve(Burrow::new_part2(self.input))
    }
}

fn solve(start: Burrow) -> usize {
    let map = make_map();
    let mut all = vec![start];
    let mut best = usize::MAX;
    while !all.is_empty() {
        let current = all.remove(0);
        let nexts = current.iterate(&map);
        let (solved, unsolved): (Vec<Burrow>, Vec<Burrow>) =
            nexts.into_iter().partition(|b| b.is_solved());
        for u in unsolved {
            let mut replace = true;
            let energy = u.total_energy();
            for i in (0..all.len()).rev() {
                if !all[i].matches(&u) {
                    continue;
                }
                if all[i].total_energy() > energy {
                    all.remove(i);
                } else {
                    replace = false;
                }
            }
            if replace {
                all.push(u);
            }
        }
        solved.iter().for_each(|b| {
            if b.total_energy() < best {
                best = b.total_energy()
            }
        });
    }
    best
}

struct Room {
    left: Option<usize>,
    right: Option<usize>,
    top: Option<usize>,
    sideroom_for: Option<char>,
}

impl Room {
    fn new_hallway(left: Option<usize>, right: Option<usize>) -> Self {
        Room {
            left,
            right,
            top: None,
            sideroom_for: None,
        }
    }

    fn new_sideroom(top: usize, sideroom_for: char) -> Self {
        Room {
            left: None,
            right: None,
            top: Some(top),
            sideroom_for: Some(sideroom_for),
        }
    }
}

fn make_map() -> Map {
    [
        /*  0 */ Room::new_hallway(None, Some(1)),
        /*  1 */ Room::new_hallway(Some(0), Some(2)),
        /*  2 */ Room::new_hallway(Some(1), Some(3)),
        /*  3 */ Room::new_hallway(Some(2), Some(4)),
        /*  4 */ Room::new_hallway(Some(3), Some(5)),
        /*  5 */ Room::new_hallway(Some(4), Some(6)),
        /*  6 */ Room::new_hallway(Some(5), Some(7)),
        /*  7 */ Room::new_hallway(Some(6), Some(8)),
        /*  8 */ Room::new_hallway(Some(7), Some(9)),
        /*  9 */ Room::new_hallway(Some(8), Some(10)),
        /* 10 */ Room::new_hallway(Some(9), None),
        /* 11 */ Room::new_sideroom(2, 'A'),
        /* 12 */ Room::new_sideroom(11, 'A'),
        /* 13 */ Room::new_sideroom(12, 'A'),
        /* 14 */ Room::new_sideroom(13, 'A'),
        /* 15 */ Room::new_sideroom(4, 'B'),
        /* 16 */ Room::new_sideroom(15, 'B'),
        /* 17 */ Room::new_sideroom(16, 'B'),
        /* 18 */ Room::new_sideroom(17, 'B'),
        /* 19 */ Room::new_sideroom(6, 'C'),
        /* 20 */ Room::new_sideroom(19, 'C'),
        /* 21 */ Room::new_sideroom(20, 'C'),
        /* 22 */ Room::new_sideroom(21, 'C'),
        /* 23 */ Room::new_sideroom(8, 'D'),
        /* 24 */ Room::new_sideroom(23, 'D'),
        /* 25 */ Room::new_sideroom(24, 'D'),
        /* 26 */ Room::new_sideroom(25, 'D'),
    ]
}

fn bottom_sideroom_for(atype: char) -> usize {
    match atype {
        'A' => 14,
        'B' => 18,
        'C' => 22,
        'D' => 26,
        _ => panic!("unknown type"),
    }
}

#[derive(Copy, Clone)]
struct Amphipod {
    a_type: char,
    energy: usize,
}

impl Amphipod {
    fn new(a_type: char) -> Self {
        Self { a_type, energy: 0 }
    }

    fn move_by(&self, dist: usize) -> Self {
        Self {
            a_type: self.a_type,
            energy: self.energy
                + match self.a_type {
                    'A' => dist,
                    'B' => 10 * dist,
                    'C' => 100 * dist,
                    'D' => 1000 * dist,
                    _ => panic!("unknown type"),
                },
        }
    }
}

struct Burrow {
    rooms: [Option<Amphipod>; 27],
}

type BurrowInit = ((char, char), (char, char), (char, char), (char, char));

impl Burrow {
    fn new_part1(rooms: BurrowInit) -> Self {
        Self {
            rooms: [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Amphipod::new(rooms.0 .0)),
                Some(Amphipod::new(rooms.0 .1)),
                Some(Amphipod::new('A')),
                Some(Amphipod::new('A')),
                Some(Amphipod::new(rooms.1 .0)),
                Some(Amphipod::new(rooms.1 .1)),
                Some(Amphipod::new('B')),
                Some(Amphipod::new('B')),
                Some(Amphipod::new(rooms.2 .0)),
                Some(Amphipod::new(rooms.2 .1)),
                Some(Amphipod::new('C')),
                Some(Amphipod::new('C')),
                Some(Amphipod::new(rooms.3 .0)),
                Some(Amphipod::new(rooms.3 .1)),
                Some(Amphipod::new('D')),
                Some(Amphipod::new('D')),
            ],
        }
    }

    fn new_part2(rooms: BurrowInit) -> Self {
        Self {
            rooms: [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Amphipod::new(rooms.0 .0)),
                Some(Amphipod::new('D')),
                Some(Amphipod::new('D')),
                Some(Amphipod::new(rooms.0 .1)),
                Some(Amphipod::new(rooms.1 .0)),
                Some(Amphipod::new('C')),
                Some(Amphipod::new('B')),
                Some(Amphipod::new(rooms.1 .1)),
                Some(Amphipod::new(rooms.2 .0)),
                Some(Amphipod::new('B')),
                Some(Amphipod::new('A')),
                Some(Amphipod::new(rooms.2 .1)),
                Some(Amphipod::new(rooms.3 .0)),
                Some(Amphipod::new('A')),
                Some(Amphipod::new('C')),
                Some(Amphipod::new(rooms.3 .1)),
            ],
        }
    }

    fn iterate(&self, map: &Map) -> Vec<Self> {
        let mut nexts = Vec::new();
        for i in 0..=10 {
            if self.rooms[i].is_none() {
                continue;
            }
            let a = self.rooms[i].unwrap();
            let mut open = bottom_sideroom_for(a.a_type);
            while self.is_sideroom_solved(open) && map[open].top.is_some() {
                open = map[open].top.unwrap();
            }
            if map[open].sideroom_for.is_some() {
                if let Some(distance) = self.path_distance(map, i, open) {
                    nexts.push(self.move_amphipod(i, open, &a, distance))
                }
            }
        }
        for i in 11..27 {
            if self.rooms[i].is_none() || self.is_sideroom_solved(i) {
                continue;
            }
            for j in [0, 1, 3, 5, 7, 9, 10] {
                if let Some(distance) = self.path_distance(map, i, j) {
                    nexts.push(self.move_amphipod(i, j, &self.rooms[i].unwrap(), distance))
                }
            }
            let mut open = bottom_sideroom_for(self.rooms[i].unwrap().a_type);
            while self.is_sideroom_solved(open) && map[open].top.is_some() {
                open = map[open].top.unwrap();
            }
            if map[open].sideroom_for.is_some() {
                if let Some(distance) = self.path_distance(map, i, open) {
                    nexts.push(self.move_amphipod(i, open, &self.rooms[i].unwrap(), distance))
                }
            }
        }
        nexts
    }

    fn move_amphipod(&self, from: usize, to: usize, amphipod: &Amphipod, distance: usize) -> Self {
        let mut new_rooms: [Option<Amphipod>; 27] = [None; 27];
        for i in 0..27 {
            if i == from {
                new_rooms[i] = None;
            } else if i == to {
                new_rooms[i] = Some(amphipod.move_by(distance));
            } else {
                new_rooms[i] = self.rooms[i];
            }
        }
        Self { rooms: new_rooms }
    }

    fn is_sideroom_solved(&self, i: usize) -> bool {
        if self.rooms[i].is_none() {
            return false;
        }
        let a = self.rooms[i].unwrap();
        match i {
            11 => a.a_type == 'A' && self.is_sideroom_solved(12),
            12 => a.a_type == 'A' && self.is_sideroom_solved(13),
            13 => a.a_type == 'A' && self.is_sideroom_solved(14),
            14 => a.a_type == 'A',
            15 => a.a_type == 'B' && self.is_sideroom_solved(16),
            16 => a.a_type == 'B' && self.is_sideroom_solved(17),
            17 => a.a_type == 'B' && self.is_sideroom_solved(18),
            18 => a.a_type == 'B',
            19 => a.a_type == 'C' && self.is_sideroom_solved(20),
            20 => a.a_type == 'C' && self.is_sideroom_solved(21),
            21 => a.a_type == 'C' && self.is_sideroom_solved(22),
            22 => a.a_type == 'C',
            23 => a.a_type == 'D' && self.is_sideroom_solved(24),
            24 => a.a_type == 'D' && self.is_sideroom_solved(25),
            25 => a.a_type == 'D' && self.is_sideroom_solved(26),
            26 => a.a_type == 'D',
            _ => false,
        }
    }

    fn path_distance(&self, rooms: &Map, start: usize, end: usize) -> Option<usize> {
        if self.rooms[end].is_some() {
            return None;
        }
        let mut s = start;
        let mut e = end;
        let mut distance = 0;
        while rooms[s].top.is_some() {
            s = rooms[s].top.unwrap();
            distance += 1;
            if self.rooms[s].is_some() {
                return None;
            }
        }
        while rooms[e].top.is_some() {
            e = rooms[e].top.unwrap();
            distance += 1;
            if self.rooms[e].is_some() {
                return None;
            }
        }
        while s > e {
            s = rooms[s].left.unwrap();
            distance += 1;
            if self.rooms[s].is_some() {
                return None;
            }
        }
        while s < e {
            s = rooms[s].right.unwrap();
            distance += 1;
            if self.rooms[s].is_some() {
                return None;
            }
        }
        Some(distance)
    }

    fn is_solved(&self) -> bool {
        [11, 15, 19, 23].iter().all(|i| self.is_sideroom_solved(*i))
    }

    fn total_energy(&self) -> usize {
        (0..27).fold(0, |acc, i| {
            acc + self.rooms[i].map(|a| a.energy).unwrap_or(0)
        })
    }

    fn matches(&self, other: &Burrow) -> bool {
        for i in 0..27 {
            if self.rooms[i].is_none() {
                if other.rooms[i].is_none() {
                    continue;
                } else {
                    return false;
                }
            } else {
                if other.rooms[i].is_none() {
                    return false;
                }
                if self.rooms[i].unwrap().a_type != other.rooms[i].unwrap().a_type {
                    return false;
                }
            }
        }
        true
    }
}
