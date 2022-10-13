#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;
use regex::Regex;

const FILENAME: &str = "inputs/day22.txt";

#[allow(dead_code)]
fn read_input() -> Vec<Step> {
    let re = Regex::new(r"(on|off) x=([-]?\d+)\.\.([-]?\d+),y=([-]?\d+)\.\.([-]?\d+),z=([-]?\d+)\.\.([-]?\d+)").unwrap();
    fs::read_to_string(FILENAME).expect("error reading")
        .trim()
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|c| Step::new(
            c.get(1).unwrap().as_str(),
            c.get(2).unwrap().as_str().parse().unwrap(),
            c.get(3).unwrap().as_str().parse().unwrap(),
            c.get(4).unwrap().as_str().parse().unwrap(),
            c.get(5).unwrap().as_str().parse().unwrap(),
            c.get(6).unwrap().as_str().parse().unwrap(),
            c.get(7).unwrap().as_str().parse().unwrap(),
        ))
        .collect()
}

#[allow(dead_code)]
pub fn part1() -> usize {
    read_input()
        .iter()
        .filter(|s| s.is_initialization())
        .fold(Vec::new(), |acc, s| execute_regions(s, &acc))
        .iter()
        .fold(0, |acc, r| acc + r.size())
}

#[allow(dead_code)]
pub fn part2() -> usize {
    read_input()
        .iter()
        .fold(Vec::new(), |acc, s| execute_regions(s, &acc))
        .iter()
        .fold(0, |acc, r| acc + r.size())
}


fn execute_regions(step: &Step, regions: &Vec<Region>) -> Vec<Region> {
    debug!((step, regions.len()));
    let step_region = step.region();
    let mut new_regions: Vec<Region> = regions
        .iter()
        .flat_map(|r| r.split_x(step.x))
        .flat_map(|r| r.split_y(step.y))
        .flat_map(|r| r.split_z(step.z))
        .filter(|r| !r.is_within(&step_region))
        .collect();
    if step.on {
        new_regions.push(step_region);
    }
    new_regions
}

struct Region {
    x: (i32,i32),
    y: (i32,i32),
    z: (i32,i32),
}

impl Region {
    fn new(x: (i32,i32), y: (i32,i32), z: (i32,i32)) -> Self {
        Self { x: x, y: y, z: z }
    }
    
    fn copy(&self) -> Self {
        Self { x: self.x, y: self.y, z: self.z }
    }
    
    fn split_x(&self, other: (i32, i32)) -> Vec<Self> {
        if self.x.1 < other.0 || self.x.0 > other.1 {
            vec!(self.copy())
        } else if self.x.0 < other.0 {
            if self.x.1 <= other.1 {
                vec!(Self::new((self.x.0, other.0-1), self.y, self.z),
                     Self::new((other.0, self.x.1), self.y, self.z))
            } else {
                vec!(Self::new((self.x.0, other.0-1), self.y, self.z),
                     Self::new((other.0, other.1), self.y, self.z),
                     Self::new((other.1+1, self.x.1), self.y, self.z))
            }
        } else {
            if self.x.1 <= other.1 {
                vec!(self.copy())
            } else {
                vec!(Self::new((self.x.0, other.1), self.y, self.z),
                     Self::new((other.1+1, self.x.1), self.y, self.z))
            }
        }
    }

    fn split_y(&self, other: (i32, i32)) -> Vec<Self> {
        if self.y.1 < other.0 || self.y.0 > other.1 {
            vec!(self.copy())
        } else if self.y.0 < other.0 {
            if self.y.1 <= other.1 {
                vec!(Self::new(self.x, (self.y.0, other.0-1), self.z),
                     Self::new(self.x, (other.0, self.y.1), self.z))
            } else {
                vec!(Self::new(self.x, (self.y.0, other.0-1), self.z),
                     Self::new(self.x, (other.0, other.1), self.z),
                     Self::new(self.x, (other.1+1, self.y.1), self.z))
            }
        } else {
            if self.y.1 <= other.1 {
                vec!(self.copy())
            } else {
                vec!(Self::new(self.x, (self.y.0, other.1), self.z),
                     Self::new(self.x, (other.1+1, self.y.1), self.z))
            }
        }
    }

    fn split_z(&self, other: (i32, i32)) -> Vec<Self> {
        if self.z.1 < other.0 || self.z.0 > other.1 {
            vec!(self.copy())
        } else if self.z.0 < other.0 {
            if self.z.1 <= other.1 {
                vec!(Self::new(self.x, self.y, (self.z.0, other.0-1)),
                     Self::new(self.x, self.y, (other.0, self.z.1)))
            } else {
                vec!(Self::new(self.x, self.y, (self.z.0, other.0-1)),
                     Self::new(self.x, self.y, (other.0, other.1)),
                     Self::new(self.x, self.y, (other.1+1, self.z.1)))
            }
        } else {
            if self.z.1 <= other.1 {
                vec!(self.copy())
            } else {
                vec!(Self::new(self.x, self.y, (self.z.0, other.1)),
                     Self::new(self.x, self.y, (other.1+1, self.z.1)))
            }
        }
    }

    fn is_within(&self, other: &Region) -> bool {
        self.x.0 >= other.x.0 && self.x.1 <= other.x.1 &&
            self.y.0 >= other.y.0 && self.y.1 <= other.y.1 &&
            self.z.0 >= other.z.0 && self.z.1 <= other.z.1
    }

    fn size(&self) -> usize {
        ((self.x.1 - self.x.0+1).abs() as usize) * ((self.y.1 - self.y.0+1).abs() as usize) * ((self.z.1 - self.z.0+1).abs() as usize)
    }
}

impl std::fmt::Debug for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({:6},{:6}) ({:6},{:6}), ({:6},{:6}), {}", self.x.0, self.x.1, self.y.0, self.y.1, self.z.0, self.z.1, self.size())
    }
}

// type Cube = (i32,i32,i32);

struct Step {
    on: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl Step {
    fn new(on: &str, xs: i32, xe: i32, ys: i32, ye: i32, zs: i32, ze: i32) -> Self {
        Self {
            on: on == "on",
            x: (xs, xe),
            y: (ys, ye),
            z: (zs, ze),
        }
    }

    fn is_initialization(&self) -> bool {
        self.x.0 >= -50 && self.x.0 <= 50 &&
            self.x.1 >= -50 && self.x.1 <= 50 &&
            self.y.0 >= -50 && self.y.0 <= 50 &&
            self.y.1 >= -50 && self.y.1 <= 50 &&
            self.z.0 >= -50 && self.z.0 <= 50 &&
            self.z.1 >= -50 && self.z.1 <= 50
    }

    fn region(&self) -> Region {
        Region::new(self.x, self.y, self.z)
    }
}

impl std::fmt::Debug for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}  ({:6},{:6}) ({:6},{:6}) ({:6},{:6})", if self.on { "on" } else { "of" }, self.x.0, self.x.1, self.y.0, self.y.1, self.z.0, self.z.1)
    }
}
