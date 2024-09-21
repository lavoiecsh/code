use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::solver::AdventSolver;

pub struct Advent2016Day04Solver {
    rooms: Vec<Room>,
}

impl Advent2016Day04Solver {
    pub fn new(input: String) -> Self {
        let re = Regex::new("([a-z-]+)-([0-9]+)\\[([a-z]+)]").unwrap();
        Self {
            rooms: input.lines()
                .map(|l| {
                    let captures = re.captures(l).unwrap().iter().map(|m| m.unwrap().as_str()).collect::<Vec<_>>();
                    Room {
                        name: captures[1].to_string(),
                        sector_id: captures[2].parse().unwrap(),
                        checksum: captures[3].to_string(),
                    }
                })
                .collect()
        }
    }
}

impl AdventSolver for Advent2016Day04Solver {
    fn solve_part1(&self) -> usize {
        self.rooms.iter()
            .filter(|r| r.is_valid())
            .map(|r| r.sector_id as usize)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.rooms.iter()
            .find(|r| r.is_valid() && r.decrypt() == "northpole object storage")
            .unwrap()
            .sector_id as usize
    }
}

struct Room {
    name: String,
    sector_id: u16,
    checksum: String,
}

impl Room {
    fn is_valid(&self) -> bool {
        let mut counts: HashMap<char, u16> = HashMap::new();
        self.name.chars()
            .filter(|c| *c != '-')
            .for_each(|c| {
                match counts.get_mut(&c) {
                    Some(prev) => *prev += 1,
                    None => { counts.insert(c, 1); }
                };
            });
        let sorted = counts.iter()
            .sorted_by(|l, r| {
                if l.1 > r.1 {
                    Ordering::Less
                } else if l.1 < r.1 {
                    Ordering::Greater
                } else if l.0 < r.0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .take(5)
            .map(|(k, _)| k)
            .collect::<String>();
        sorted == self.checksum
    }

    fn decrypt(&self) -> String {
        let a = b'a';
        let cesar = (self.sector_id % 26) as u8;
        self.name.chars()
            .map(|c| if c == '-' { ' ' } else { (((c as u8 - a) + cesar) % 26 + a) as char })
            .collect()
    }
}
