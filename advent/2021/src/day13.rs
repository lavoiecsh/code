#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

use std::collections::HashSet;

const FILENAME: &str = "inputs/day13.txt";

type Point = (usize, usize);
type Fold = (bool, usize);

#[allow(dead_code)]
fn read_input() -> (HashSet<Point>, Vec<Fold>) {
    let mut points = HashSet::new();
    let mut folds = Vec::new();
    let mut reading_points: bool = true;
    for line in fs::read_to_string(FILENAME).expect("error reading").trim().lines() {
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
    (points, folds)
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let (points, folds) = read_input();
    fold(folds.first().unwrap(), &points).len()
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let (mut points, folds) = read_input();
    for f in folds {
        points = fold(&f, &points);
    }
    print!("{}", draw(&points));
    0
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
