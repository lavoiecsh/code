#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

const FILENAME: &str = "inputs/day15.txt";

type RisksMatrix = Vec<Vec<usize>>;

struct RisksMap {
    map: RisksMatrix,
    max_y: usize,
    max_x: usize,
}

#[allow(dead_code)]
fn read_input() -> RisksMap {
    let mut risks_map = RisksMap {
        map: fs::read_to_string(FILENAME).expect("error reading")
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c as usize - 48).collect::<Vec<usize>>())
            .collect(),
        max_y: 0,
        max_x: 0
    };
    risks_map.max_y = risks_map.map.len()-1;
    risks_map.max_x = risks_map.map[0].len()-1;
    risks_map
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let risks = read_input();
    let total_risks = compute_total_risks(&risks);
    total_risks.map[total_risks.max_y][total_risks.max_x]
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let top_left = read_input();
    let full = increase_map(&top_left);
    let total_risks = compute_total_risks(&full);
    total_risks.map[total_risks.max_y][total_risks.max_x]
}

fn compute_total_risks(risks: &RisksMap) -> RisksMap {
    let mut total_risks: RisksMap = RisksMap {
        map: vec![vec![usize::MAX; risks.max_x+1]; risks.max_y+1],
        max_y: risks.max_y,
        max_x: risks.max_x,
    };
    let mut modified: bool = true;
    total_risks.map[0][0] = 0;
    while modified {
        modified = false;
        for y in 0..=total_risks.max_y {
            for x in 0..=total_risks.max_x {
                let b = best_around(&total_risks, y, x);
                if b != usize::MAX && b + risks.map[y][x] < total_risks.map[y][x] {
                    total_risks.map[y][x] = b + risks.map[y][x];
                    modified = true;
                }
            }
        }
    }
    total_risks
}

fn best_around(total_risks: &RisksMap, y: usize, x: usize) -> usize {
    let up = if y > 0 { total_risks.map[y-1][x] } else { usize::MAX };
    let down = if y < total_risks.max_y { total_risks.map[y+1][x] } else { usize::MAX };
    let left = if x > 0 { total_risks.map[y][x-1] } else { usize::MAX };
    let right = if x < total_risks.max_x { total_risks.map[y][x+1] } else { usize::MAX };
    *(vec![up, down, left, right]).iter().min().unwrap()
}

fn increase_map(top_left: &RisksMap) -> RisksMap {
    let true_max_y = top_left.max_y+1;
    let true_max_x = top_left.max_x+1;
    let mut full = RisksMap {
        map: vec![vec![0; true_max_x*5]; true_max_y*5],
        max_y: true_max_y*5 - 1,
        max_x: true_max_x*5 - 1,
    };
    for ry in 0..5 {
        for rx in 0..5 {
            for y in 0..=top_left.max_y {
                for x in 0..=top_left.max_x {
                    let next = top_left.map[y][x] + ry + rx;
                    full.map[ry*true_max_y + y][rx*true_max_x + x] = if next < 10 { next } else { next - 9 };
                }
            }
        }
    }
    full
}
