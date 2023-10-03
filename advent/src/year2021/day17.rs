use regex::Regex;

use crate::solver::AdventSolver;

struct Area {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

pub struct Advent2021Day17Solver {
    area: Area
}

impl Advent2021Day17Solver {
    pub fn new(input: String) -> Self {
        let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
        let caps = re.captures(&input).unwrap();
        let get = |n: usize| caps.get(n).unwrap().as_str().parse::<i32>().unwrap();
        Self { area: Area { min_x: get(1), max_x: get(2), min_y: get(3), max_y: get(4) } }
    }
}

impl AdventSolver for Advent2021Day17Solver {
    fn solve_part1(&self) -> usize {
        let vel_y = (-self.area.min_y - 1) as usize;
        (vel_y * vel_y + vel_y) / 2
    }

    fn solve_part2(&self) -> usize {
        let min_vel_x = find_start_x(self.area.min_x);
        let max_vel_x = self.area.max_x;
        let min_vel_y = self.area.min_y;
        let max_vel_y = -self.area.min_y - 1;
        let mut count: usize = 0;
        for vel_x in min_vel_x..=max_vel_x {
            for vel_y in min_vel_y..=max_vel_y {
                let mut probe = Probe::new(vel_x, vel_y);
                if probe.falls_in_area(&self.area) {
                    count += 1;
                }
            }
        }
        count
    }
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
