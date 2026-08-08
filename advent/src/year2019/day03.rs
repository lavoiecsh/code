use crate::solver::AdventSolver;

pub struct Advent2019Day03Solver {
    wire_1_path: Path,
    wire_2_path: Path,
}

impl Advent2019Day03Solver {
    pub fn new(input: &str) -> Self {
        let mut lines = input.lines();
        Self {
            wire_1_path: Path::from(lines.next().unwrap()),
            wire_2_path: Path::from(lines.next().unwrap()),
        }
    }
}

impl AdventSolver for Advent2019Day03Solver {
    fn solve_part1(&self) -> usize {
        let segments_1 = self.wire_1_path.segments();
        let segments_2 = self.wire_2_path.segments();
        let mut closest_intersection = usize::MAX;
        for segment_1 in &segments_1 {
            for segment_2 in &segments_2 {
                if let Some((intersection, _, _)) = segment_1.intersect_with(segment_2) {
                    if intersection == (0, 0) {
                        continue;
                    }
                    let distance = intersection.0.abs() as usize + intersection.1.abs() as usize;
                    if distance < closest_intersection {
                        closest_intersection = distance;
                    }
                }
            }
        }
        closest_intersection
    }

    fn solve_part2(&self) -> usize {
        let segments_1 = self.wire_1_path.segments();
        let segments_2 = self.wire_2_path.segments();
        let mut lowest_steps = usize::MAX;
        let mut steps_1 = 0;
        for segment_1 in &segments_1 {
            let mut steps_2 = 0;
            for segment_2 in &segments_2 {
                if let Some((intersection, d1, d2)) = segment_1.intersect_with(segment_2) {
                    if intersection == (0, 0) {
                        continue;
                    }
                    let total_steps = steps_1 + d1 as usize + steps_2 + d2 as usize;
                    if total_steps < lowest_steps {
                        lowest_steps = total_steps;
                    }
                }
                steps_2 += segment_2.distance();
            }
            steps_1 += segment_1.distance();
        }
        lowest_steps
    }
}

type Pos = (i32, i32);
#[derive(Debug)]
enum PathSegment {
    Down { x: i32, fy: i32, ty: i32, d: i32 },
    Up { x: i32, fy: i32, ty: i32, d: i32 },
    Left { y: i32, fx: i32, tx: i32, d: i32 },
    Right { y: i32, fx: i32, tx: i32, d: i32 },
}

impl PathSegment {
    fn distance(&self) -> usize {
        match self {
            PathSegment::Down { d, .. } |
            PathSegment::Up { d, .. } |
            PathSegment::Left { d, .. } |
            PathSegment::Right { d, .. } => *d as usize,
        }
    }

    fn intersect_with(&self, other: &Self) -> Option<(Pos, i32, i32)> {
        match (self, other) {
            (PathSegment::Down { x, fy, ty, .. }, PathSegment::Right { y, fx, tx, .. }) => {
                if x >= fx && x <= tx && y >= ty && y <= fy {
                    Some(((*x, *y), (fy - y).abs(), (x - fx).abs()))
                } else {
                    None
                }
            }
            (PathSegment::Right { y, fx, tx, .. }, PathSegment::Down { x, fy, ty, .. }) => {
                if x >= fx && x <= tx && y >= ty && y <= fy {
                    Some(((*x, *y), (x - fx).abs(), (fy - y).abs()))
                } else {
                    None
                }
            }
            (PathSegment::Down { x, fy, ty, .. }, PathSegment::Left { y, fx, tx, .. }) => {
                if x >= tx && x <= fx && y >= ty && y <= fy {
                    Some(((*x, *y), (fy - y).abs(), (fx - x).abs()))
                } else {
                    None
                }
            }
            (PathSegment::Left { y, fx, tx, .. }, PathSegment::Down { x, fy, ty, .. }) => {
                if x >= tx && x <= fx && y >= ty && y <= fy {
                    Some(((*x, *y), (fx - x).abs(), (fy - y).abs()))
                } else {
                    None
                }
            }
            (PathSegment::Up { x, fy, ty, .. }, PathSegment::Right { y, fx, tx, .. }) => {
                if x >= fx && x <= tx && y >= fy && y <= ty {
                    Some(((*x, *y), (y - fy).abs(), (x - fx).abs()))
                } else {
                    None
                }
            }
            (PathSegment::Right { y, fx, tx, .. }, PathSegment::Up { x, fy, ty, .. }) => {
                if x >= fx && x <= tx && y >= fy && y <= ty {
                    Some(((*x, *y), (x - fx).abs(), (y - fy).abs()))
                } else {
                    None
                }
            }
            (PathSegment::Up { x, fy, ty, .. }, PathSegment::Left { y, fx, tx, .. }) => {
                if x >= tx && x <= fx && y >= fy && y <= ty {
                    Some(((*x, *y), (y - fy).abs(), (fx - x).abs()))
                } else {
                    None
                }
            }
            (PathSegment::Left { y, fx, tx, .. }, PathSegment::Up { x, fy, ty, .. }) => {
                if x >= tx && x <= fx && y >= fy && y <= ty {
                    Some(((*x, *y), (fx - x).abs(), (y - fy).abs()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

struct Path {
    movements: Vec<PathMovement>,
}

impl Path {
    fn segments(&self) -> Vec<PathSegment> {
        let mut segments = Vec::new();
        let mut from = (0, 0);
        for movement in &self.movements {
            let (to, segment) = movement.segment_starting_at(from);
            segments.push(segment);
            from = to;
        }
        segments
    }
}

struct PathMovement {
    direction: PathDirection,
    distance: i32,
}

impl PathMovement {
    fn segment_starting_at(&self, pos: Pos) -> (Pos, PathSegment) {
        match self.direction {
            PathDirection::Down => {
                let to = pos.1 - self.distance;
                (
                    (pos.0, to),
                    PathSegment::Down {
                        x: pos.0,
                        fy: pos.1,
                        ty: to,
                        d: self.distance,
                    },
                )
            }
            PathDirection::Up => {
                let to = pos.1 + self.distance;
                (
                    (pos.0, to),
                    PathSegment::Up {
                        x: pos.0,
                        fy: pos.1,
                        ty: to,
                        d: self.distance,
                    },
                )
            }
            PathDirection::Left => {
                let to = pos.0 - self.distance;
                (
                    (to, pos.1),
                    PathSegment::Left {
                        y: pos.1,
                        fx: pos.0,
                        tx: to,
                        d: self.distance,
                    },
                )
            }
            PathDirection::Right => {
                let to = pos.0 + self.distance;
                (
                    (to, pos.1),
                    PathSegment::Right {
                        y: pos.1,
                        fx: pos.0,
                        tx: to,
                        d: self.distance,
                    },
                )
            }
        }
    }
}

enum PathDirection {
    Right,
    Down,
    Left,
    Up,
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Self {
            movements: value.split(',').map(PathMovement::from).collect(),
        }
    }
}

impl From<&str> for PathMovement {
    fn from(value: &str) -> Self {
        let split = value.split_at(1);
        Self {
            direction: PathDirection::from(split.0.chars().next().unwrap()),
            distance: split.1.parse().unwrap(),
        }
    }
}

impl From<char> for PathDirection {
    fn from(value: char) -> Self {
        match value {
            'R' => PathDirection::Right,
            'D' => PathDirection::Down,
            'L' => PathDirection::Left,
            'U' => PathDirection::Up,
            _v => unreachable!("unknown path direction {_v}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "\
R8,U5,L5,D3
U7,R6,D4,L4
";
    const EXAMPLE_2: &str = "\
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
";
    const EXAMPLE_3: &str = "\
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
";

    #[test]
    fn finds_closest_intersections() {
        let solver_1 = Advent2019Day03Solver::new(EXAMPLE_1);
        assert_eq!(solver_1.solve_part1(), 6);
        let solver_2 = Advent2019Day03Solver::new(EXAMPLE_2);
        assert_eq!(solver_2.solve_part1(), 159);
        let solver_3 = Advent2019Day03Solver::new(EXAMPLE_3);
        assert_eq!(solver_3.solve_part1(), 135);
    }

    #[test]
    fn finds_lowest_steps_intersections() {
        let solver_1 = Advent2019Day03Solver::new(EXAMPLE_1);
        assert_eq!(solver_1.solve_part2(), 30);
        let solver_2 = Advent2019Day03Solver::new(EXAMPLE_2);
        assert_eq!(solver_2.solve_part2(), 610);
        let solver_3 = Advent2019Day03Solver::new(EXAMPLE_3);
        assert_eq!(solver_3.solve_part2(), 410);
    }
}
