use crate::solver::AdventSolver;

struct Entry {
    patterns: Vec<String>,
    outputs: Vec<String>,
}

impl Entry {
    fn calculate(&self) -> usize {
        const STRING: String = String::new();
        let mut found_patterns: [String; 10] = [STRING; 10];
        found_patterns[1] = self
            .patterns
            .iter()
            .find(|p| p.len() == SEGMENT_COUNT[1])
            .unwrap()
            .to_string();
        found_patterns[4] = self
            .patterns
            .iter()
            .find(|p| p.len() == SEGMENT_COUNT[4])
            .unwrap()
            .to_string();
        found_patterns[7] = self
            .patterns
            .iter()
            .find(|p| p.len() == SEGMENT_COUNT[7])
            .unwrap()
            .to_string();
        found_patterns[8] = self
            .patterns
            .iter()
            .find(|p| p.len() == SEGMENT_COUNT[8])
            .unwrap()
            .to_string();

        found_patterns[9] = self
            .patterns
            .iter()
            .find(|p| p.len() == SEGMENT_COUNT[9] && contains_all(&found_patterns[4], p))
            .unwrap()
            .to_string();
        found_patterns[0] = self
            .patterns
            .iter()
            .find(|p| {
                p.len() == SEGMENT_COUNT[0]
                    && **p != found_patterns[9]
                    && contains_all(&found_patterns[7], p)
            })
            .unwrap()
            .to_string();
        found_patterns[6] = self
            .patterns
            .iter()
            .find(|p| {
                p.len() == SEGMENT_COUNT[6] && **p != found_patterns[0] && **p != found_patterns[9]
            })
            .unwrap()
            .to_string();

        found_patterns[3] = self
            .patterns
            .iter()
            .find(|p| p.len() == SEGMENT_COUNT[3] && contains_all(&found_patterns[1], p))
            .unwrap()
            .to_string();
        found_patterns[5] = self
            .patterns
            .iter()
            .find(|p| {
                p.len() == SEGMENT_COUNT[5]
                    && **p != found_patterns[3]
                    && contains_all(p, &found_patterns[9])
            })
            .unwrap()
            .to_string();
        found_patterns[2] = self
            .patterns
            .iter()
            .find(|p| {
                p.len() == SEGMENT_COUNT[2] && **p != found_patterns[3] && **p != found_patterns[5]
            })
            .unwrap()
            .to_string();

        self.outputs
            .iter()
            .fold(0, |acc, o| acc * 10 + lookup(&found_patterns, o))
    }
}

fn contains_all(search: &str, input: &str) -> bool {
    search.chars().all(|c| input.contains(c))
}

fn lookup(patterns: &[String; 10], output: &String) -> usize {
    for n in 0..10 {
        if patterns[n] == *output {
            return n;
        }
    }
    usize::MAX
}

const SEGMENT_COUNT: [usize; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];

pub struct Advent2021Day08Solver {
    entries: Vec<Entry>,
}

impl Advent2021Day08Solver {
    pub fn new(input: &str) -> Self {
        Self {
            entries: input.lines().map(line_to_entry).collect(),
        }
    }
}

impl AdventSolver for Advent2021Day08Solver {
    fn solve_part1(&self) -> usize {
        self.entries
            .iter()
            .flat_map(|entry| entry.outputs.to_vec())
            .fold(0, |acc, output| acc + if is_1478(&output) { 1 } else { 0 })
    }

    fn solve_part2(&self) -> usize {
        self.entries
            .iter()
            .fold(0, |acc, entry| acc + entry.calculate())
    }
}

fn is_1478(item: &str) -> bool {
    item.len() == SEGMENT_COUNT[1]
        || item.len() == SEGMENT_COUNT[4]
        || item.len() == SEGMENT_COUNT[7]
        || item.len() == SEGMENT_COUNT[8]
}

fn line_to_entry(line: &str) -> Entry {
    let mut sides = line.split(" | ");
    Entry {
        patterns: sides
            .next()
            .unwrap()
            .split(" ")
            .map(String::from)
            .map(sort_string)
            .collect(),
        outputs: sides
            .next()
            .unwrap()
            .split(" ")
            .map(String::from)
            .map(sort_string)
            .collect(),
    }
}

fn sort_string(input: String) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort();
    String::from_iter(chars.iter())
}
