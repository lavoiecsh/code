use regex::Regex;

use crate::solver::AdventSolver;

const TIME_LIMIT: usize = 2503;
struct Reindeer {
    speed: usize,
    time: usize,
    rest: usize,
}

pub struct Advent2015Day14Solver {
    reindeer: Vec<Reindeer>,
}

impl Advent2015Day14Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(
            r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
        )
        .unwrap();
        Self {
            reindeer: input
                .lines()
                .map(|l| {
                    let m = re.captures(l).unwrap();
                    Reindeer {
                        speed: m.get(2).unwrap().as_str().parse().unwrap(),
                        time: m.get(3).unwrap().as_str().parse().unwrap(),
                        rest: m.get(4).unwrap().as_str().parse().unwrap(),
                    }
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2015Day14Solver {
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
        let mut positions: Vec<ReindeerPosition> = self
            .reindeer
            .iter()
            .map(|r| ReindeerPosition {
                time: r.time,
                resting: false,
                position: 0,
                points: 0,
            })
            .collect();
        for _ in 0..TIME_LIMIT {
            for (j, position) in positions.iter_mut().enumerate() {
                position.time -= 1;
                if !position.resting {
                    position.position += self.reindeer[j].speed;
                }
                if position.time == 0 {
                    position.resting = !position.resting;
                    position.time = if position.resting {
                        self.reindeer[j].rest
                    } else {
                        self.reindeer[j].time
                    };
                }
            }
            let leading_position = positions.iter().map(|p| p.position).max().unwrap();
            for position in positions.iter_mut() {
                if position.position == leading_position {
                    position.points += 1;
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
