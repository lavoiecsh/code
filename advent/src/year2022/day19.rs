use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::fs::read_to_string;
use itertools::Itertools;

use regex::Regex;

use crate::solver::AdventSolver;

pub struct Advent2022Day19Solver {
    blueprints: Vec<Blueprint>
}

impl Advent2022Day19Solver {
    pub fn new() -> Self {
        let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        Self {
            blueprints: read_to_string("src/year2022/day19.txt")
                .unwrap()
                .lines()
                .map(|l| {
                    let cap = re.captures(l).unwrap();
                    let get = |n| cap.get(n).unwrap().as_str().parse().unwrap();
                    Blueprint {
                        number: get(1),
                        ore_robot_ore_cost: get(2),
                        clay_robot_ore_cost: get(3),
                        obsidian_robot_ore_cost: get(4),
                        obsidian_robot_clay_cost: get(5),
                        geode_robot_ore_cost: get(6),
                        geode_robot_obsidian_cost: get(7),
                    }
                })
                .collect()
        }
    }
}

struct Blueprint {
    number: usize,
    ore_robot_ore_cost: usize,
    clay_robot_ore_cost: usize,
    obsidian_robot_ore_cost: usize,
    obsidian_robot_clay_cost: usize,
    geode_robot_ore_cost: usize,
    geode_robot_obsidian_cost: usize,
}

impl Blueprint {
    fn geode_count(&self, length: usize) -> usize {
        let mut states: HashSet<State> = HashSet::new();
        states.insert(State::new());
        for _ in 0..length {
            let next_states: HashSet<State> = states.iter().flat_map(|s| s.next_states(&self)).collect();
            states = next_states
                .iter()
                .sorted_by(|a,b| State::cmp(b,a))
                .take(100000)
                .map(|s| s.clone())
                .collect();
        }
        states.iter().map(|s| s.geode_count).max().unwrap()
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    ore_count: usize,
    clay_count: usize,
    obsidian_count: usize,
    geode_count: usize,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("States")
            .field("count", &(self.ore_count, self.clay_count, self.obsidian_count, self.geode_count))
            .field("robots", &(self.ore_robots, self.clay_robots, self.obsidian_robots, self.geode_robots))
            .finish()
    }
}

impl State {
    fn new() -> Self {
        Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,
            geode_count: 0,
        }
    }

    fn cmp(&self, other: &Self) -> Ordering {
        usize::cmp(&self.geode_count, &other.geode_count)
            .then(usize::cmp(&self.geode_robots, &other.geode_robots))
            .then(usize::cmp(&self.obsidian_robots, &other.obsidian_robots))
            .then(usize::cmp(&self.clay_robots, &other.clay_robots))
            .then(usize::cmp(&self.ore_robots, &other.ore_robots))
    }

    fn next(&self) -> Self {
        Self {
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
            ore_count: self.ore_count + self.ore_robots,
            clay_count: self.clay_count + self.clay_robots,
            obsidian_count: self.obsidian_count + self.obsidian_robots,
            geode_count: self.geode_count + self.geode_robots,
        }
    }

    fn next_states(&self, blueprint: &Blueprint) -> Vec<State> {
        let mut nexts = vec!();
        let next = self.next();
        nexts.push(next.clone());
        if self.ore_count >= blueprint.ore_robot_ore_cost {
            let mut tmp = next.clone();
            tmp.ore_robots += 1;
            tmp.ore_count -= blueprint.ore_robot_ore_cost;
            nexts.push(tmp);
        }
        if self.ore_count >= blueprint.clay_robot_ore_cost {
            let mut tmp = next.clone();
            tmp.clay_robots += 1;
            tmp.ore_count -= blueprint.clay_robot_ore_cost;
            nexts.push(tmp);
        }
        if self.ore_count >= blueprint.obsidian_robot_ore_cost && self.clay_count >= blueprint.obsidian_robot_clay_cost {
            let mut tmp = next.clone();
            tmp.obsidian_robots += 1;
            tmp.ore_count -= blueprint.obsidian_robot_ore_cost;
            tmp.clay_count -= blueprint.obsidian_robot_clay_cost;
            nexts.push(tmp);
        }
        if self.ore_count >= blueprint.geode_robot_ore_cost && self.obsidian_count >= blueprint.geode_robot_obsidian_cost {
            let mut tmp = next.clone();
            tmp.geode_robots += 1;
            tmp.ore_count -= blueprint.geode_robot_ore_cost;
            tmp.obsidian_count -= blueprint.geode_robot_obsidian_cost;
            nexts.push(tmp);
        }
        nexts
    }
}

impl AdventSolver for Advent2022Day19Solver {
    fn day(&self) -> usize { 19 }
    fn year(&self) -> usize { 2022 }

    fn solve_part1(&self) -> usize {
        self.blueprints
            .iter()
            .map(|b| b.geode_count(24) * b.number)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.blueprints
            .iter()
            .take(3)
            .map(|b| b.geode_count(32))
            .fold(1, |acc, cur| acc * cur)
    }
}
