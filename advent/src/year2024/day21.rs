use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub struct Advent2024Day21Solver {
    codes: Vec<NumericCode>,
}

impl Advent2024Day21Solver {
    pub fn new(input: &str) -> Self {
        Self {
            codes: input.lines().map(NumericCode::from).collect(),
        }
    }
}

impl AdventSolver for Advent2024Day21Solver {
    fn solve_part1(&self) -> usize {
        self.codes.iter().map(|c| c.complexity(2)).sum()
    }

    fn solve_part2(&self) -> usize {
        self.codes.iter().map(|c| c.complexity(25)).sum()
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct NumericCode {
    code: String,
}

#[derive(Clone, Eq, PartialEq)]
struct DirectionalCode {
    pairs: HashMap<(char, char), usize>,
}

impl NumericCode {
    fn complexity(&self, robot_count: usize) -> usize {
        self.shortest_sequence(robot_count) * self.value()
    }

    fn shortest_sequence(&self, robot_count: usize) -> usize {
        let mut codes = self.to_directional();
        for _i in 0..robot_count {
            codes = codes.into_iter().flat_map(|code| code.to_directional()).collect();
            let min_length = codes.iter().map(|code| code.len()).min().unwrap_or(0);
            codes = codes.into_iter().filter(|code| code.len() == min_length).collect();
        }
        codes.iter().map(|code| code.len()).min().unwrap_or(0)
    }

    fn value(&self) -> usize {
        self.code.chars().fold(0, |acc, cur| match cur {
            'A' => acc,
            num => acc * 10 + (num as usize - '0' as usize),
        })
    }

    fn to_directional(&self) -> Vec<DirectionalCode> {
        let mut codes = Vec::new();
        codes.push("".to_string());
        let mut last_position = 'A';
        for position in self.code.chars() {
            let next_directions = directions_num(last_position, position);
            codes = codes
                .into_iter()
                .cartesian_product(next_directions)
                .map(|(c,d)| { let mut nc = String::from(c); nc.push_str(d); nc.push('A'); nc })
                .collect();
            last_position = position;
        }
        codes
            .into_iter()
            .map(|code| DirectionalCode::from(code))
            .collect()
    }
}

impl DirectionalCode {
    fn new() -> Self {
        Self { pairs: HashMap::new() }
    }

    fn len(&self) -> usize {
        self.pairs.values().sum::<usize>()
    }

    fn push(&self, path: &'static str, count: usize) -> Self{
        let mut clone = self.clone();
        let mut last = 'A';
        for c in path.chars() {
            clone.pairs.entry((last, c)).and_modify(|e| *e += count).or_insert(count);
            last = c;
        }
        clone.pairs.entry((last, 'A')).and_modify(|e| *e += count).or_insert(count);
        clone
    }

    fn to_directional(&self) -> Vec<DirectionalCode> {
        let mut nexts = vec![DirectionalCode::new()];
        for (&(from, to), &count) in &self.pairs {
            let path = directions_dir(from, to);
            nexts = nexts
                .into_iter()
                .map(|n| n.push(path, count))
                .collect();
        }
        let min_length = nexts.iter().map(|n| n.len()).min().unwrap_or(0);
        nexts.into_iter().filter(|n| n.len() == min_length).collect()
    }
}

impl From<String> for DirectionalCode {
    fn from(value: String) -> Self {
        let mut code = DirectionalCode::new();
        let mut last = 'A';
        for c in value.chars() {
            code.pairs.entry((last, c)).and_modify(|c| *c += 1).or_insert(1);
            last = c;
        }
        code
    }
}

fn directions_num(from: char, to: char) -> Vec<&'static str> {
    match (from, to) {
        ('A', 'A') => vec![""],
        ('A', '0') => vec!["<"],
        ('A', '1') => vec!["^<<", "<^<"],
        ('A', '2') => vec!["^<", "<^"],
        ('A', '3') => vec!["^"],
        ('A', '4') => vec!["^^<<", "^<^<", "^<<^", "<^^<", "<^<^"],
        ('A', '5') => vec!["^^<", "^<^", "<^^"],
        ('A', '6') => vec!["^^"],
        ('A', '7') => vec!["^^^<<", "^^<^<", "^^<<^", "^<^^<", "^<^<^", "^<<^^", "<^^^<", "<^^<^", "<^<^^"],
        ('A', '8') => vec!["^^^<", "^^<^", "^<^^", "<^^^"],
        ('A', '9') => vec!["^^^"],

        ('0', 'A') => vec![">"],
        ('0', '0') => vec![""],
        ('0', '1') => vec!["^<"],
        ('0', '2') => vec!["^"],
        ('0', '3') => vec!["^>", ">^"],
        ('0', '4') => vec!["^^<", "^<^"],
        ('0', '5') => vec!["^^"],
        ('0', '6') => vec!["^^>", "^>^", ">^^"],
        ('0', '7') => vec!["^^^<", "^^<^", "^<^^"],
        ('0', '8') => vec!["^^^"],
        ('0', '9') => vec!["^^^>", "^^>^", "^>^^", ">^^^"],

        ('1', 'A') => vec![">>v", ">v>"],
        ('1', '0') => vec![">v"],
        ('1', '1') => vec![""],
        ('1', '2') => vec![">"],
        ('1', '3') => vec![">>"],
        ('1', '4') => vec!["^"],
        ('1', '5') => vec!["^>", ">^"],
        ('1', '6') => vec!["^>>", ">^>", ">>^"],
        ('1', '7') => vec!["^^"],
        ('1', '8') => vec!["^^>", "^>^", ">^^"],
        ('1', '9') => vec!["^^>>", "^>^>", "^>>^", ">^^>", ">^>^", ">>^^"],

        ('2', 'A') => vec!["v>", ">v"],
        ('2', '0') => vec!["v"],
        ('2', '1') => vec!["<"],
        ('2', '2') => vec![""],
        ('2', '3') => vec![">"],
        ('2', '4') => vec!["^<", "<^"],
        ('2', '5') => vec!["^"],
        ('2', '6') => vec!["^>", ">^"],
        ('2', '7') => vec!["^^<", "^<^", "<^^"],
        ('2', '8') => vec!["^^"],
        ('2', '9') => vec!["^^>", "^>^", ">^^"],

        ('3', 'A') => vec!["v"],
        ('3', '0') => vec!["v<", "<v"],
        ('3', '1') => vec!["<<"],
        ('3', '2') => vec!["<"],
        ('3', '3') => vec![""],
        ('3', '4') => vec!["^<<", "<^<", "<<^"],
        ('3', '5') => vec!["^<", "<^"],
        ('3', '6') => vec!["^"],
        ('3', '7') => vec!["^^<<", "^<^<", "^<<^", "<^^<", "<^<^", "<<^^"],
        ('3', '8') => vec!["^^<", "^<^", "<^^"],
        ('3', '9') => vec!["^^"],

        ('4', 'A') => vec!["v>v>", "v>>v", ">vv>", ">v>v", ">>vv"],
        ('4', '0') => vec!["v>v", ">vv"],
        ('4', '1') => vec!["v"],
        ('4', '2') => vec!["v>", ">v"],
        ('4', '3') => vec!["v>>", ">v>", ">>v"],
        ('4', '4') => vec![""],
        ('4', '5') => vec![">"],
        ('4', '6') => vec![">>"],
        ('4', '7') => vec!["^"],
        ('4', '8') => vec!["^>", ">^"],
        ('4', '9') => vec!["^>>", ">^>", ">>^"],

        ('5', 'A') => vec!["vv>", "v>v", ">vv"],
        ('5', '0') => vec!["vv"],
        ('5', '1') => vec!["v<", "<v"],
        ('5', '2') => vec!["v"],
        ('5', '3') => vec!["v>", ">v"],
        ('5', '4') => vec!["<"],
        ('5', '5') => vec![""],
        ('5', '6') => vec![">"],
        ('5', '7') => vec!["^<", "<^"],
        ('5', '8') => vec!["^"],
        ('5', '9') => vec!["^>", ">^"],

        ('6', 'A') => vec!["vv"],
        ('6', '0') => vec!["vv<", "v<v", "<vv"],
        ('6', '1') => vec!["v<<", "<v<", "<<v"],
        ('6', '2') => vec!["v<", "<v"],
        ('6', '3') => vec!["v"],
        ('6', '4') => vec!["<<"],
        ('6', '5') => vec!["<"],
        ('6', '6') => vec![""],
        ('6', '7') => vec!["^<<", "<^<", "<<^"],
        ('6', '8') => vec!["^<", "<^"],
        ('6', '9') => vec!["^"],

        ('7', 'A') => vec!["vv>v>", "vv>>v", "v>vv>", "v>v>v", "v>>vv", ">vvv>", ">vv>v", ">v>vv", ">>vvv"],
        ('7', '0') => vec!["vv>v", "v>vv", ">vvv"],
        ('7', '1') => vec!["vv"],
        ('7', '2') => vec!["vv>", "v>v", ">vv"],
        ('7', '3') => vec!["vv>>", "v>v>", "v>>v", ">vv>", ">v>v", ">>vv"],
        ('7', '4') => vec!["v"],
        ('7', '5') => vec!["v>", ">v"],
        ('7', '6') => vec!["v>>", ">v>", ">>v"],
        ('7', '7') => vec![""],
        ('7', '8') => vec![">"],
        ('7', '9') => vec![">>"],

        ('8', 'A') => vec!["vvv>", "vv>v", "v>vv", ">vvv"],
        ('8', '0') => vec!["vvv"],
        ('8', '1') => vec!["vv<", "v<v", "<vv"],
        ('8', '2') => vec!["vv"],
        ('8', '3') => vec!["vv>", "v>v", ">vv"],
        ('8', '4') => vec!["v<", "<v"],
        ('8', '5') => vec!["v"],
        ('8', '6') => vec!["v>", ">v"],
        ('8', '7') => vec!["<"],
        ('8', '8') => vec![""],
        ('8', '9') => vec![">"],

        ('9', 'A') => vec!["vvv"],
        ('9', '0') => vec!["vvv<", "vv<v", "v<vv", "<vvv"],
        ('9', '1') => vec!["vv<<", "v<v<", "v<<v", "<vv<", "<v<v", "<<vv"],
        ('9', '2') => vec!["vv<", "v<v", "<vv"],
        ('9', '3') => vec!["vv"],
        ('9', '4') => vec!["v<<", "<v<", "<<v"],
        ('9', '5') => vec!["v<", "<v"],
        ('9', '6') => vec!["v"],
        ('9', '7') => vec!["<<"],
        ('9', '8') => vec!["<"],
        ('9', '9') => vec![""],

        _ => unreachable!("invalid numeric keypad character ({from}, {to})"),
    }
}

fn directions_dir(from: char, to: char) -> &'static str {
    match (from, to) {
        ('A', 'A') => "",
        ('A', '^') => "<",
        ('A', '>') => "v",
        ('A', 'v') => "<v",
        ('A', '<') => "v<<",
        ('^', '^') => "",
        ('^', 'A') => ">",
        ('^', 'v') => "v",
        ('^', '<') => "v<",
        ('^', '>') => "v>",
        ('<', '<') => "",
        ('<', 'v') => ">",
        ('<', '>') => ">>",
        ('<', '^') => ">^",
        ('<', 'A') => ">>^",
        ('v', 'v') => "",
        ('v', '<') => "<",
        ('v', '>') => ">",
        ('v', '^') => "^",
        ('v', 'A') => "^>",
        ('>', '>') => "",
        ('>', 'v') => "<",
        ('>', '<') => "<<",
        ('>', 'A') => "^",
        ('>', '^') => "<^",
        _ => unreachable!("invalid directional characters ({from}, {to})"),
    }
}

impl From<&str> for NumericCode {
    fn from(value: &str) -> Self {
        Self {
            code: value.to_string(),
        }
    }
}

impl Debug for DirectionalCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("\nDirectionalCode, len = {}\n{}", self.len(), self.pairs.iter().map(|((f,t),c)| format!("{f}{t} = {c}")).join("\n")))
    }
}

//noinspection SpellCheckingInspection
#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &str = "\
029A
980A
179A
456A
379A
";

    #[test]
    fn computes_lengths_of_shortest_sequence_correctly() {
        let solver = Advent2024Day21Solver::new(EXAMPLE);
        assert_eq!(solver.codes[0].shortest_sequence(2), 68);
        assert_eq!(solver.codes[1].shortest_sequence(2), 60);
        assert_eq!(solver.codes[2].shortest_sequence(2), 68);
        assert_eq!(solver.codes[3].shortest_sequence(2), 64);
        assert_eq!(solver.codes[4].shortest_sequence(2), 64);
    }

    #[test]
    fn computes_complexity_of_codes() {
        let solver = Advent2024Day21Solver::new(EXAMPLE);
        assert_eq!(solver.solve_part1(), 126384);
    }
}
