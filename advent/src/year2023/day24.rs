use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Mul, Range, Sub};

use itertools::Itertools;
use num_integer::sqrt;
use num_traits::abs;
use regex::{Match, Regex};

use crate::solver::AdventSolver;

pub struct Advent2023Day24Solver {
    hailstorm: Hailstorm,
}

impl Advent2023Day24Solver {
    pub fn new(input: String) -> Self {
        let re = Regex::new(r"(\d+), (\d+), (\d+) @\s+(-?\d+),\s+(-?\d+),\s+(-?\d+)").unwrap();
        let parse = |c: Option<Match>| c.unwrap().as_str().parse().unwrap();
        Self {
            hailstorm: Hailstorm {
                hailstones: input.lines()
                    .filter_map(|l| re.captures(l))
                    .map(|cap| Hailstone::new(
                        Int3 {
                            x: parse(cap.get(1)),
                            y: parse(cap.get(2)),
                            z: parse(cap.get(3)),
                        },
                        Int3 {
                            x: parse(cap.get(4)),
                            y: parse(cap.get(5)),
                            z: parse(cap.get(6)),
                        },
                    ))
                    .collect()
            }
        }
    }
}

impl AdventSolver for Advent2023Day24Solver {
    fn solve_part1(&self) -> usize {
        self.hailstorm.crossing_count(Range2 {
            x: 200000000000000f64..400000000000000f64,
            y: 200000000000000f64..400000000000000f64,
        })
    }

    fn solve_part2(&self) -> usize {
        let rock = self.hailstorm.rock_hitting_everything();
        if !rock.hits_all(&self.hailstorm.hailstones) {
            return 0;
        }
        (rock.position.x + rock.position.y + rock.position.z) as usize
    }
}

struct Hailstorm {
    hailstones: Vec<Hailstone>,
}

impl Hailstorm {
    fn crossing_count(&self, range: Range2) -> usize {
        let mut count = 0;
        for i in 0..self.hailstones.len() {
            for j in i + 1..self.hailstones.len() {
                if let Some(intersection) = self.hailstones[i].future_intersection_with(&self.hailstones[j]) {
                    if range.contains(intersection) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn rock_hitting_everything(&self) -> Hailstone {
        let mut x_parallels = vec!();
        let mut y_parallels = vec!();
        let mut z_parallels = vec!();
        for i in 0..self.hailstones.len() {
            let hsi = &self.hailstones[i];
            for j in i + 1..self.hailstones.len() {
                let hsj = &self.hailstones[j];
                if hsi.velocity.x == hsj.velocity.x {
                    x_parallels.push(Parallel::new(hsi.position.x - hsj.position.x, hsi.velocity.x));
                }
                if hsi.velocity.y == hsj.velocity.y {
                    y_parallels.push(Parallel::new(hsi.position.y - hsj.position.y, hsi.velocity.y));
                }
                if hsi.velocity.z == hsj.velocity.z {
                    z_parallels.push(Parallel::new(hsi.position.z - hsj.position.z, hsi.velocity.z));
                }
            }
        }

        let x_velocities = x_parallels.iter().map(|p| p.intersecting_velocities()).reduce(|a, c| a.intersection(&c).cloned().collect()).unwrap();
        let y_velocities = y_parallels.iter().map(|p| p.intersecting_velocities()).reduce(|a, c| a.intersection(&c).cloned().collect()).unwrap();
        let z_velocities = z_parallels.iter().map(|p| p.intersecting_velocities()).reduce(|a, c| a.intersection(&c).cloned().collect()).unwrap();

        if let Some(x_match) = self.hailstones.iter().find(|hs| x_velocities.iter().contains(&hs.velocity.x)) {
            let other = self.hailstones.iter().find(|&hs| hs != x_match).unwrap();
            let time = (x_match.position.x - other.position.x) / (other.velocity.x - x_match.velocity.x);
            for &vy in &y_velocities {
                let py = other.position.y + time * (other.velocity.y - vy);
                for &vz in &z_velocities {
                    let pz = other.position.z + time * (other.velocity.z - vz);
                    let rock = Hailstone::new(Int3 { x: x_match.position.x, y: py, z: pz }, Int3 { x: x_match.velocity.x, y: vy, z: vz });
                    if rock.hits_all(&self.hailstones) {
                        return rock;
                    }
                }
            }
        } else if let Some(y_match) = self.hailstones.iter().find(|hs| y_velocities.iter().contains(&hs.velocity.y)) {
            let other = self.hailstones.iter().find(|&hs| hs != y_match).unwrap();
            let time = (y_match.position.y - other.position.y) / (other.velocity.y - y_match.velocity.y);
            for &vx in &x_velocities {
                let px = other.position.x + time * (other.velocity.x - vx);
                for &vz in &z_velocities {
                    let pz = other.position.z + time * (other.velocity.z - vz);
                    let rock = Hailstone::new(Int3 { x: px, y: y_match.position.y, z: pz }, Int3 { x: vx, y: y_match.velocity.y, z: vz });
                    if rock.hits_all(&self.hailstones) {
                        return rock;
                    }
                }
            }
        } else if let Some(z_match) = self.hailstones.iter().find(|hs| z_velocities.iter().contains(&hs.velocity.z)) {
            let other = self.hailstones.iter().find(|&hs| hs != z_match).unwrap();
            let time = (z_match.position.z - other.position.z) / (other.velocity.z - z_match.velocity.z);
            for &vx in &x_velocities {
                let px = other.position.x + time * (other.velocity.x - vx);
                for &vy in &y_velocities {
                    let py = other.position.y + time * (other.velocity.y - vy);
                    let rock = Hailstone::new(Int3 { x: px, y: py, z: z_match.position.z }, Int3 { x: vx, y: vy, z: z_match.velocity.z });
                    if rock.hits_all(&self.hailstones) {
                        return rock;
                    }
                }
            }
        }
        panic!("rock not found");
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Hailstone {
    position: Int3,
    velocity: Int3,
}

impl Hailstone {
    fn new(position: Int3, velocity: Int3) -> Self {
        Self { position, velocity }
    }

    fn future_intersection_with(&self, other: &Self) -> Option<Float2> {
        let p1 = self.as_float();
        let p2 = self.next();
        let p3 = other.as_float();
        let p4 = other.next();
        let den = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);
        if den == 0f64 { return None; }
        let n1 = p1.x * p2.y - p1.y * p2.x;
        let n2 = p3.x * p4.y - p3.y * p4.x;
        let nx = n1 * (p3.x - p4.x) - n2 * (p1.x - p2.x);
        let ny = n1 * (p3.y - p4.y) - n2 * (p1.y - p2.y);
        let x = nx / den;
        let y = ny / den;
        if (self.velocity.x > 0 && x < p1.x) ||
            (self.velocity.x < 0 && x > p1.x) ||
            (other.velocity.x > 0 && x < p3.x) ||
            (other.velocity.x < 0 && x > p3.x) {
            None
        } else {
            Some(Float2 { x, y })
        }
    }

    fn as_float(&self) -> Float2 {
        Float2 {
            x: self.position.x as f64,
            y: self.position.y as f64,
        }
    }

    fn next(&self) -> Float2 {
        Float2 {
            x: (self.position.x + self.velocity.x) as f64,
            y: (self.position.y + self.velocity.y) as f64,
        }
    }

    fn hits_all(&self, others: &[Self]) -> bool {
        let times = others.iter().filter_map(|o| self.hits(o)).collect::<HashSet<i64>>();
        times.len() == others.len()
    }

    fn hits(&self, other: &Self) -> Option<i64> {
        let den = other.velocity.x - self.velocity.x;
        if den == 0 { return None; }
        let num = other.position.x - self.position.x;
        if num % den != 0 { return None; }
        let t = abs(num / den);

        if self.position + self.velocity * t == other.position + other.velocity * t {
            Some(t)
        } else {
            None
        }
    }
}

impl Debug for Hailstone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?} @ {:?}", self.position, self.velocity))
    }
}

#[derive(Debug)]
struct Parallel {
    pos_diff: i64,
    velocity: i64,
}

impl Parallel {
    fn new(pos_diff: i64, velocity: i64) -> Self {
        Self { pos_diff: abs(pos_diff), velocity }
    }

    fn intersecting_velocities(&self) -> HashSet<i64> {
        let mut velocities = HashSet::new();
        for i in 1..=sqrt(self.pos_diff) {
            if self.pos_diff % i != 0 { continue; }
            velocities.insert(self.velocity + i);
            velocities.insert(self.velocity - i);
            velocities.insert(self.velocity + self.pos_diff / i);
            velocities.insert(self.velocity - self.pos_diff / i);
        }
        velocities
    }
}

struct Range2 {
    x: Range<f64>,
    y: Range<f64>,
}

impl Range2 {
    fn contains(&self, float: Float2) -> bool {
        self.x.contains(&float.x) && self.y.contains(&float.y)
    }
}

#[derive(Copy, Clone)]
struct Float2 {
    x: f64,
    y: f64,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Int3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Debug for Int3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:3}, {:3}, {:3}", self.x, self.y, self.z))
    }
}

impl Mul<i64> for Int3 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Add for Int3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for Int3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day24Solver {
    Advent2023Day24Solver::new(String::from("\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"))
}

#[test]
fn counts_future_hailstone_crossings() {
    let solver = test_solver_1();
    let solution = solver.hailstorm.crossing_count(Range2 { x: 7f64..27f64, y: 7f64..27f64 });
    assert_eq!(solution, 2);
}

#[test]
fn finds_rock_hitting_everything() {
    let solver = test_solver_1();
    let rock = solver.hailstorm.rock_hitting_everything();
    assert_eq!(rock.position.x, 24);
    assert_eq!(rock.position.y, 13);
    assert_eq!(rock.position.z, 10);
    assert_eq!(rock.velocity.x, -3);
    assert_eq!(rock.velocity.y, 1);
    assert_eq!(rock.velocity.z, 2);
}
