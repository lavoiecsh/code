use crate::solver::AdventSolver;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Write};

pub struct Advent2025Day11Solver {
    graph: Graph,
}

impl Advent2025Day11Solver {
    pub fn new(input: &str) -> Self {
        Self {
            graph: Graph {
                connections: input
                    .lines()
                    .map(|l| l.split_once(": ").unwrap())
                    .map(|(from, to)| (Node::from(from), to.split(' ').map(Node::from).collect()))
                    .collect(),
            },
        }
    }
}

impl AdventSolver for Advent2025Day11Solver {
    fn solve_part1(&self) -> usize {
        self.graph.count("you", "out")
    }

    fn solve_part2(&self) -> usize {
        let svr_fft = self.graph.count("svr", "fft");
        let fft_dac = self.graph.count("fft", "dac");
        let dac_out = self.graph.count("dac", "out");
        svr_fft * fft_dac * dac_out
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    name: [char; 3],
}
struct Graph {
    connections: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn count(&self, from: impl Into<Node>, to: impl Into<Node>) -> usize {
        let from = from.into();
        let to = to.into();
        let mut counts: HashMap<Node, usize> = HashMap::new();
        counts.insert(from, 1);
        self.count_rec(&mut counts, to);
        *counts.get(&to).unwrap()
    }

    fn count_rec(&self, counts: &mut HashMap<Node, usize>, to: Node) -> usize {
        if let Some(count) = counts.get(&to) {
            return *count;
        }
        let before = self.connections.iter()
            .filter(|(_,value)| value.contains(&to))
            .map(|(&key,_)| self.count_rec(counts, key))
            .sum();
        counts.insert(to, before);
        before
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        Self {
            name: [
                chars.next().unwrap(),
                chars.next().unwrap(),
                chars.next().unwrap(),
            ],
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.name[0])?;
        f.write_char(self.name[1])?;
        f.write_char(self.name[2])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const EXAMPLE_2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn counts_path_from_you_to_out() {
        let solver = Advent2025Day11Solver::new(EXAMPLE_1);
        assert_eq!(solver.solve_part1(), 5);
    }

    #[test]
    fn counts_path_from_svr_to_out_through_dac_and_fft() {
        let solver = Advent2025Day11Solver::new(EXAMPLE_2);
        assert_eq!(solver.solve_part2(), 2);
    }
}
