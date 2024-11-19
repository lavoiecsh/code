use crate::solver::AdventSolver;

pub struct Advent2017Day09Solver {
    characters: Vec<char>,
}

impl Advent2017Day09Solver {
    pub fn new(input: &str) -> Self {
        Self {
            characters: input.chars().collect(),
        }
    }
}

impl AdventSolver for Advent2017Day09Solver {
    fn solve_part1(&self) -> usize {
        Parser::new(&self.characters).parse().score(1)
    }

    fn solve_part2(&self) -> usize {
        Parser::new(&self.characters).parse().garbage_length()
    }
}

struct Parser<'a> {
    characters: &'a Vec<char>,
    index: usize,
}

impl<'a> Parser<'a> {
    fn new(characters: &'a Vec<char>) -> Self {
        Self {
            characters,
            index: 0,
        }
    }

    fn parse(&mut self) -> Group {
        if self.characters[self.index] == '{' {
            self.parse_group()
        } else {
            self.parse_garbage()
        }
    }

    fn parse_garbage(&mut self) -> Group {
        let mut garbage = Vec::new();
        let mut skip = false;
        loop {
            garbage.push(self.characters[self.index]);
            if skip {
                skip = false;
                self.index += 1;
                continue;
            }
            match self.characters[self.index] {
                '!' => {
                    skip = true;
                }
                '>' => {
                    break;
                }
                _ => {}
            }
            self.index += 1;
        }
        self.index += 1;
        Group::Garbage(garbage)
    }

    fn parse_group(&mut self) -> Group {
        self.index += 1;
        let mut subgroups = Vec::new();
        while self.index != self.characters.len() && self.characters[self.index] != '}' {
            match self.characters[self.index] {
                '{' => {
                    subgroups.push(self.parse_group());
                }
                ',' => {
                    self.index += 1;
                }
                _ => {
                    subgroups.push(self.parse_garbage());
                }
            }
        }
        self.index += 1;
        Group::Group(subgroups)
    }
}

enum Group {
    Garbage(Vec<char>),
    Group(Vec<Group>),
}

impl Group {
    fn score(&self, recursive: usize) -> usize {
        match self {
            Group::Garbage(_) => 0,
            Group::Group(groups) => {
                recursive + groups.iter().map(|g| g.score(recursive + 1)).sum::<usize>()
            }
        }
    }

    fn garbage_length(&self) -> usize {
        match self {
            Group::Garbage(garbage) => garbage_length(garbage),
            Group::Group(groups) => groups.iter().map(|g| g.garbage_length()).sum::<usize>(),
        }
    }
}

fn garbage_length(garbage: &[char]) -> usize {
    let mut count = 0;
    let mut skip = false;
    for g in garbage.iter().take(garbage.len() - 1).skip(1) {
        if skip {
            skip = false;
        } else if *g == '!' {
            skip = true;
        } else {
            count += 1;
        }
    }
    count
}
