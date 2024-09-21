use std::collections::{HashMap, VecDeque};

use regex::Regex;

use crate::solver::AdventSolver;

pub struct Advent2016Day10Solver {
    inputs: Vec<(u16, Destination)>,
    sends: HashMap<u16, (Destination, Destination)>,
}

impl Advent2016Day10Solver {
    pub fn new(input: String) -> Self {
        let input_re = Regex::new(r"value (\d+) goes to (bot|output) (\d+)").unwrap();
        let sends_re = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();
        let mut inputs = Vec::new();
        let mut sends = HashMap::new();
        input.lines()
            .for_each(|l| {
                if let Some(cap) = input_re.captures(l) {
                    inputs.push((cap.get(1).unwrap().as_str().parse().unwrap(), Destination::new(cap.get(2).unwrap().as_str(), cap.get(3).unwrap().as_str())));
                } else if let Some(cap) = sends_re.captures(l) {
                    sends.insert(cap.get(1).unwrap().as_str().parse().unwrap(), (
                        Destination::new(cap.get(2).unwrap().as_str(), cap.get(3).unwrap().as_str()),
                        Destination::new(cap.get(4).unwrap().as_str(), cap.get(5).unwrap().as_str())
                    ));
                }
            });
        Self { inputs, sends }
    }
}

impl AdventSolver for Advent2016Day10Solver {
    fn solve_part1(&self) -> usize {
        let mut factory = Factory::new(self);
        self.inputs.iter()
            .fold(None, |acc,(value, destination)| {
                if acc.is_some() {
                    return acc
                }
                let comparisons = factory.add_value(*value, destination);
                comparisons.iter()
                    .find(|(_,l,h)| *l == 17 && *h == 61)
                    .cloned()
            })
            .unwrap()
            .0 as usize
    }

    fn solve_part2(&self) -> usize {
        let mut factory = Factory::new(self);
        self.inputs.iter()
            .for_each(|(value, destination)| { factory.add_value(*value, destination); });
        (0..=2)
            .map(|i| factory.outputs.get(&i).unwrap().first().unwrap())
            .fold(1usize, |acc,cur| acc * *cur as usize)
    }
}

#[derive(Clone)]
enum Destination {
    Bot(u16),
    Output(u16),
}

impl Destination {
    fn new(destination_type: &str, destination_number: &str) -> Self {
        match destination_type {
            "bot" => Destination::Bot(destination_number.parse().unwrap()),
            "output" => Destination::Output(destination_number.parse().unwrap()),
            x => panic!("unknown destination type {x}"),
        }
    }
}

struct Factory {
    bots: HashMap<u16, Bot>,
    outputs: HashMap<u16, Vec<u16>>,
}

impl Factory {
    fn new(solver: &Advent2016Day10Solver) -> Self {
        let mut bots = HashMap::new();
        let mut outputs = HashMap::new();
        solver.sends.iter()
            .for_each(|(b, (l,h))| {
                bots.insert(*b, Bot::new(*b, l.clone(), h.clone()));
                if let Destination::Output(output) = l {
                    outputs.insert(*output, Vec::new());
                }
                if let Destination::Output(output) = h {
                    outputs.insert(*output, Vec::new());
                }
            });
        solver.inputs.iter()
            .for_each(|(_, d)| {
                if let Destination::Output(output) = d {
                    outputs.insert(*output, Vec::new());
                }
            });
        Self { bots, outputs }
    }

    fn add_value(&mut self, value: u16, destination: &Destination) -> Vec<(u16, u16, u16)> {
        let mut full_queue = VecDeque::new();
        self.add_value_rec(value, destination, &mut full_queue);
        let mut comparisons = Vec::new();
        while !full_queue.is_empty() {
            let bot_id = full_queue.pop_front().unwrap();
            let bot = self.bots.get_mut(&bot_id).unwrap();
            let ((low_value, low_destination), (high_value, high_destination)) = bot.empty();
            comparisons.push((bot_id, low_value, high_value));
            self.add_value_rec(low_value, &low_destination, &mut full_queue);
            self.add_value_rec(high_value, &high_destination, &mut full_queue);
        }
        comparisons
    }

    fn add_value_rec(&mut self, value: u16, destination: &Destination, full_queue: &mut VecDeque<u16>) {
        match destination {
            Destination::Bot(x) => {
                let bot = self.bots.get_mut(x).unwrap();
                if bot.add(value) {
                    full_queue.push_back(bot.id);
                }
            },
            Destination::Output(x) => { self.outputs.get_mut(x).unwrap().push(value); }
        };
    }
}

struct Bot {
    id: u16,
    numbers: Vec<u16>,
    low: Destination,
    high: Destination,
}

impl Bot {
    fn new(id: u16, low: Destination, high: Destination) -> Self {
        Self { id, numbers: Vec::new(), low, high }
    }

    fn add(&mut self, value: u16) -> bool {
        self.numbers.push(value);
        self.numbers.len() == 2
    }

    fn empty(&mut self) -> ((u16, Destination), (u16, Destination)) {
        let low_index = if self.numbers[0] < self.numbers[1] { 0 } else { 1 };
        let high_index = (low_index + 1) % 2;
        let low_value = self.numbers[low_index];
        let high_value = self.numbers[high_index];
        self.numbers.clear();
        ((low_value, self.low.clone()), (high_value, self.high.clone()))
    }
}
