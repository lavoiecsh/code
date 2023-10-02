use std::collections::{HashMap, HashSet};
use std::slice::Iter;

use itertools::Itertools;
use regex::Regex;

use crate::solver::AdventSolver;

struct Valve {
    flow_rate: usize,
    leads_to: Vec<String>,
}

type ValveMap = HashMap<String, Valve>;

pub struct Advent2022Day16Solver {
    valves: ValveMap,
    valves_with_flow_rate: Vec<String>,
}

impl Advent2022Day16Solver {
    pub fn new(input: String) -> Self {
        let re = Regex::new(r"Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
        let valves: ValveMap = input
            .lines()
            .map(|l| {
                let cap = re.captures(l).unwrap();
                (cap.get(1).unwrap().as_str().to_string(),
                 Valve {
                     flow_rate: cap.get(2).unwrap().as_str().parse().unwrap(),
                     leads_to: cap.get(3).unwrap().as_str().split(", ").map(String::from).collect(),
                 })
            })
            .collect();
        let valves_with_flow_rate = valves.iter().filter(|(_, v)| v.flow_rate != 0).map(|(n, _)| n.clone()).collect();
        Self {
            valves,
            valves_with_flow_rate,
        }
    }

    fn flow_rate(&self, valve: &String) -> usize {
        self.valves.get(valve).unwrap().flow_rate
    }

    fn leads_to(&self, valve: &String) -> Iter<String> {
        self.valves.get(valve).unwrap().leads_to.iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    pressure: usize,
    pressure_total: usize,
    open_valves: Vec<String>,
    position: String,
    elephant: String,
}

impl State {
    fn new() -> Self {
        Self { pressure: 0, pressure_total: 0, open_valves: vec!(), position: String::from("AA"), elephant: String::from("AA") }
    }

    fn do_nothing(&self) -> Self {
        Self {
            pressure: self.pressure,
            pressure_total: self.pressure_total + self.pressure,
            open_valves: self.open_valves.clone(),
            position: self.position.clone(),
            elephant: self.elephant.clone(),
        }
    }

    fn is_done(&self, solver: &Advent2022Day16Solver) -> bool {
        self.open_valves.len() == solver.valves_with_flow_rate.len()
    }

    fn can_open(&self, solver: &Advent2022Day16Solver, valve: &String) -> bool {
        !self.open_valves.contains(valve) && solver.valves_with_flow_rate.contains(valve)
    }

    fn next_states_alone(&self, solver: &Advent2022Day16Solver) -> Vec<Self> {
        let mut nexts = vec!();
        let next = self.do_nothing();
        nexts.push(next.clone());
        if self.is_done(solver) {
            return nexts;
        }
        if self.can_open(solver, &self.position) {
            let mut tmp = next.clone();
            tmp.open_valves.push(self.position.clone());
            tmp.pressure += solver.flow_rate(&self.position);
            nexts.push(tmp);
        }
        for np in solver.leads_to(&self.position) {
            let mut tmp = next.clone();
            tmp.position = np.clone();
            nexts.push(tmp);
        }
        nexts
    }

    fn open_valve(&self, solver: &Advent2022Day16Solver, valve: &String) -> Self {
        let mut tmp = self.clone();
        tmp.open_valves.push(valve.clone());
        tmp.pressure += solver.flow_rate(valve);
        tmp
    }

    fn move_position_to(&self, valve: &String) -> Self {
        let mut tmp = self.clone();
        tmp.position = valve.clone();
        tmp
    }

    fn move_elephant_to(&self, valve: &String) -> Self {
        let mut tmp = self.clone();
        tmp.elephant = valve.clone();
        tmp
    }

    fn next_states(&self, solver: &Advent2022Day16Solver) -> Vec<Self> {
        if self.is_done(solver) {
            return vec!(self.do_nothing());
        }
        let mut nexts = vec!();
        // nothing + nothing
        let next = self.do_nothing();
        nexts.push(next.clone());
        if self.can_open(solver, &self.position) {
            // open + nothing
            let tmp = next.open_valve(solver, &self.position);
            nexts.push(tmp.clone());

            // open + open
            if self.position != self.elephant && self.can_open(solver, &self.elephant) {
                nexts.push(tmp.open_valve(solver, &self.elephant));
            }

            // open + move
            for ne in solver.leads_to(&self.elephant) {
                nexts.push(tmp.move_elephant_to(ne));
            }
        }
        if self.can_open(solver, &self.elephant) {
            // nothing + open
            let tmp = next.open_valve(solver, &self.elephant);
            nexts.push(tmp.clone());

            // move + open
            for np in solver.leads_to(&self.position) {
                nexts.push(tmp.move_position_to(np));
            }
        }
        // move + nothing
        for np in solver.leads_to(&self.position) {
            let tmp = next.move_position_to(np);
            nexts.push(tmp.clone());

            // move + move
            for ne in solver.leads_to(&self.elephant) {
                nexts.push(tmp.move_elephant_to(ne));
            }
        }
        // nothing + move
        for ne in solver.leads_to(&self.elephant) {
            nexts.push(next.move_elephant_to(ne));
        }
        nexts
    }
}

impl AdventSolver for Advent2022Day16Solver {
    fn day(&self) -> usize { 16 }
    fn year(&self) -> usize { 2022 }

    fn solve_part1(&self) -> usize {
        let mut all_states = vec!(State::new());
        for _ in 0..30 {
            let new_states: HashSet<State> = all_states
                .iter()
                .flat_map(|s| s.next_states_alone(&self))
                .collect();
            all_states = new_states
                .iter()
                .sorted_by(|l, r| usize::cmp(&r.pressure_total, &l.pressure_total))
                .take(100000)
                .map(|s| s.clone())
                .collect();
        }
        all_states
            .iter()
            .map(|s| s.pressure_total)
            .max()
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        let mut all_states = vec!(State::new());
        for i in 0..26 {
            let cutoff = if i < 15 { 1000000 } else { 100000 };
            let new_states: HashSet<State> = all_states
                .iter()
                .flat_map(|s| s.next_states(&self))
                .collect();
            all_states = new_states
                .iter()
                .sorted_by(|l, r| usize::cmp(&r.pressure, &l.pressure)
                    .then(usize::cmp(&r.pressure_total, &l.pressure_total)))
                .take(cutoff)
                .map(|s| s.clone())
                .collect();
        }
        all_states
            .iter()
            .map(|s| s.pressure_total)
            .max()
            .unwrap()
    }
}
