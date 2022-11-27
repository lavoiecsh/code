use std::collections::HashSet;
use std::fs::read_to_string;
use crate::solver::AdventSolver;

type Point = (usize, usize);
type Fold = (bool, usize);

pub struct Advent2021Day13Solver {
    points: HashSet<Point>,
    folds: Vec<Fold>,
}

impl AdventSolver for Advent2021Day13Solver {
    fn day(&self) -> usize { 13 }
    fn year(&self) -> usize { 2021 }

    fn solve_part1(&self) -> usize {
        fold(self.folds.first().unwrap(), &self.points).len()
    }

    fn solve_part2_string(&self) -> String {
        let mut points = self.points.clone();
        for f in &self.folds {
            points = fold(&f, &points);
        }
        draw(&points)
    }
}

fn fold(fold: &Fold, points: &HashSet<Point>) -> HashSet<Point> {
    let mut new_points: HashSet<Point> = HashSet::new();
    for p in points {
        if fold.0 {
            assert!(p.1 != fold.1);
            if p.1 < fold.1 {
                new_points.insert(*p);
            } else {
                new_points.insert((p.0, fold.1 - (p.1 - fold.1)));
            }
        } else {
            assert!(p.0 != fold.1);
            if p.0 < fold.1 {
                new_points.insert(*p);
            } else {
                new_points.insert((fold.1 - (p.0 - fold.1), p.1));
            }
        }
    }
    new_points
}

fn draw(points: &HashSet<Point>) -> String {
    let mut max_x = 0;
    let mut max_y = 0;
    for p in points {
        if p.0 > max_x {
            max_x = p.0;
        }
        if p.1 > max_y {
            max_y = p.1;
        }
    }
    let mut paper = String::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&(x, y)) {
                paper += "█";
            } else {
                paper += " ";
            }
        }
        paper += "\n";
    }
    paper
}

pub fn advent2021_day13_solver() -> Box<dyn AdventSolver> {
    let mut points = HashSet::new();
    let mut folds = Vec::new();
    let mut reading_points: bool = true;
    for line in read_to_string("src/year2021/day13.txt").unwrap().trim().lines() {
        if line == "" {
            reading_points = false;
            continue;
        }
        if reading_points {
            let mut split = line.split(",");
            points.insert((split.next().unwrap().parse().expect("error parsing"), split.next().unwrap().parse().expect("error parsing")));
        } else {
            let mut split = line.split(" ");
            split.next();
            split.next();
            split = split.next().unwrap().split("=");
            folds.push((split.next().unwrap() == "y", split.next().unwrap().parse().expect("error parsing")));
        }
    }
    Box::new(Advent2021Day13Solver {
        points,
        folds,
    })
}
