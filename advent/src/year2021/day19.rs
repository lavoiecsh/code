use std::collections::{HashMap, HashSet};

use crate::solver::AdventSolver;

pub struct Advent2021Day19Solver {
    scanners: Vec<Scanner>,
}

impl Advent2021Day19Solver {
    pub fn new(input: &str) -> Self {
        let lines: Vec<String> = input.lines().map(String::from).collect();
        let mut scanners = Vec::new();
        let mut scanner_id = usize::MAX;
        for line in lines {
            if line.is_empty() {
                continue;
            }
            if line.starts_with("---") {
                scanner_id = if scanner_id == usize::MAX {
                    0
                } else {
                    scanner_id + 1
                };
                scanners.push(Scanner {
                    beacons: Vec::new(),
                });
                continue;
            }
            let mut coords = line.split(",");
            scanners[scanner_id].beacons.push(Position {
                x: coords.next().unwrap().parse().expect("error parsing"),
                y: coords.next().unwrap().parse().expect("error parsing"),
                z: coords.next().unwrap().parse().expect("error parsing"),
            });
        }
        Self { scanners }
    }

    fn compute(&self) -> (HashSet<Position>, Vec<Position>) {
        let mut absolute_beacons: HashSet<Position> = HashSet::new();
        let mut scanner_positions: Vec<Position> = Vec::new();
        let mut iter = self.scanners.iter();
        for b in &iter.next().unwrap().beacons {
            absolute_beacons.insert(*b);
        }
        scanner_positions.push(Position { x: 0, y: 0, z: 0 });
        let mut unfound_scanners: Vec<Scanner> = iter
            .map(|s| Scanner {
                beacons: s.beacons.to_vec(),
            })
            .collect();
        while !unfound_scanners.is_empty() {
            let mut matched = false;
            for i in 0..unfound_scanners.len() {
                let test = &unfound_scanners[i];
                let test_rots = test.all_rotations();
                for rot in &test_rots {
                    let mut position_map: HashMap<Position, usize> = HashMap::new();
                    for ab in &absolute_beacons {
                        for rb in &rot.beacons {
                            let p = rb.as_reference(ab);
                            position_map.insert(p, position_map.get(&p).unwrap_or(&0) + 1);
                        }
                    }
                    let (best_position, count) =
                        position_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
                    if *count >= 12 {
                        matched = true;
                        for b in &rot.beacons {
                            absolute_beacons.insert(b.as_absolute(best_position));
                        }
                        scanner_positions.push(*best_position);
                        break;
                    }
                }
                if matched {
                    unfound_scanners.remove(i);
                    break;
                }
            }
        }
        (absolute_beacons, scanner_positions)
    }
}

impl AdventSolver for Advent2021Day19Solver {
    fn solve_part1(&self) -> usize {
        self.compute().0.len()
    }

    fn solve_part2(&self) -> usize {
        let scanner_positions = self.compute().1;
        let mut largest = 0;
        for sp1 in &scanner_positions {
            for sp2 in &scanner_positions {
                let dist = sp1.distance_to(sp2);
                if dist > largest {
                    largest = dist;
                }
            }
        }
        largest as usize
    }
}

#[derive(Eq, Hash, Copy, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn as_absolute(&self, reference: &Self) -> Self {
        Position {
            x: self.x + reference.x,
            y: self.y + reference.y,
            z: self.z + reference.z,
        }
    }

    fn as_reference(&self, absolute: &Self) -> Self {
        Position {
            x: absolute.x - self.x,
            y: absolute.y - self.y,
            z: absolute.z - self.z,
        }
    }

    fn distance_to(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn roll(&self) -> Self {
        Position {
            x: self.x,
            y: -self.z,
            z: self.y,
        }
    }

    fn turn_cw(&self) -> Self {
        Position {
            x: self.z,
            y: self.y,
            z: -self.x,
        }
    }

    fn turn_ccw(&self) -> Self {
        Position {
            x: -self.z,
            y: self.y,
            z: self.x,
        }
    }
}

struct Scanner {
    beacons: Vec<Position>,
}

impl Scanner {
    fn all_rotations(&self) -> Vec<Scanner> {
        let mut rotated_scanners = Vec::new();
        for ri in 0..6 {
            rotated_scanners.push(rotated_scanners.last().unwrap_or(self).roll());
            let turn = if ri % 2 == 0 {
                Scanner::turn_cw
            } else {
                Scanner::turn_ccw
            };
            for _ in 0..3 {
                rotated_scanners.push(turn(rotated_scanners.last().unwrap()));
            }
        }
        rotated_scanners
    }

    fn roll(&self) -> Self {
        Scanner {
            beacons: self.beacons.iter().map(Position::roll).collect(),
        }
    }

    fn turn_cw(&self) -> Self {
        Scanner {
            beacons: self.beacons.iter().map(Position::turn_cw).collect(),
        }
    }

    fn turn_ccw(&self) -> Self {
        Scanner {
            beacons: self.beacons.iter().map(Position::turn_ccw).collect(),
        }
    }
}
