use crate::solver::AdventSolver;

pub struct Advent2015Day02Solver {
    presents: Vec<Present>,
}

impl Advent2015Day02Solver {
    pub fn new(input: &str) -> Self {
        Self {
            presents: input.trim().lines().map(Present::new).collect(),
        }
    }
}

struct Present {
    l: usize,
    w: usize,
    h: usize,
}

impl Present {
    fn new(s: &str) -> Self {
        let mut dimensions = s.split("x");
        Self {
            l: dimensions.next().unwrap().parse().unwrap(),
            w: dimensions.next().unwrap().parse().unwrap(),
            h: dimensions.next().unwrap().parse().unwrap(),
        }
    }

    fn area_part1(&self) -> usize {
        let side_areas: [usize; 3] = [self.l * self.w, self.w * self.h, self.h * self.l];
        side_areas.iter().map(|x| x * 2).sum::<usize>() + side_areas.iter().min().unwrap()
    }

    fn area_part2(&self) -> usize {
        let mut sides: [usize; 3] = [self.l, self.w, self.h];
        sides.sort();
        sides.iter().map(|x| x * 2).take(2).sum::<usize>() + self.l * self.w * self.h
    }
}

impl AdventSolver for Advent2015Day02Solver {
    fn solve_part1(&self) -> usize {
        self.presents.iter().map(Present::area_part1).sum()
    }

    fn solve_part2(&self) -> usize {
        self.presents.iter().map(Present::area_part2).sum()
    }
}
