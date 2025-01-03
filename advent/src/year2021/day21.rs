use regex::Regex;

use crate::solver::AdventSolver;

pub struct Advent2021Day21Solver {
    player1: usize,
    player2: usize,
}

impl Advent2021Day21Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"Player . starting position: (\d)").unwrap();
        let mut iter = input.trim().lines();
        let cap1 = re.captures(iter.next().unwrap()).unwrap();
        let cap2 = re.captures(iter.next().unwrap()).unwrap();
        Self {
            player1: cap1.get(1).unwrap().as_str().parse().unwrap(),
            player2: cap2.get(1).unwrap().as_str().parse().unwrap(),
        }
    }
}

impl AdventSolver for Advent2021Day21Solver {
    fn solve_part1(&self) -> usize {
        let mut player1 = Player::new(self.player1);
        let mut player2 = Player::new(self.player2);
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

    fn solve_part2(&self) -> usize {
        let mut all_games: Vec<(Game, usize)> = vec![(Game::new(self.player1, self.player2), 1)];
        let mut p1_wins = 0;
        let mut p2_wins = 0;
        while let Some((game, count)) = all_games.pop() {
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
        if p1_wins > p2_wins {
            p1_wins
        } else {
            p2_wins
        }
    }
}

#[derive(Copy, Clone)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        Self { position, score: 0 }
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
                    nexts.push(*self.clone().play_turn_value(t1 + t2 + t3))
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
        Self {
            p1: Player::new(p1),
            p2: Player::new(p2),
            p1_turn: true,
        }
    }

    fn play_turn(&self) -> Vec<Self> {
        if self.p1_turn {
            self.p1
                .play_dirac_turn()
                .iter()
                .map(|np1| Self {
                    p1: *np1,
                    p2: self.p2,
                    p1_turn: false,
                })
                .collect()
        } else {
            self.p2
                .play_dirac_turn()
                .iter()
                .map(|np2| Self {
                    p1: self.p1,
                    p2: *np2,
                    p1_turn: true,
                })
                .collect()
        }
    }

    fn matches(&self, other: &Self) -> bool {
        self.p1.matches(&other.p1) && self.p2.matches(&other.p2) && self.p1_turn == other.p1_turn
    }
}

struct DeterministicDie {
    current: usize,
    roll_count: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        Self {
            current: 100,
            roll_count: 0,
        }
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
