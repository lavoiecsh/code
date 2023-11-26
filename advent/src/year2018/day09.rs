use std::collections::VecDeque;
use itertools::Itertools;
use crate::solver::AdventSolver;

pub struct Advent2018Day09Solver {
    players: usize,
    last_marble: usize,
}

impl Advent2018Day09Solver {
    pub fn new(input: String) -> Self {
        let split = input.split(" ").collect_vec();
        Self {
            players: split[0].parse().unwrap(),
            last_marble: split[6].parse().unwrap(),
        }
    }
}

impl AdventSolver for Advent2018Day09Solver {
    fn solve_part1(&self) -> usize {
        let mut game = Game::new(self.players);
        game.play(self.last_marble);
        *game.players.iter().max().unwrap()
    }

    fn solve_part2(&self) -> usize {
        let mut game = Game::new(self.players);
        game.play(self.last_marble * 100);
        *game.players.iter().max().unwrap()
    }
}

struct Game {
    marbles: VecDeque<usize>,
    players: Vec<usize>,
}

impl Game {
    fn new(player_count: usize) -> Self {
        let mut players = Vec::new();
        players.resize(player_count, 0);
        Self { marbles: VecDeque::from([0]), players }
    }

    fn play(&mut self, last_marble: usize) {
        let mut current_player = 1;
        self.marbles.push_front(1);
        for m in 2..=last_marble {
            if let Some(score) = self.insert(m) {
                self.players[current_player] += score;
            }
            current_player += 1;
            current_player %= self.players.len();
        }
    }

    fn insert(&mut self, marble: usize) -> Option<usize> {
        if marble % 23 != 0 {
            self.marbles.rotate_left(2);
            self.marbles.push_front(marble);
            None
        } else {
            self.marbles.rotate_right(7);
            let second = self.marbles.pop_front().unwrap();
            Some(marble + second)
        }
    }
}
