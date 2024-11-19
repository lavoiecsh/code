use std::collections::VecDeque;
use std::ops::AddAssign;

use itertools::Itertools;
use num_traits::abs;
use regex::{Match, Regex};

use crate::solver::AdventSolver;

pub struct Advent2017Day20Solver {
    particles: Vec<Particle>,
}

impl Advent2017Day20Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();
        Self {
            particles: input
                .lines()
                .filter_map(|l| re.captures(l))
                .enumerate()
                .map(|(i, c)| {
                    Particle::new(
                        i,
                        Vector::new(to_isize(c.get(1)), to_isize(c.get(2)), to_isize(c.get(3))),
                        Vector::new(to_isize(c.get(4)), to_isize(c.get(5)), to_isize(c.get(6))),
                        Vector::new(to_isize(c.get(7)), to_isize(c.get(8)), to_isize(c.get(9))),
                    )
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2017Day20Solver {
    fn solve_part1(&self) -> usize {
        const COUNT: usize = 1000;
        let mut particles = self.particles.clone();
        let mut closests: VecDeque<usize> = VecDeque::new();
        while closests.len() < COUNT || !closests.iter().all_equal() {
            particles.iter_mut().for_each(Particle::update);
            if closests.len() == COUNT {
                closests.pop_front();
            }
            closests.push_back(
                particles
                    .iter()
                    .map(Particle::distance_to_zero)
                    .position_min()
                    .unwrap(),
            );
        }
        closests[0]
    }

    fn solve_part2(&self) -> usize {
        const COUNT: usize = 100;
        let mut particles = self.particles.clone();
        let mut lengths: VecDeque<usize> = VecDeque::new();
        while lengths.len() < COUNT || !lengths.iter().all_equal() {
            particles.iter_mut().for_each(Particle::update);
            particles = particles
                .iter()
                .filter(|p| !particles.iter().any(|p2| p.collides_with(p2)))
                .cloned()
                .collect();
            if lengths.len() == COUNT {
                lengths.pop_front();
            }
            lengths.push_back(particles.len());
        }
        lengths[0]
    }
}

#[derive(Clone)]
struct Particle {
    index: usize,
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
}

impl Particle {
    fn new(index: usize, position: Vector, velocity: Vector, acceleration: Vector) -> Self {
        Self {
            index,
            position,
            velocity,
            acceleration,
        }
    }

    fn update(&mut self) {
        self.velocity += &self.acceleration;
        self.position += &self.velocity;
    }

    fn distance_to_zero(&self) -> isize {
        self.position.distance_to_zero()
    }

    fn collides_with(&self, other: &Self) -> bool {
        self.index != other.index && self.position == other.position
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl Vector {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn distance_to_zero(&self) -> isize {
        abs(self.x) + abs(self.y) + abs(self.z)
    }
}

impl AddAssign<&Vector> for Vector {
    fn add_assign(&mut self, rhs: &Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

fn to_isize(value: Option<Match>) -> isize {
    value.unwrap().as_str().parse().unwrap()
}
