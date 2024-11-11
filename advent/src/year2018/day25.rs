use itertools::Itertools;
use crate::solver::AdventSolver;

pub struct Advent2018Day25Solver {
    coordinates: Vec<Pos>,
}

impl Advent2018Day25Solver {
    pub fn new(input: String) -> Self {
        Self {
            coordinates: input.lines()
                .map(Pos::from)
                .collect(),
        }
    }
}

impl AdventSolver for Advent2018Day25Solver {
    fn solve_part1(&self) -> usize {
        let mut constellations = Constellations::new();
        self.coordinates.iter().for_each(|&c| constellations.add_point(c));
        constellations.count()
    }
}

struct Constellations {
    constellations: Vec<Constellation>,
}

impl Constellations {
    fn new() -> Self {
        Self { constellations: Vec::new() }
    }
    
    fn add_point(&mut self, point: Pos) {
        match self.constellations.iter()
            .enumerate()
            .filter(|(_,c)| c.can_contain(&point))
            .map(|(i,_)| i)
            .sorted()
            .as_slice() {
            [] => { self.constellations.push(Constellation::new(point)); }
            [ci] => { self.constellations[*ci].add_point(point); }
            [ci, cis @ ..] => {
                for i in cis.iter().rev() {
                    let c = self.constellations.remove(*i);
                    self.constellations[*ci].join_with(c);
                }
                self.constellations[*ci].add_point(point);
            }
        }
    }
    
    fn count(&self) -> usize {
        self.constellations.len()
    }
}

struct Constellation {
    points: Vec<Pos>,
}

impl Constellation {
    fn new(point: Pos) -> Self {
        Self { points: vec![point] }
    }
    
    fn add_point(&mut self, point: Pos) {
        self.points.push(point);
    }
    
    fn can_contain(&self, point: &Pos) -> bool {
        self.points.iter()
            .any(|p| p.close_enough(point))
    }
    
    fn join_with(&mut self, other: Constellation) {
        self.points.extend(other.points)
    }
}

#[derive(Copy, Clone)]
struct Pos(i32, i32, i32, i32);

impl From<&str> for Pos {
    fn from(value: &str) -> Self {
        let values = value.split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect_vec();
        Self(values[0], values[1], values[2], values[3])
    }
}

impl Pos {
    fn close_enough(&self, other: &Self) -> bool {
        self.0.abs_diff(other.0) +
            self.1.abs_diff(other.1) +
            self.2.abs_diff(other.2) +
            self.3.abs_diff(other.3) <= 3
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_constellations() {
        assert_eq!(Advent2018Day25Solver::new(String::from(EXAMPLE_1)).solve_part1(), 2);
        assert_eq!(Advent2018Day25Solver::new(String::from(EXAMPLE_2)).solve_part1(), 4);
        assert_eq!(Advent2018Day25Solver::new(String::from(EXAMPLE_3)).solve_part1(), 3);
        assert_eq!(Advent2018Day25Solver::new(String::from(EXAMPLE_4)).solve_part1(), 8);
    }

    static EXAMPLE_1: &str = "\
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0
";
    static EXAMPLE_2: &str = "\
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
";
    static EXAMPLE_3: &str = "\
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2
";
    static EXAMPLE_4: &str = "\
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2
";
}
