use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};

use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2023Day20Solver {
    broadcaster: Vec<ModuleName>,
    flip_flops: HashMap<ModuleName, Vec<ModuleName>>,
    conjunctions: HashMap<ModuleName, Vec<ModuleName>>,
}

impl Advent2023Day20Solver {
    pub fn new(input: String) -> Self {
        let mut s = Self {
            broadcaster: vec!(),
            flip_flops: HashMap::new(),
            conjunctions: HashMap::new(),
        };
        for line in input.lines() {
            let l = line.split(" -> ").collect_vec();
            let mut n = l[0].chars();
            let d = l[1].split(", ").map(String::from).collect_vec();
            match n.next().unwrap() {
                'b' => { s.broadcaster = d; }
                '%' => { s.flip_flops.insert(n.as_str().to_string(), d); }
                '&' => { s.conjunctions.insert(n.as_str().to_string(), d); }
                _ => panic!("unknown module name {line}"),
            }
        }
        s
    }

    fn new_configuration(&self) -> ModuleConfiguration {
        let mut modules: HashMap<ModuleName, Box<dyn Module>> = HashMap::new();
        modules.insert(String::from("broadcaster"), Box::new(Broadcaster::new(self.broadcaster.clone())));
        let rx_conjunction = self.conjunctions.iter().find(|(_, d)| d.contains(&String::from("rx")));
        let rx_requirements = if let Some((rx_conjunction, _)) = rx_conjunction {
            self.conjunctions.iter().filter(|(_, d)| d.contains(rx_conjunction)).map(|(n, _)| n.clone()).collect()
        } else { Vec::new() };
        for (ffn, ffd) in &self.flip_flops {
            modules.insert(ffn.to_string(), Box::new(FlipFlop::new(ffd.clone())));
        }
        for (cn, cd) in &self.conjunctions {
            let mut inputs = vec!();
            if self.broadcaster.contains(cn) {
                inputs.push(String::from("broadcaster"));
            }
            inputs.extend(self.flip_flops.iter().filter(|(_, d)| d.contains(cn)).map(|(n, _)| n.clone()));
            inputs.extend(self.conjunctions.iter().filter(|(_, d)| d.contains(cn)).map(|(n, _)| n.clone()));
            modules.insert(cn.to_string(), Box::new(Conjunction::new(inputs, cd.clone())));
        }
        ModuleConfiguration::new(modules, rx_requirements)
    }
}

impl AdventSolver for Advent2023Day20Solver {
    fn solve_part1(&self) -> usize {
        let mut configuration = self.new_configuration();
        (0..1000).for_each(|_| configuration.push_button());
        configuration.low * configuration.high
    }

    fn solve_part2(&self) -> usize {
        let mut configuration = self.new_configuration();
        while configuration.rx_low_button_presses().is_none() {
            configuration.push_button();
        }
        configuration.rx_low_button_presses().unwrap()
    }
}

type ModuleName = String;

struct ModuleConfiguration {
    modules: HashMap<ModuleName, Box<dyn Module>>,
    low: usize,
    high: usize,
    button_presses: usize,
    rx_requirements: HashMap<String, Option<usize>>,
}

impl ModuleConfiguration {
    fn new(modules: HashMap<ModuleName, Box<dyn Module>>, rx_requirements: Vec<String>) -> Self {
        Self {
            modules,
            low: 0,
            high: 0,
            button_presses: 0,
            rx_requirements: rx_requirements.into_iter().map(|r| (r, None)).collect(),
        }
    }

    fn rx_low_button_presses(&self) -> Option<usize> {
        if self.rx_requirements.values().all(|v| v.is_some()) {
            Some(self.rx_requirements.values().fold(1, |a, c| a * c.unwrap()))
        } else {
            None
        }
    }

    fn push_button(&mut self) {
        self.button_presses += 1;
        let mut queue = VecDeque::new();
        queue.push_back(Pulse { from: String::from("button"), high: false, to: String::from("broadcaster") });
        while let Some(current) = queue.pop_front() {
            if current.high {
                self.high += 1;
                if let Some(v) = self.rx_requirements.get(&current.from) {
                    if v.is_none() {
                        self.rx_requirements.insert(current.from.clone(), Some(self.button_presses));
                    }
                }
            } else {
                self.low += 1;
            }
            if let Some(module) = self.modules.get_mut(&current.to) {
                queue.extend(module.receive(current));
            }
        }
    }
}

trait Module {
    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse>;
}

struct Broadcaster {
    destinations: Vec<ModuleName>,
}

impl Broadcaster {
    fn new(destinations: Vec<ModuleName>) -> Self {
        Self { destinations }
    }
}

impl Module for Broadcaster {
    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.destinations.iter()
            .map(|d| pulse.next(pulse.high, d))
            .collect()
    }
}

struct FlipFlop {
    high: bool,
    destinations: Vec<ModuleName>,
}

impl FlipFlop {
    fn new(destinations: Vec<ModuleName>) -> Self {
        Self { high: false, destinations }
    }
}

impl Module for FlipFlop {
    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse> {
        if pulse.high { return vec!(); }
        self.high = !self.high;
        self.destinations.iter()
            .map(|d| pulse.next(self.high, d))
            .collect()
    }
}

struct Conjunction {
    lasts: HashMap<ModuleName, bool>,
    destinations: Vec<ModuleName>,
}

impl Conjunction {
    fn new(inputs: Vec<ModuleName>, destinations: Vec<ModuleName>) -> Self {
        Self { lasts: inputs.into_iter().map(|mn| (mn, false)).collect(), destinations }
    }
}

impl Module for Conjunction {
    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.lasts.insert(pulse.from.clone(), pulse.high);
        let high = self.lasts.values().all(|&h| h);
        self.destinations.iter()
            .map(|d| pulse.next(!high, d))
            .collect()
    }
}

struct Pulse {
    from: ModuleName,
    high: bool,
    to: ModuleName,
}

impl Pulse {
    fn next(&self, high: bool, destination: &ModuleName) -> Self {
        Self { from: self.to.clone(), high, to: destination.clone() }
    }
}

impl Debug for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} -{}-> {}", self.from, if self.high { "high" } else { "low-" }, self.to))
    }
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day20Solver {
    Advent2023Day20Solver::new(String::from("\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"))
}

#[test]
fn counts_low_high_pulses() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part1(), 11687500);
}
