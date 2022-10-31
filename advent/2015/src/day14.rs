use std::fs;
use regex::Regex;

const FILENAME: &str = "inputs/day14.txt";

struct Reindeer {
    name: String,
    speed: usize,
    time: usize,
    rest: usize,
}

fn read_input() -> Vec<Reindeer> {
    let re = Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();
    fs::read_to_string(FILENAME)
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

pub fn part1() -> usize {
    const TIME_LIMIT: usize = 2503;
    read_input()
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

struct ReindeerPosition {
    time: usize,
    resting: bool,
    position: usize,
    points: usize,
}

pub fn part2() -> usize {
    const TIME_LIMIT: usize = 2503;
    let reindeer = read_input();
    let  mut positions: Vec<ReindeerPosition> = reindeer.iter().map(|r| ReindeerPosition {
        time: r.time,
        resting: false,
        position: 0,
        points: 0,
    }).collect();
    let reindeer_count = reindeer.len();
    for i in 0..TIME_LIMIT {
        for j in 0..reindeer_count {
            positions[j].time -= 1;
            if !positions[j].resting {
                positions[j].position += reindeer[j].speed;
            }
            if positions[j].time == 0 {
                positions[j].resting = !positions[j].resting;
                positions[j].time = if positions[j].resting { reindeer[j].rest } else { reindeer[j].time };
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
