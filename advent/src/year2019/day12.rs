use crate::solver::AdventSolver;
use num_traits::Inv;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::AddAssign;
use num_integer::Integer;

pub struct Advent2019Day12Solver {
    moons: Vec<Moon>,
}

impl Advent2019Day12Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new("<x=(-?\\d+), y=(-?\\d+), z=(-?\\d+)>").unwrap();
        Self {
            moons: input.lines()
                .map(|l| {
                    let m = re.captures(l).unwrap();
                    Moon::new(m[1].parse().unwrap(), m[2].parse().unwrap(), m[3].parse().unwrap())
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2019Day12Solver {
    fn solve_part1(&self) -> usize {
        let mut system = MoonSystem::new(self.moons.clone());
        system.simulate(1000);
        system.total_energy() as usize
    }

    fn solve_part2(&self) -> usize {
        let mut system = MoonSystem::new(self.moons.clone());
        let mut memory = vec![
            MoonMemory::new(|c| c.x),
            MoonMemory::new(|c| c.y),
            MoonMemory::new(|c| c.z),
        ];
        memory.iter_mut().for_each(|m| m.evaluate(&system.moons, 0));
        let mut index = 0;
        let step = 1;
        while memory.iter().any(|m| !m.has_solution()) {
            index += step;
            system.simulate(step);
            memory.iter_mut().for_each(|m| m.evaluate(&system.moons, index));
        }
        memory[0].repeat.unwrap()
            .lcm(&memory[1].repeat.unwrap())
            .lcm(&memory[2].repeat.unwrap())
    }
}

#[derive(Debug, Clone)]
struct MoonMemory {
    coord_fn: fn (Coord) -> i64,
    start: Option<usize>,
    repeat: Option<usize>,
    seen: HashMap<Vec<(i64,i64)>, usize>,
}

impl MoonMemory {
    fn new(coord_fn: fn (Coord) -> i64) -> Self {
        Self {
            coord_fn,
            start: None,
            repeat: None,
            seen: HashMap::new(),
        }
    }

    fn evaluate(&mut self, moons: &[Moon], step: usize) {
        if self.has_solution() {
            return;
        }
        let eval = moons.iter()
            .map(|m| ((self.coord_fn)(m.pos), (self.coord_fn)(m.vel)))
            .collect();
        if let Some(&found) = self.seen.get(&eval) {
            self.start = Some(found);
            self.repeat = Some(step - found);
        } else {
            self.seen.insert(eval, step);
        }
    }

    fn has_solution(&self) -> bool {
        self.start.is_some() && self.repeat.is_some()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct MoonSystem {
    moons: Vec<Moon>,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Moon {
    pos: Coord,
    vel: Coord,
}
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl MoonSystem {
    fn new(moons: Vec<Moon>) -> Self {
        Self {
            moons,
        }
    }

    fn simulate(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }

    fn step(&mut self) {
        self.apply_gravity();
        self.apply_velocity();
    }

    fn apply_gravity(&mut self) {
        for m1 in 0..self.moons.len() {
            for m2 in m1+1..self.moons.len() {
                let gravity_effect = self.moons[m1].calculate_gravity(&self.moons[m2]);
                self.moons[m1].apply_gravity(gravity_effect);
                self.moons[m2].apply_gravity(gravity_effect.inv());
            }
        }
    }

    fn apply_velocity(&mut self) {
        self.moons.iter_mut()
            .for_each(|m| m.apply_velocity())
    }

    fn total_energy(&self) -> i64 {
        self.moons.iter()
            .map(Moon::energy)
            .sum()
    }
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            pos: Coord { x, y, z },
            vel: Coord { x: 0, y: 0, z: 0 },
        }
    }

    fn calculate_gravity(&self, rhs: &Self) -> Coord {
        const TO_EFFECT: fn(Ordering) -> i64 = |o| match o {
            Ordering::Equal => 0,
            Ordering::Greater => -1,
            Ordering::Less => 1,
        };
        Coord {
            x: TO_EFFECT(self.pos.x.cmp(&rhs.pos.x)),
            y: TO_EFFECT(self.pos.y.cmp(&rhs.pos.y)),
            z: TO_EFFECT(self.pos.z.cmp(&rhs.pos.z)),
        }
    }

    fn apply_gravity(&mut self, effect: Coord) {
        self.vel += effect;
    }

    fn apply_velocity(&mut self) {
        self.pos += self.vel;
    }

    fn energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn potential_energy(&self) -> i64 {
        self.pos.energy()
    }

    fn kinetic_energy(&self) -> i64 {
        self.vel.energy()
    }
}

impl Coord {
    fn energy(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Inv for Coord {
    type Output = Self;

    fn inv(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";
    const EXAMPLE_2: &str = "\
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
";

    #[test]
    fn simulates_moons_movements() {
        let solver_1 = Advent2019Day12Solver::new(EXAMPLE_1);
        let mut system_1 = MoonSystem::new(solver_1.moons.clone());
        system_1.simulate(10);
        assert_eq!(179, system_1.total_energy());

        let solver_2 = Advent2019Day12Solver::new(EXAMPLE_2);
        let mut system_2 = MoonSystem::new(solver_2.moons.clone());
        system_2.simulate(100);
        assert_eq!(1940, system_2.total_energy());
    }

    #[test]
    fn counts_step_to_repeat() {
        let solver_1 = Advent2019Day12Solver::new(EXAMPLE_1);
        assert_eq!(2772, solver_1.solve_part2());

        let solver_2 = Advent2019Day12Solver::new(EXAMPLE_2);
        assert_eq!(4686774924, solver_2.solve_part2());
    }
}