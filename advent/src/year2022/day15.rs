use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::{RangeInclusive};
use num_bigint::{BigUint};
use regex::Regex;
use num_traits::identities::Zero;
use crate::solver::AdventSolver;

type Pos = (isize, isize);

#[derive(Debug)]
struct Sensor {
    position: Pos,
    beacon: Pos,
    dist: isize,
}

fn dist(pos1: &Pos, pos2: &Pos) -> isize {
    (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs()
}

impl Sensor {
    fn new(position: Pos, beacon: Pos) -> Self {
        Self {
            position,
            beacon,
            dist: dist(&position, &beacon),
        }
    }

    fn overlap_range(&self, y: isize) -> Option<RangeInclusive<isize>> {
        let remaining = self.dist - if self.position.1 > y { self.position.1 - y } else { y - self.position.1 };
        if remaining < 0 { None } else { Some(self.position.0 - remaining..=self.position.0 + remaining) }
    }

    fn overlap(&self, y: isize, neg_map: &mut BigUint, pos_map: &mut BigUint) -> () {
        match self.overlap_range(y) {
            None => {},
            Some(range) => {
                if range.start() < &0 && range.end() > &0 {
                    (0..=-range.start()).for_each(|x| neg_map.set_bit(x as u64, true));
                    (0..=*range.end()).for_each(|x| pos_map.set_bit(x as u64, true));
                } else {
                    range.for_each(|x| pos_map.set_bit(x as u64, true));
                }
            }
        }
    }

    fn remove_detected(&self, y: isize, max: isize, map: &mut BigUint) -> () {
        match self.overlap_range(y) {
            None => {},
            Some(range) => (isize::max(*range.start(), 0)..=isize::min(*range.end(), max))
                .for_each(|x| map.set_bit(x as u64, false)),
        }
    }

    fn is_in_border(&self, pos: &Pos) -> bool {
        dist(&self.position, pos) <= self.dist
    }

    fn border(&self) -> Vec<Pos> {
        let mut pos = vec!();
        for y in 0..self.dist {
            let y_above = self.position.1 - self.dist + y;
            let y_below = self.position.1 + self.dist - y;
            let x_left = self.position.0 - y - 1;
            let x_right = self.position.0 + y + 1;
            pos.push((x_left, y_above));
            pos.push((x_left, y_below));
            pos.push((x_right, y_above));
            pos.push((x_right, y_below));
        }
        pos.push((self.position.0 - self.dist - 1, self.position.1));
        pos.push((self.position.0 + self.dist + 1, self.position.1));
        pos.push((self.position.0, self.position.1 - self.dist - 1));
        pos.push((self.position.0, self.position.1 + self.dist + 1));
        pos
    }
}

pub struct Advent2022Day15Solver {
    sensors: Vec<Sensor>,
    part1_y: isize,
    part2_max: usize,
}

impl Advent2022Day15Solver {
    pub fn new() -> Self {
        let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
        Self {
            part1_y: 2000000,
            part2_max: 4000000,
            sensors: read_to_string("src/year2022/day15.txt")
                .unwrap()
                .lines()
                .map(|l| {
                    let cap = re.captures(l).unwrap();
                    let get = |n| cap.get(n).unwrap().as_str().parse().unwrap();
                    Sensor::new((get(1), get(2)), (get(3), get(4)))
                })
                .collect()
        }
    }
}

impl AdventSolver for Advent2022Day15Solver {
    fn day(&self) -> usize { 15 }
    fn year(&self) -> usize { 2022 }

    fn solve_part1(&self) -> usize {
        let mut neg_map = BigUint::zero();
        let mut pos_map = BigUint::zero();
        let mut beacons = HashSet::new();
        for sensor in &self.sensors {
            sensor.overlap(self.part1_y, &mut neg_map, &mut pos_map);
            if sensor.beacon.1 == self.part1_y {
                beacons.insert(sensor.beacon.0);
            }
        }
        beacons
            .iter()
            .for_each(|x| if x < &0 { neg_map.set_bit(-x as u64, false) } else { pos_map.set_bit(*x as u64, false) });
        neg_map.count_ones() as usize
            + pos_map.count_ones() as usize
            - if neg_map.bit(0) && pos_map.bit(0) { 1 } else { 0 }
    }

    fn solve_part2(&self) -> usize {
        let max: usize = self.part2_max;
        let imax: isize = max as isize;
        let all_borders: Vec<Pos> = self.sensors
            .iter()
            .flat_map(|s| s.border())
            .filter(|(x,y)| x >= &0 && x <= &imax && y >= &0 && y <= &imax)
            .collect();
        let pos = all_borders
            .iter()
            .find(|p| self.sensors.iter().all(|s| !s.is_in_border(p)))
            .unwrap();
        (pos.0 * 4000000 + pos.1) as usize
    }
}
