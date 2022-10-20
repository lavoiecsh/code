use std::fs;

const FILENAME: &str = "inputs/day08.txt";

fn read_input() -> Vec<String> {
    fs::read_to_string(FILENAME)
        .expect("error reading")
        .trim()
        .lines()
        .map(String::from)
        .collect()
}

pub fn part1() -> usize {
    read_input()
        .iter()
        .map(|l| literal_count(l) - memory_count(l))
        .sum()
}

pub fn part2() -> usize {
    read_input()
        .iter()
        .map(|l| literal_count(&escape_string(l)) - literal_count(l))
        .sum()
}

fn literal_count(l: &String) -> usize {
    l.len()
}

fn memory_count(l: &String) -> usize {
    let mut count: usize = 0;
    let mut i = 0;
    let chars: Vec<char> = l.chars().collect();
    while i < chars.len() {
        if chars[i] == '"' {
            i += 1;
            continue;
        }
        if chars[i] == '\\' {
            if chars[i+1] == '\\' || chars[i+1] == '"' {
                count +=1 ;
                i += 2;
                continue;
            }
            if chars[i+1] == 'x' {
                count += 1;
                i += 4;
                continue;
            }
        }
        count += 1;
        i += 1;
    }
    count
}

fn escape_string(l: &String) -> String {
    let mut escaped: String = l.chars()
        .map(|c| if c == '\\' { "\\\\".to_string() } else if c == '"' { "\\\"".to_string() } else { c.to_string() })
        .collect();
    escaped.insert(0, '"');
    escaped.push('"');
    escaped
}
