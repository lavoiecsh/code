use std::fs;
use regex::Regex;
use crate::problem_solver::ProblemSolver;

struct Reindeer {
    name: String,
    speed: usize,
    time: usize,
    rest: usize,
}

pub struct Problem14Solver {
    reindeer: Vec<Reindeer>
}

impl Problem14Solver {
    pub fn new() -> Self {
        let re = Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();
        Self {
            reindeer: fs::read_to_string("inputs/day14.txt")
                .expect("error reading")
                .trim()
                .lines()
                .map(|l| {
                    let m = re.captures(l).unwrap();
                    Reindeer {
                        name: m.get(1).unwrap().as_str().to_string(),
                        speed: m.get(2).unwrap().as_str().parse().unwrap(),
                        time: m.get(3).unwrap().as_str().parse().unwrap(),
                        rest: m.get(4).unwrap().as_str().parse().unwrap(),
                    }
                })
                .collect()
        }
    }
}

const TIME_LIMIT: usize = 2503;
impl ProblemSolver for Problem14Solver {
    fn solve_part1(&self) -> usize {
        self.reindeer
            .iter()
            .map(|r| {
                let cycle_time = r.time + r.rest;
                let complete_cycles = TIME_LIMIT / cycle_time;
                let complete_cycles_distance = r.speed * r.time * complete_cycles;
                let remaining_time = TIME_LIMIT - complete_cycles * cycle_time;
                let remaining_distance = r.speed * usize::min(remaining_time, r.time);
                complete_cycles_distance + remaining_distance
            })
            .max()
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        let  mut positions: Vec<ReindeerPosition> = self.reindeer.iter().map(|r| ReindeerPosition {
            time: r.time,
            resting: false,
            position: 0,
            points: 0,
        }).collect();
        let reindeer_count = self.reindeer.len();
        for i in 0..TIME_LIMIT {
            for j in 0..reindeer_count {
                positions[j].time -= 1;
                if !positions[j].resting {
                    positions[j].position += self.reindeer[j].speed;
                }
                if positions[j].time == 0 {
                    positions[j].resting = !positions[j].resting;
                    positions[j].time = if positions[j].resting { self.reindeer[j].rest } else { self.reindeer[j].time };
                }
            }
            let leading_position = positions.iter().map(|p| p.position).max().unwrap();
            for j in 0..reindeer_count {
                if positions[j].position == leading_position {
                    positions[j].points += 1;
                }
            }
        }
        positions.iter().map(|r| r.points).max().unwrap()
    }
}

struct ReindeerPosition {
    time: usize,
    resting: bool,
    position: usize,
    points: usize,
}
