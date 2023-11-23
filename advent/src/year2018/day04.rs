use std::collections::HashMap;
use std::ops::Range;

use itertools::Itertools;
use regex::Regex;

use crate::solver::AdventSolver;

pub struct Advent2018Day04Solver {
    guards: HashMap<usize, Guard>
}

impl Advent2018Day04Solver {
    pub fn new(input: String) -> Self {
        let shift_re = Regex::new(r".*#(\d+) .*").unwrap();
        let sleep_re = Regex::new(r".*:(\d+)] falls asleep").unwrap();
        let wakes_re = Regex::new(r".*:(\d+)] wakes up").unwrap();
        let logs: Vec<&str> = input.lines().sorted().collect();
        let mut guards = HashMap::new();
        let mut current_guard = 0;
        let mut sleep_start = 0;
        for log in logs {
            if let Some(cap) = shift_re.captures(log) {
                current_guard = cap.get(1).unwrap().as_str().parse().unwrap();
                guards.entry(current_guard).or_insert_with(|| Guard { shifts: Vec::new() })
                    .shifts.push(Shift { sleeps: Vec::new() });
            } else if let Some(cap) = sleep_re.captures(log) {
                sleep_start = cap.get(1).unwrap().as_str().parse().unwrap();
            } else if let Some(cap) = wakes_re.captures(log) {
                let sleep_end = cap.get(1).unwrap().as_str().parse().unwrap();
                guards.get_mut(&current_guard).unwrap()
                    .shifts.iter_mut().last().unwrap()
                    .sleeps.push(sleep_start..sleep_end);
            }
        }
        Self { guards }
    }
}

impl AdventSolver for Advent2018Day04Solver {
    fn solve_part1(&self) -> usize {
        let (id, guard) = self.guards.iter()
            .max_by(|(_,l),(_,r)| l.minutes_asleep().cmp(&r.minutes_asleep())).unwrap();
        id * guard.most_asleep_minute().0
    }

    fn solve_part2(&self) -> usize {
        let (id, (minute, _)) = self.guards.iter()
            .map(|(i,g)| (i,g.most_asleep_minute()))
            .max_by(|(_,(_,l)),(_,(_,r))| l.cmp(&r))
            .unwrap();
        id * minute
    }
}

struct Guard {
    shifts: Vec<Shift>,
}

impl Guard {
    fn minutes_asleep(&self) -> usize {
        self.shifts.iter().map(Shift::minutes_asleep).sum()
    }

    fn most_asleep_minute(&self) -> (usize, usize) {
        let mut minutes: Vec<usize> = Vec::new();
        minutes.resize(60, 0);
        self.shifts.iter()
            .for_each(|shift| shift.sleeps.iter()
                .for_each(|sleep| sleep.clone().for_each(|minute| minutes[minute] += 1)));
        minutes.into_iter().enumerate().max_by(|(_,l),(_,r)| l.cmp(&r)).unwrap()
    }
}

struct Shift {
    sleeps: Vec<Range<usize>>,
}

impl Shift {
    fn minutes_asleep(&self) -> usize {
        self.sleeps.iter().map(|r| r.end - r.start).sum()
    }
}
