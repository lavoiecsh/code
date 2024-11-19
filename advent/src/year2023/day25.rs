use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};

use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2023Day25Solver {
    connections: Vec<(String, String)>,
}

impl Advent2023Day25Solver {
    pub fn new(input: &str) -> Self {
        Self {
            connections: input
                .lines()
                .map(|l| l.split_once(": ").unwrap())
                .flat_map(|(from, tos)| tos.split(' ').map(|to| (from.to_string(), to.to_string())))
                .collect(),
        }
    }
}

impl AdventSolver for Advent2023Day25Solver {
    fn solve_part1(&self) -> usize {
        let graph = Graph::new(&self.connections);
        let mut visited_counts: HashMap<(usize, usize), usize> = HashMap::new();
        for node in &graph.nodes {
            let visited = graph.visited_edges(node.id);
            visited
                .into_iter()
                .for_each(|v| *visited_counts.entry(v).or_insert(0) += 1);
        }
        let most_visited = visited_counts
            .iter()
            .sorted_by_key(|(_, v)| *v)
            .rev()
            .take(100)
            .collect_vec();
        for p in most_visited.iter().map(|(k, _)| **k).combinations(3) {
            let groups = graph.groups(&p);
            let count = groups.iter().filter(|&g| *g == 0).count();
            if count != groups.len() {
                return count * (groups.len() - count);
            }
        }
        panic!("no solution found")
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new(connections: &Vec<(String, String)>) -> Self {
        let mut nodes: Vec<Node> = connections
            .iter()
            .flat_map(|(fr, to)| [fr, to])
            .unique()
            .enumerate()
            .map(|(i, n)| Node::new(i, n.to_string()))
            .collect();
        for (fr, to) in connections {
            let fri = nodes.iter().position(|n| &n.name == fr).unwrap();
            let toi = nodes.iter().position(|n| &n.name == to).unwrap();
            nodes[fri].links.push(toi);
            nodes[toi].links.push(fri);
        }
        Self { nodes }
    }

    fn visited_edges(&self, id: usize) -> Vec<(usize, usize)> {
        let mut visited: Vec<(usize, usize)> = vec![];
        let mut touched: HashSet<usize> = HashSet::new();
        touched.insert(id);
        let mut queue = vec![id];
        while touched.len() != self.nodes.len() {
            let mut untouched: Vec<usize> = vec![];
            while let Some(current) = queue.pop() {
                let untouched_links = self.nodes[current]
                    .links
                    .iter()
                    .filter(|l| !touched.contains(l))
                    .collect_vec();
                visited.extend(untouched_links.iter().map(|l| (current, **l)));
                untouched.extend(untouched_links);
            }
            touched.extend(untouched.iter());
            queue = untouched;
        }
        visited
            .into_iter()
            .map(|(f, t)| (usize::min(f, t), usize::max(f, t)))
            .collect_vec()
    }

    fn groups(&self, broken_links: &[(usize, usize)]) -> Vec<usize> {
        let mut created_groups = vec![];
        let mut group = (0..self.nodes.len()).collect_vec();
        while let Some(next_group) = group.iter().position(|g| !created_groups.contains(g)) {
            let mut queue: VecDeque<usize> = self.nodes[next_group]
                .links
                .iter()
                .filter(|&&l| {
                    !broken_links.contains(&(next_group, l))
                        && !broken_links.contains(&(l, next_group))
                })
                .cloned()
                .collect();
            while let Some(current) = queue.pop_front() {
                if group[current] == next_group {
                    continue;
                }
                group[current] = next_group;
                queue.extend(self.nodes[current].links.iter().filter(|&&l| {
                    !broken_links.contains(&(current, l)) && !broken_links.contains(&(l, current))
                }));
            }
            created_groups.push(next_group);
        }
        group
    }
}

struct Node {
    id: usize,
    name: String,
    links: Vec<usize>,
}

impl Node {
    fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            links: vec![],
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:4} ({:3}) = {:2} -> {}",
            self.id,
            self.name,
            self.links.len(),
            self.links.iter().join(", ")
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

    #[test]
    fn finds_2_groups_with_3_cuts() {
        let solver = Advent2023Day25Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 54);
    }
}
