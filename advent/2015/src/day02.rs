use std::fs;
use crate::problem_solver::ProblemSolver;

pub struct Problem02Solver {
    boxes: Vec<Box>
}

impl Problem02Solver {
    pub fn new() -> Self {
        Self { 
            boxes: fs::read_to_string("inputs/day02.txt")
                .expect("error reading")
                .trim()
                .lines()
                .map(Box::new)
                .collect()
        }
    }
}

impl ProblemSolver for Problem02Solver {
    fn solve_part1(&self) -> usize {
        self.boxes
            .iter()
            .map(Box::area_part1)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.boxes
            .iter()
            .map(Box::area_part2)
            .sum()
    }
}


struct Box {
    l: usize,
    w: usize,
    h: usize,
}

impl Box {
    fn new(s: &str) -> Box {
        let mut dimensions = s.split("x");
        Box {
            l: dimensions.next().unwrap().parse().expect("error parsing"),
            w: dimensions.next().unwrap().parse().expect("error parsing"),
            h: dimensions.next().unwrap().parse().expect("error parsing"),
        }
    }

    fn area_part1(&self) -> usize {
        let side_areas: [usize; 3] = [self.l*self.w, self.w*self.h, self.h*self.l];
        side_areas.iter().map(|x|x*2).sum::<usize>() + side_areas.iter().min().unwrap()
    }

    fn area_part2(&self) -> usize {
        let mut sides: [usize; 3] = [self.l, self.w, self.h];
        sides.sort();
        sides.iter().map(|x|x*2).take(2).sum::<usize>() + self.volume()
    }

    fn volume(&self) -> usize {
        self.l * self.w * self.h
    }
}
