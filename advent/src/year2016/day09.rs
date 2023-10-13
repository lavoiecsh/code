use crate::solver::AdventSolver;

pub struct Advent2016Day09Solver {
    message: String
}

impl Advent2016Day09Solver {
    pub fn new(input: String) -> Self {
        Self { message: input }
    }
}

impl AdventSolver for Advent2016Day09Solver {
    fn solve_part1(&self) -> usize {
        decompress(&self.message).len()
    }

    fn solve_part2(&self) -> usize {
        decompressed_length(&self.message)
    }
}

fn decompress(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut output: Vec<_> = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '(' {
            output.push(chars[i]);
            i += 1;
            continue;
        }
        let mut l: Vec<char> = Vec::new();
        i += 1;
        while chars[i] != 'x' {
            l.push(chars[i]);
            i += 1;
        }
        let mut c: Vec<char> = Vec::new();
        i += 1;
        while chars[i] != ')' {
            c.push(chars[i]);
            i += 1;
        }
        let length = l.iter().collect::<String>().parse::<usize>().unwrap();
        let count = c.iter().collect::<String>().parse::<usize>().unwrap();
        i += 1;
        for _ in 0..count {
            for j in i..(i+length) {
                output.push(chars[j]);
            }
        }
        i += length;
    }
    output.iter().collect()
}

fn decompressed_length(input: &str) -> usize {
    let mut total = 0;
    let mut it = input.chars();
    let mut reading_length = false;
    let mut reading_count = false;
    let mut length_str: Vec<char> = Vec::new();
    let mut count_str: Vec<char> = Vec::new();
    while let Some(c) = it.next() {
        match (c, reading_length, reading_count) {
            ('(', false, false) => {
                reading_length = true;
                length_str = Vec::new();
            },
            ('x', true, false) => {
                reading_length = false;
                reading_count = true;
                count_str = Vec::new();
            },
            (')', false, true) => {
                reading_count = false;
                let length: usize = length_str.iter().collect::<String>().parse().unwrap();
                let count: usize = count_str.iter().collect::<String>().parse().unwrap();
                let mut tmp: Vec<char> = Vec::new();
                (0..length).for_each(|_| tmp.push(it.next().unwrap()));
                total += decompressed_length(&tmp.iter().collect::<String>()) * count;
            },
            (x, true, false) => {
                length_str.push(x);
            },
            (x, false, true) => {
                count_str.push(x);
            },
            (_, false, false) => total += 1,
            _ => {},
        }
    }
    total
}
