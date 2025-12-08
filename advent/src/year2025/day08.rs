use crate::solver::AdventSolver;
use itertools::Itertools;
use std::cmp::Ordering;

pub struct Advent2025Day08Solver {
    playground: Playground,
}

impl Advent2025Day08Solver {
    pub fn new(input: &str) -> Self {
        Self {
            playground: Playground::new(input
                    .lines()
                    .map(|l| {
                        let s = l.split(',').map(|v| v.parse().unwrap()).collect_vec();
                        JunctionBox::new(s[0], s[1], s[2])
                    })
                    .collect()),
        }
    }
}

impl AdventSolver for Advent2025Day08Solver {
    fn solve_part1(&self) -> usize {
        let mut playground = self.playground.clone();
        playground.connect_n(1000);
        playground.three_largest_circuits()
    }

    fn solve_part2(&self) -> usize {
        let mut playground = self.playground.clone();
        playground.connect_all();
        playground.last_connection() as usize
    }
}

#[derive(Clone)]
struct Playground {
    junction_boxes: Vec<JunctionBox>,
    circuits: Vec<Vec<usize>>,
    distances: Vec<(usize, usize, i64)>,
    connections: usize,
}

impl Playground {
    fn new(junction_boxes: Vec<JunctionBox>) -> Self {
        let mut distances = Vec::new();
        for i in 0..junction_boxes.len() {
            for j in i + 1..junction_boxes.len() {
                distances.push((
                    i,
                    j,
                    junction_boxes[i].distance_to(&junction_boxes[j]),
                ));
            }
        }
        distances.sort_by_key(|(_, _, d)| *d);
        Self {
            circuits: (0..junction_boxes.len()).map(|i| vec![i]).collect(),
            junction_boxes,
            distances,
            connections: 0,
        }
    }

    fn three_largest_circuits(&self) -> usize {
        self.circuits.iter().take(3).map(Vec::len).product()
    }

    fn last_connection(&self) -> i64 {
        let (a, b, _) = self.distances[self.connections - 1];
        self.junction_boxes[a].x * self.junction_boxes[b].x
    }

    fn connect_n(&mut self, count: usize) {
        while self.connections < count {
            let (a, b, _) = self.distances[self.connections];
            self.connect(a, b);
            self.connections += 1;
        }
    }

    fn connect_all(&mut self) {
        while !self.is_fully_connected() {
            let (a, b, _) = self.distances[self.connections];
            self.connect(a, b);
            self.connections += 1;
        }
    }

    fn is_fully_connected(&self) -> bool {
        self.circuits.len() == 1
    }

    fn connect(&mut self, a: usize, b: usize) {
        let (ai, _) = self.circuits.iter().find_position(|c| c.contains(&a)).unwrap();
        let (bi, _) = self.circuits.iter().find_position(|c| c.contains(&b)).unwrap();
        match ai.cmp(&bi) {
            Ordering::Equal => return,
            Ordering::Less => {
                let bc = self.circuits.swap_remove(bi);
                self.circuits[ai].extend(bc);
            }
            Ordering::Greater => {
                let ac = self.circuits.swap_remove(ai);
                self.circuits[bi].extend(ac);
            }
        }
        self.circuits.sort_by_key(|c| -(c.len() as i64));
    }
}

#[derive(Clone, Debug)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn distance_to(&self, other: &Self) -> i64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        x * x + y * y + z * z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn finds_three_largest_circuits() {
        let solver = Advent2025Day08Solver::new(EXAMPLE);
        let mut playground = solver.playground.clone();
        playground.connect_n(10);
        assert_eq!(playground.circuits.iter().map(Vec::len).collect_vec(), vec![5, 4, 2, 2, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(playground.three_largest_circuits(), 40);
    }

    #[test]
    fn finds_last_connection() {
        let solver = Advent2025Day08Solver::new(EXAMPLE);
        let mut playground = solver.playground.clone();
        playground.connect_all();
        assert_eq!(playground.last_connection(), 25272);
    }
}
