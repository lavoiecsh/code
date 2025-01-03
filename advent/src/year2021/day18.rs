use crate::solver::AdventSolver;

pub struct Advent2021Day18Solver {
    numbers: Vec<SnailfishNumber>,
}

impl Advent2021Day18Solver {
    pub fn new(input: &str) -> Self {
        Self {
            numbers: input
                .lines()
                .map(|l| LineParser::new(l.to_string()).build_snailfish())
                .collect(),
        }
    }

    fn sum_all(&self) -> SnailfishNumber {
        let mut sum = SnailfishNumber::from(&self.numbers[0]);
        for n in 1..self.numbers.len() {
            sum.add(&self.numbers[n]);
        }
        sum
    }
}

impl AdventSolver for Advent2021Day18Solver {
    fn solve_part1(&self) -> usize {
        self.sum_all().magnitude()
    }

    fn solve_part2(&self) -> usize {
        let mut best = 0;
        for a in 0..self.numbers.len() {
            for b in 0..self.numbers.len() {
                if a == b {
                    continue;
                }
                let mut number = SnailfishNumber::from(&self.numbers[a]);
                number.add(&self.numbers[b]);
                let mag = number.magnitude();
                if mag > best {
                    best = mag;
                }
            }
        }
        best
    }
}

type NodeId = usize;

struct Node {
    value: Option<u8>,
    left: Option<NodeId>,
    right: Option<NodeId>,
    parent: Option<NodeId>,
}

impl Node {
    fn copy(&self, offset: NodeId) -> Node {
        Node {
            value: self.value,
            left: self.left.map(|lid| lid + offset),
            right: self.right.map(|rid| rid + offset),
            parent: self.parent.map(|pid| pid + offset),
        }
    }
}

struct SnailfishNumber {
    nodes: Vec<Node>,
    root: NodeId,
}

impl SnailfishNumber {
    fn new() -> SnailfishNumber {
        SnailfishNumber {
            nodes: Vec::new(),
            root: usize::MAX,
        }
    }

    fn from(number: &SnailfishNumber) -> SnailfishNumber {
        SnailfishNumber {
            nodes: number.nodes.iter().map(|n| n.copy(0)).collect(),
            root: number.root,
        }
    }

    fn push_value(&mut self, value: u8) -> NodeId {
        let next = self.nodes.len();
        self.nodes.push(Node {
            value: Some(value),
            left: None,
            right: None,
            parent: None,
        });
        next
    }

    fn push_pair(&mut self, left: NodeId, right: NodeId) -> NodeId {
        let next = self.nodes.len();
        self.nodes.push(Node {
            value: None,
            left: Some(left),
            right: Some(right),
            parent: None,
        });
        self.nodes[left].parent = Some(next);
        self.nodes[right].parent = Some(next);
        next
    }

    fn value(&self, nid: NodeId) -> u8 {
        self.nodes[nid].value.unwrap()
    }

    fn is_value(&self, nid: NodeId) -> bool {
        self.nodes[nid].value.is_some()
    }

    fn left(&self, nid: NodeId) -> NodeId {
        self.nodes[nid].left.unwrap()
    }

    fn right(&self, nid: NodeId) -> NodeId {
        self.nodes[nid].right.unwrap()
    }

    fn parent(&self, nid: NodeId) -> NodeId {
        self.nodes[nid].parent.unwrap()
    }

    fn add(&mut self, other: &SnailfishNumber) {
        let offset = self.nodes.len();
        self.nodes
            .extend(other.nodes.iter().map(|n| n.copy(offset)));
        let new_root = Node {
            value: None,
            left: Some(self.root),
            right: Some(other.root + offset),
            parent: None,
        };
        let new_rid = self.nodes.len();
        self.nodes.push(new_root);
        self.nodes[self.root].parent = Some(new_rid);
        self.nodes[other.root + offset].parent = Some(new_rid);
        self.root = new_rid;
        self.reduce();
    }

    fn magnitude(&self) -> usize {
        self.magnitude_node(self.root)
    }

    fn magnitude_node(&self, nid: NodeId) -> usize {
        if self.is_value(nid) {
            self.value(nid) as usize
        } else {
            self.magnitude_node(self.left(nid)) * 3 + self.magnitude_node(self.right(nid)) * 2
        }
    }

    fn reduce(&mut self) {
        let mut modified = true;
        while modified {
            modified = false;
            if let Some(to_explode) = self.n_depth_pair(self.root, 4) {
                self.explode(to_explode);
                modified = true;
                continue;
            }

            if let Some(to_split) = self.large_number(self.root) {
                self.split(to_split);
                modified = true;
            }
        }
    }

    fn n_depth_pair(&self, nid: NodeId, depth: usize) -> Option<NodeId> {
        if self.is_value(nid) {
            return None;
        }

        if depth == 0 {
            return Some(nid);
        }

        self.n_depth_pair(self.left(nid), depth - 1)
            .or(self.n_depth_pair(self.right(nid), depth - 1))
    }

    fn explode(&mut self, nid: NodeId) {
        let lv = self.value(self.left(nid));
        let rv = self.value(self.right(nid));
        self.nodes[nid] = Node {
            value: Some(0),
            left: None,
            right: None,
            parent: self.nodes[nid].parent,
        };
        if let Some(lid) = self.immediate_left(nid) {
            self.nodes[lid].value = Some(self.value(lid) + lv);
        }
        if let Some(rid) = self.immediate_right(nid) {
            self.nodes[rid].value = Some(self.value(rid) + rv);
        }
    }

    fn immediate_left(&self, nid: NodeId) -> Option<NodeId> {
        let mut tid = nid;
        let mut pid = self.parent(tid);
        while pid != self.root && tid == self.left(pid) {
            tid = pid;
            pid = self.parent(pid);
        }
        if pid == self.root && tid == self.left(pid) {
            return None;
        }
        tid = self.left(pid);
        while !self.is_value(tid) {
            tid = self.right(tid);
        }
        Some(tid)
    }

    fn immediate_right(&self, nid: NodeId) -> Option<NodeId> {
        let mut tid = nid;
        let mut pid = self.parent(tid);
        while pid != self.root && tid == self.right(pid) {
            tid = pid;
            pid = self.parent(pid);
        }
        if pid == self.root && tid == self.right(pid) {
            return None;
        }
        tid = self.right(pid);
        while !self.is_value(tid) {
            tid = self.left(tid);
        }
        Some(tid)
    }

    fn large_number(&self, nid: NodeId) -> Option<NodeId> {
        if self.is_value(nid) {
            return if self.value(nid) >= 10 {
                Some(nid)
            } else {
                None
            };
        }

        self.large_number(self.left(nid))
            .or(self.large_number(self.right(nid)))
    }

    fn split(&mut self, nid: NodeId) {
        let value = self.value(nid);
        let lv = value / 2;
        let rv = value - lv;
        let lid = self.push_value(lv);
        self.nodes[lid].parent = Some(nid);
        let rid = self.push_value(rv);
        self.nodes[rid].parent = Some(nid);
        self.nodes[nid] = Node {
            value: None,
            left: Some(lid),
            right: Some(rid),
            parent: self.nodes[nid].parent,
        };
    }
}

struct LineParser {
    chars: Vec<char>,
    cursor: usize,
}

impl LineParser {
    fn new(line: String) -> LineParser {
        LineParser {
            chars: line.chars().collect(),
            cursor: 0,
        }
    }

    fn build_snailfish(&mut self) -> SnailfishNumber {
        let mut number = SnailfishNumber::new();
        number.root = self.sub(&mut number);
        number
    }

    fn sub(&mut self, number: &mut SnailfishNumber) -> NodeId {
        if self.chars[self.cursor] != '[' {
            let value = self.chars[self.cursor] as u8 - 48;
            self.cursor += 1;
            return number.push_value(value);
        }

        self.cursor += 1;
        let left = self.sub(number);
        assert_eq!(self.chars[self.cursor], ',');
        self.cursor += 1;
        let right = self.sub(number);
        assert_eq!(self.chars[self.cursor], ']');
        self.cursor += 1;
        number.push_pair(left, right)
    }
}
