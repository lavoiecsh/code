use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Advent2024Day23Solver {
    network_map: NetworkMap,
}

impl Advent2024Day23Solver {
    pub fn new(input: &str) -> Self {
        let mut network_map = NetworkMap::new();
        input.lines().for_each(|l| {
            let mut s = l.split('-');
            let c1 = s.next().unwrap().chars().collect_vec();
            let c2 = s.next().unwrap().chars().collect_vec();
            network_map.add_connection((c1[0], c1[1]), (c2[0], c2[1]));
        });
        Self { network_map }
    }
}

impl AdventSolver for Advent2024Day23Solver {
    fn solve_part1(&self) -> usize {
        self.network_map
            .groups()
            .into_iter()
            .filter(|g| g.contains_chief_historian())
            .count()
    }

    fn solve_part2_string(&self) -> String {
        self.network_map
            .largest_group()
            .password()
    }
}

type Computer = (char, char);

struct NetworkMap {
    computers: HashSet<Computer>,
    connections: HashMap<Computer, Vec<Computer>>,
    initial_groups: Vec<ComputerGroup>,
}

impl NetworkMap {
    fn new() -> Self {
        Self {
            computers: HashSet::new(),
            connections: HashMap::new(),
            initial_groups: Vec::new(),
        }
    }

    fn add_connection(&mut self, source: Computer, target: Computer) {
        self.computers.insert(source);
        self.computers.insert(target);
        self.connections
            .entry(source)
            .and_modify(|c| c.push(target))
            .or_insert_with(|| vec![target]);
        self.connections
            .entry(target)
            .and_modify(|c| c.push(source))
            .or_insert_with(|| vec![source]);
        self.initial_groups.push(ComputerGroup::new(source, target));
    }

    fn groups(&self) -> Vec<ComputerGroup> {
        self.augment_groups(&self.initial_groups)
    }

    fn largest_group(&self) -> ComputerGroup {
        let mut groups = self.initial_groups.clone();
        let mut next_groups = self.augment_groups(&groups);
        while !next_groups.is_empty() {
            groups = next_groups;
            next_groups = self.augment_groups(&groups);
        }
        groups[0].clone()
    }

    fn augment_groups(&self, groups: &Vec<ComputerGroup>) -> Vec<ComputerGroup> {
        let mut new_groups = Vec::new();
        for g in groups {
            for (c, connections) in &self.connections {
                if let Some(new_group) = g.augment(*c) {
                    if g.computers.iter().all(|c2| connections.contains(c2)) {
                        new_groups.push(new_group);
                    }
                }
            }
        }
        new_groups
    }
}

#[derive(Clone, Eq, PartialEq)]
struct ComputerGroup {
    computers: Vec<Computer>,
}

impl ComputerGroup {
    fn new(computer_1: Computer, computer_2: Computer) -> Self {
        let mut computers = vec![computer_1, computer_2];
        computers.sort();
        Self { computers }
    }

    fn augment(&self, computer: Computer) -> Option<Self> {
        if self.computers.last()? >= &computer {
            return None;
        }
        let mut clone = self.clone();
        clone.computers.push(computer);
        Some(clone)
    }

    fn contains_chief_historian(&self) -> bool {
        self.computers.iter().any(|c| c.0 == 't')
    }

    fn password(&self) -> String {
        self.computers
            .iter()
            .map(|(a,b)| format!("{a}{b}"))
            .join(",")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn finds_connected_computers() {
        let solver = Advent2024Day23Solver::new(EXAMPLE);
        assert_eq!(solver.network_map.groups().len(), 12);
    }

    #[test]
    fn finds_connected_computers_with_starting_t() {
        let solver = Advent2024Day23Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 7);
    }

    #[test]
    fn finds_largest_group() {
        let solver = Advent2024Day23Solver::new(EXAMPLE);
        assert_eq!(solver.network_map.largest_group().computers.len(), 4);
    }

    #[test]
    fn finds_password_to_lan_party() {
        let solver = Advent2024Day23Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part2_string(), "co,de,ka,ta");
    }
}
