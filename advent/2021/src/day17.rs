#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

use regex::Regex;

const FILENAME: &str = "inputs/day17.txt";

struct Area {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

#[allow(dead_code)]
fn read_input() -> Area {
    let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let line = fs::read_to_string(FILENAME).expect("error reading");
    let caps = re.captures(&line).unwrap();
    Area {
        min_x: caps.get(1).unwrap().as_str().parse().expect("error parsing"),
        max_x: caps.get(2).unwrap().as_str().parse().expect("error parsing"),
        min_y: caps.get(3).unwrap().as_str().parse().expect("error parsing"),
        max_y: caps.get(4).unwrap().as_str().parse().expect("error parsing"),
    }
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let area = read_input();
    let vel_y = (-area.min_y - 1) as usize;
    (vel_y * vel_y + vel_y) / 2
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let area = read_input();
    let min_vel_x = find_start_x(area.min_x);
    let max_vel_x = area.max_x;
    let min_vel_y = area.min_y;
    let max_vel_y = -area.min_y - 1;
    debug!((min_vel_x, max_vel_x));
    debug!((min_vel_y, max_vel_y));
    let mut count: usize = 0;
    for vel_x in min_vel_x..=max_vel_x {
        for vel_y in min_vel_y..=max_vel_y {
            let mut probe = Probe::new(vel_x, vel_y);
            if probe.falls_in_area(&area) {
                debug!((vel_x, vel_y));
                count += 1;
            }
        }
    }
    count
}

struct Probe {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
}

impl Probe {
    fn new(vel_x: i32, vel_y: i32) -> Probe {
        Probe {
            x: 0,
            y: 0,
            vel_x: vel_x,
            vel_y: vel_y,
        }
    }
    
    fn step(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;
        self.vel_x = if self.vel_x < 0 { self.vel_x + 1 } else if self.vel_x > 0 { self.vel_x - 1 } else { 0 };
        self.vel_y -= 1;
    }

    fn is_in_area(&self, area: &Area) -> bool {
        self.x >= area.min_x &&
            self.x <= area.max_x &&
            self.y >= area.min_y &&
            self.y <= area.max_y
    }

    fn falls_in_area(&mut self, area: &Area) -> bool {
        while self.y >= area.min_y {
            if self.is_in_area(&area) {
                return true;
            }
            self.step();
        }
        return false;
    }
}

fn find_start_x(min_x: i32) -> i32 {
    let mut t = 0;
    let mut i = 1;
    while t < min_x {
        t += i;
        i += 1;
    }
    i-1
}
