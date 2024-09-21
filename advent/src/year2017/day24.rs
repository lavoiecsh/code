use crate::solver::AdventSolver;

pub struct Advent2017Day24Solver {
    components: Vec<Component>,
}

impl Advent2017Day24Solver {
    pub fn new(input: String) -> Self {
        Self { components: input.lines().map(Component::new).collect() }
    }
}

impl AdventSolver for Advent2017Day24Solver {
    fn solve_part1(&self) -> usize {
        let bridge = Bridge::start(&self.components);
        bridge.highest_strength()
    }

    fn solve_part2(&self) -> usize {
        let bridge = Bridge::start(&self.components);
        bridge.longest_strength().1
    }
}

struct Bridge<'a> {
    components: &'a Vec<Component>,
    selected: Vec<(usize, usize)>,
    children: Vec<Bridge<'a>>,
}

impl<'a> Bridge<'a> {
    fn start(components: &'a Vec<Component>) -> Self {
        let mut start = Self {
            components,
            selected: Vec::new(),
            children: components.iter()
                .enumerate()
                .filter_map(|(i, c)| c.port_index(0)
                    .map(|p| Bridge { components, selected: vec!((i, p)), children: Vec::new() }))
                .collect(),
        };
        start.children.iter_mut().for_each(|c| c.calculate_children());
        start
    }

    fn calculate_children(&mut self) {
        let last = self.selected.iter().last().unwrap();
        let port = self.components[last.0].other_port(last.1);
        self.children = self.components.iter()
            .enumerate()
            .filter(|(i,_)| self.selected.iter().all(|(i2,_)| i != i2))
            .filter_map(|(i, c)| c.port_index(port)
                .map(|p| {
                    let mut selected = self.selected.clone();
                    selected.push((i,p));
                    Bridge { components: self.components, selected, children: Vec::new() }
                }))
            .collect();
        self.children.iter_mut().for_each(|c| c.calculate_children());
    }

    fn highest_strength(&self) -> usize {
        if self.children.is_empty() {
            self.strength()
        } else {
            self.children.iter().map(Bridge::highest_strength).max().unwrap()
        }
    }

    fn longest_strength(&self) -> (usize,usize) {
        if self.children.is_empty() {
            (self.selected.len(), self.strength())
        } else {
            self.children.iter().map(Bridge::longest_strength)
                .max_by(|l,r| l.0.cmp(&r.0).then(l.1.cmp(&r.1)))
                .unwrap()
        }
    }

    fn strength(&self) -> usize {
        self.selected.iter().map(|(i,_)| self.components[*i].strength()).sum()
    }
}

struct Component {
    ports: Vec<usize>,
}

impl Component {
    fn new(line: &str) -> Self {
        Self { ports: line.split("/").map(|p| p.parse().unwrap()).collect() }
    }

    fn port_index(&self, port: usize) -> Option<usize> {
        self.ports.iter().position(|p| *p == port)
    }

    fn other_port(&self, index: usize) -> usize {
        self.ports[(index + 1) % 2]
    }

    fn strength(&self) -> usize {
        self.ports.iter().sum()
    }
}
