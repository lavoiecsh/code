use crate::solver::AdventSolver;
use itertools::Itertools;
use regex::{Match, Regex};

pub struct Advent2018Day23Solver {
    nanobots: Vec<Nanobot>,
}

impl Advent2018Day23Solver {
    pub fn new(input: String) -> Self {
        let re = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
        let parse = |c: Option<Match>| c.unwrap().as_str().parse::<i64>().unwrap();
        Self {
            nanobots: input.lines()
                .filter_map(|l| re.captures(l))
                .map(|c| Nanobot {
                    pos: Pos(parse(c.get(1)), parse(c.get(2)), parse(c.get(3))),
                    radius: parse(c.get(4)),
                })
                .collect()
        }
    }
}

impl AdventSolver for Advent2018Day23Solver {
    fn solve_part1(&self) -> usize {
        let largest_range = self.nanobots.iter()
            .max_by(|a,b| a.radius.cmp(&b.radius).then_with(|| b.pos.distance_from_origin().cmp(&a.pos.distance_from_origin())))
            .unwrap();
        self.nanobots.iter()
            .filter(|n| n.is_in_range_of(largest_range))
            .count()
    }

    fn solve_part2(&self) -> usize {
        self.nanobots.iter()
            .flat_map(|n| {
                let d = n.pos.distance_from_origin();
                vec![(i64::max(d - n.radius, 0), 1), (d + n.radius + 1, -1)]
            })
            .sorted_by_key(|(d,_)| *d)
            .fold((0, 0, 0), |(b,c,m), (d,e)| {
                let c2 = c + e;
                if c2 > m {
                    (d, c2, c2)
                } else {
                    (b, c2, m)
                }
            })
            .0 as usize
    }
}

#[derive(Debug, Clone, Copy)]
struct Pos(i64, i64, i64);

impl Pos {
    fn distance_from_origin(&self) -> i64 {
        self.0 + self.1 + self.2
    }

    fn distance_to(&self, other: &Pos) -> i64 {
        (
            self.0.abs_diff(other.0) +
            self.1.abs_diff(other.1) +
            self.2.abs_diff(other.2)
        ) as i64
    }
}

#[derive(Debug)]
struct Nanobot {
    pos: Pos,
    radius: i64,
}

impl Nanobot {
    fn is_in_range_of(&self, other: &Nanobot) -> bool {
        other.pos.distance_to(&self.pos) <= other.radius
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_nanobots_in_range_of_largest_radius() {
        let solver = Advent2018Day23Solver::new(String::from("\
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
"));
        assert_eq!(solver.solve_part1(), 7);
    }

    #[test]
    fn finds_coordinates_closest_to_all_points() {
        let solver = Advent2018Day23Solver::new(String::from("\
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
"));
        assert_eq!(solver.solve_part2(), 36);
    }
}
