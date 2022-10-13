#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

use regex::Regex;

const FILENAME: &str = "inputs/day21.txt";

#[allow(dead_code)]
fn read_input() -> (usize, usize) {
    let re = Regex::new(r"Player . starting position: (\d)").unwrap();
    let input = fs::read_to_string(FILENAME).expect("error reading");
    let mut iter = input.trim().lines();
    let cap1 = re.captures(iter.next().unwrap()).unwrap();
    let cap2 = re.captures(iter.next().unwrap()).unwrap();
    (cap1.get(1).unwrap().as_str().parse().expect("error parsing"),
     cap2.get(1).unwrap().as_str().parse().expect("error parsing"))
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let (p1, p2) = read_input();
    let mut player1 = Player::new(p1);
    let mut player2 = Player::new(p2);
    let mut die = DeterministicDie::new();
    while player1.score < 1000 && player2.score < 1000 {
        player1.play_deterministic_turn(&mut die);
        if player1.score >= 1000 {
            break;
        }
        player2.play_deterministic_turn(&mut die);
    }
    if player1.score >= 1000 {
        player2.score * die.roll_count
    } else {
        player1.score * die.roll_count
    }
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let (p1, p2) = read_input();
    let mut all_games: Vec<(Game, usize)> = vec![(Game::new(p1, p2), 1)];
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    while !all_games.is_empty() {
        let (game, count) = all_games.pop().unwrap();
        let next_games = game.play_turn();
        for ng in next_games {
            if ng.p1.score >= 21 {
                p1_wins += count;
                continue;
            }
            if ng.p2.score >= 21 {
                p2_wins += count;
                continue;
            }
            let mut found = false;
            for i in 0..all_games.len() {
                if all_games[i].0.matches(&ng) {
                    all_games[i].1 += count;
                    found = true;
                    break;
                }
            }
            if !found {
                all_games.push((ng, count));
            }
        }
    }
    if p1_wins > p2_wins { p1_wins } else { p2_wins }
}

struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Self { position: position, score: 0 }
    }

    fn copy(&self) -> Self {
        Self { position: self.position, score: self.score }
    }

    fn play_deterministic_turn(&mut self, die: &mut DeterministicDie) {
        let rolls = die.roll() + die.roll() + die.roll();
        self.play_turn_value(rolls);
    }

    fn play_dirac_turn(&self) -> Vec<Self> {
        let mut nexts: Vec<Self> = Vec::new();
        for t1 in 1..=3 {
            for t2 in 1..=3 {
                for t3 in 1..=3 {
                    nexts.push(self.copy().play_turn_value(t1+t2+t3).copy())
                }
            }
        }
        nexts
    }

    fn play_turn_value(&mut self, value: usize) -> &mut Self {
        self.position += value;
        while self.position > 10 {
            self.position -= 10;
        }
        self.score += self.position;
        self
    }

    fn matches(&self, other: &Self) -> bool {
        self.position == other.position && self.score == other.score
    }
}

struct Game {
    p1: Player,
    p2: Player,
    p1_turn: bool,
}

impl Game {
    fn new(p1: usize, p2: usize) -> Self {
        Self { p1: Player::new(p1), p2: Player::new(p2), p1_turn: true }
    }
    
    fn play_turn(&self) -> Vec<Self> {
        if self.p1_turn {
            self.p1.play_dirac_turn().iter().map(|np1| Self { p1: np1.copy(), p2: self.p2.copy(), p1_turn: false }).collect()
        } else {
            self.p2.play_dirac_turn().iter().map(|np2| Self { p1: self.p1.copy(), p2: np2.copy(), p1_turn: true }).collect()
        }
    }

    fn matches(&self, other: &Self) -> bool {
        self.p1.matches(&other.p1) && self.p2.matches(&other.p2) && self.p1_turn == other.p1_turn
    }
}

impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "p1: {} {}, p2: {} {}, next: {}", self.p1.position, self.p1.score, self.p2.position, self.p2.score, if self.p1_turn { "1" } else { "2" })
    }
}

struct DeterministicDie {
    current: usize,
    roll_count: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        Self { current: 100, roll_count: 0 }
    }

    fn roll(&mut self) -> usize {
        self.roll_count += 1;
        self.current += 1;
        if self.current > 100 {
            self.current = 1;
        }
        self.current
    }
}
