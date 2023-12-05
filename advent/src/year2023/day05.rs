use std::cmp::Ordering;
use std::collections::VecDeque;
use itertools::Itertools;
use crate::solver::AdventSolver;

pub struct Advent2023Day05Solver {
    seeds: Vec<u32>,
    almanac: Almanac,
}

impl Advent2023Day05Solver {
    pub fn new(input: String) -> Self {
        let split = input.split_once("\n").unwrap();
        Self {
            seeds: split.0.split(" ").skip(1).map(|n| n.parse().unwrap()).collect(),
            almanac: Almanac::from(split.1),
        }
    }
}

impl AdventSolver for Advent2023Day05Solver {
    fn solve_part1(&self) -> usize {
        self.seeds.iter()
            .map(|seed| self.almanac.seed_location(*seed))
            .min()
            .unwrap()
            as usize
    }

    fn solve_part2(&self) -> usize {
        let mut seed_ranges = Vec::new();
        for i in 0..self.seeds.len() / 2 {
            seed_ranges.push(self.seeds[i * 2]..self.seeds[i * 2] + self.seeds[i * 2 + 1]);
        }
        self.almanac.seed_range_location(&seed_ranges) as usize
    }
}

struct Almanac {
    seed_location: Transform,
}

impl Almanac {
    fn new(transforms: Vec<Transform>) -> Self {
        Self { seed_location: transforms.iter().fold(Transform::init(), |acc, cur| acc.merge(cur)) }
    }

    fn seed_location(&self, seed: u32) -> u32 {
        self.seed_location.transform(seed)
    }

    fn seed_range_location(&self, seeds: &Vec<std::ops::Range<u32>>) -> u32 {
        self.seed_location.lowest_matching(seeds).destination
    }
}

struct Transform {
    ranges: Vec<Range>,
}

impl Transform {
    fn init() -> Self {
        Self { ranges: vec!(Range { source: 0, destination: 0, length: u32::MAX }) }
    }

    fn new(ranges: Vec<Range>) -> Self {
        let mut ranges = ranges;
        ranges.sort_by_key(|r| r.source);
        if ranges[0].source != 0 {
            ranges.insert(0, Range { source: 0, destination: 0, length: ranges[0].source });
        }
        let mut i = 1;
        while i < ranges.len() {
            let last_source = ranges[i - 1].source + ranges[i - 1].length;
            if ranges[i].source != last_source {
                ranges.insert(i, Range {
                    source: last_source,
                    destination: last_source,
                    length: ranges[i].source - last_source,
                });
            }
            i += 1;
        }
        let last_source = ranges[i - 1].source - 1 + ranges[i - 1].length;
        if last_source != u32::MAX {
            ranges.push(Range { source: last_source, destination: last_source, length: u32::MAX - last_source })
        }
        Self { ranges }
    }

    fn transform(&self, input: u32) -> u32 {
        self.ranges.iter().filter_map(|r| r.to_destination(input)).next().unwrap()
    }

    fn lowest_matching(&self, ranges: &Vec<std::ops::Range<u32>>) -> &Range {
        self.ranges.iter()
            .sorted_by_key(|r| r.destination)
            .skip_while(|r| !ranges.iter().any(|s| s.contains(&r.source)))
            .next()
            .unwrap()
    }

    fn merge(&self, next: &Self) -> Self {
        let mut self_ranges: VecDeque<Range> = self.ranges.iter().cloned().sorted_by_key(|r| r.destination).collect();
        let mut next_ranges: VecDeque<Range> = next.ranges.iter().cloned().sorted_by_key(|r| r.source).collect();
        let mut ranges = Vec::new();
        while !self_ranges.is_empty() && !next_ranges.is_empty() {
            let sr = self_ranges.pop_front().unwrap();
            let nr = next_ranges.pop_front().unwrap();
            match sr.length.cmp(&nr.length) {
                Ordering::Equal => {
                    ranges.push(Range { source: sr.source, destination: nr.destination, length: sr.length });
                }
                Ordering::Less => {
                    ranges.push(Range { source: sr.source, destination: nr.destination, length: sr.length });
                    next_ranges.push_front(Range {
                        source: nr.source + sr.length,
                        destination: nr.destination + sr.length,
                        length: nr.length - sr.length,
                    });
                }
                Ordering::Greater => {
                    ranges.push(Range { source: sr.source, destination: nr.destination, length: nr.length });
                    self_ranges.push_front(Range {
                        source: sr.source + nr.length,
                        destination: sr.destination + nr.length,
                        length: sr.length - nr.length,
                    });
                }
            }
        }
        ranges.sort_by_key(|r| r.source);
        Self { ranges }
    }
}

#[derive(Debug, Clone)]
struct Range {
    destination: u32,
    source: u32,
    length: u32,
}

impl Range {
    fn to_destination(&self, input: u32) -> Option<u32> {
        if input < self.source { return None; }
        let offset = input - self.source;
        if offset < self.length { Some(self.destination + offset) } else { None }
    }
}

impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut seed_soil = Vec::new();
        let mut soil_fertilizer = Vec::new();
        let mut fertilizer_water = Vec::new();
        let mut water_light = Vec::new();
        let mut light_temperature = Vec::new();
        let mut temperature_humidity = Vec::new();
        let mut humidity_location = Vec::new();
        let mut state: &str = "";
        for line in value.lines().filter(|l| !l.is_empty()) {
            match (state, line.split(" ").next().unwrap()) {
                ("", "seed-to-soil") => state = "ss",
                ("ss", "soil-to-fertilizer") => state = "sf",
                ("ss", _) => seed_soil.push(Range::from(line)),
                ("sf", "fertilizer-to-water") => state = "fw",
                ("sf", _) => soil_fertilizer.push(Range::from(line)),
                ("fw", "water-to-light") => state = "wl",
                ("fw", _) => fertilizer_water.push(Range::from(line)),
                ("wl", "light-to-temperature") => state = "lt",
                ("wl", _) => water_light.push(Range::from(line)),
                ("lt", "temperature-to-humidity") => state = "th",
                ("lt", _) => light_temperature.push(Range::from(line)),
                ("th", "humidity-to-location") => state = "hl",
                ("th", _) => temperature_humidity.push(Range::from(line)),
                ("hl", _) => humidity_location.push(Range::from(line)),
                (s, _) => panic!("unknown map state {s}, {line}"),
            }
        }
        Almanac::new(vec!(
            Transform::new(seed_soil),
            Transform::new(soil_fertilizer),
            Transform::new(fertilizer_water),
            Transform::new(water_light),
            Transform::new(light_temperature),
            Transform::new(temperature_humidity),
            Transform::new(humidity_location)
        ))
    }
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let numbers: Vec<u32> = value.split(" ").map(|s| s.parse().unwrap()).collect();
        Range {
            destination: numbers[0],
            source: numbers[1],
            length: numbers[2],
        }
    }
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day05Solver {
    Advent2023Day05Solver::new(String::from("\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"))
}

#[test]
fn simple_seeds() {
    let solver = test_solver_1();
    let locations: Vec<u32> = solver.seeds.iter().map(|seed| solver.almanac.seed_location(*seed)).collect();
    assert_eq!(locations, vec!(82, 43, 86, 35));
    assert_eq!(solver.solve_part1(), 35);
}

#[test]
fn range_seeds() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part2(), 46);
}
