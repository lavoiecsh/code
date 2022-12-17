use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::{RangeInclusive};
use num_bigint::{BigUint};
use regex::Regex;
use num_traits::identities::Zero;
use crate::solver::AdventSolver;
use rayon::prelude::*;

type Pos = (isize, isize);

#[derive(Debug)]
struct Sensor {
    position: Pos,
    beacon: Pos,
    dist: isize,
}

impl Sensor {
    fn new(position: Pos, beacon: Pos) -> Self {
        Self {
            position,
            beacon,
            dist: (position.0 - beacon.0).abs() + (position.1 - beacon.1).abs(),
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
        println!("This is a slow one: 30 minutes +");
        let max: usize = self.part2_max;
        let imax: isize = max as isize;
        let mut max_map = BigUint::new(vec!());
        for x in 0..=max {
            max_map.set_bit(x as u64, true);
        }
        (0..=imax)
            .into_par_iter()
            .map(|y| {
                let mut map = max_map.clone();
                for sensor in &self.sensors {
                    sensor.remove_detected(y, imax, &mut map);
                }
                if map.count_ones() != 0 {
                    return Some((map.bits() - 1) as usize * 4000000 + y as usize);
                }
                return None;
            })
            .find_first(Option::is_some)
            .unwrap()
            .unwrap()
    }
}
