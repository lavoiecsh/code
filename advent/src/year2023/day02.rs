use crate::solver::AdventSolver;

pub struct Advent2023Day02Solver {
    games: Vec<Game>,
}

impl Advent2023Day02Solver {
    pub fn new(input: String) -> Self {
        Self { games: input.lines().map(Game::from).collect() }
    }
}

impl AdventSolver for Advent2023Day02Solver {
    fn solve_part1(&self) -> usize {
        self.games.iter()
            .filter(|g| g.is_possible())
            .map(|g| g.id)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.games.iter()
            .map(|g| g.power())
            .sum()
    }
}

struct Game {
    id: usize,
    reveals: Vec<Reveal>,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.reveals.iter().all(Reveal::is_possible)
    }

    fn power(&self) -> usize {
        let all_cubes: Vec<&Cube> = self.reveals.iter()
            .flat_map(|r| r.cubes.iter())
            .collect();
        let minimum_red = all_cubes.iter().filter_map(|c| c.red_count()).max().unwrap();
        let minimum_green = all_cubes.iter().filter_map(|c| c.green_count()).max().unwrap();
        let minimum_blue = all_cubes.iter().filter_map(|c| c.blue_count()).max().unwrap();
        minimum_red * minimum_green * minimum_blue
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut s = value.split(": ");
        Self {
            id: s.next().unwrap().split(" ").skip(1).next().unwrap().parse().unwrap(),
            reveals: s.next().unwrap().split("; ").map(Reveal::from).collect(),
        }
    }
}

struct Reveal {
    cubes: Vec<Cube>,
}

impl Reveal {
    fn is_possible(&self) -> bool {
        self.cubes.iter().all(Cube::is_possible)
    }
}

impl From<&str> for Reveal {
    fn from(value: &str) -> Self {
        Self { cubes: value.split(", ").map(Cube::from).collect() }
    }
}

enum Cube {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl Cube {
    fn is_possible(&self) -> bool {
        match self {
            Cube::Red(x) => *x <= 12,
            Cube::Green(x) => *x <= 13,
            Cube::Blue(x) => *x <= 14,
        }
    }

    fn red_count(&self) -> Option<usize> {
        if let Cube::Red(count) = self { Some(*count) } else { None }
    }

    fn green_count(&self) -> Option<usize> {
        if let Cube::Green(count) = self { Some(*count) } else { None }
    }

    fn blue_count(&self) -> Option<usize> {
        if let Cube::Blue(count) = self { Some(*count) } else { None }
    }
}

impl From<&str> for Cube {
    fn from(value: &str) -> Self {
        let s: Vec<&str> = value.split(" ").collect();
        let count = s[0].parse().unwrap();
        match s[1] {
            "blue" => Cube::Blue(count),
            "red" => Cube::Red(count),
            "green" => Cube::Green(count),
            x => panic!("unknown color {x}"),
        }
    }
}

#[test]
fn validates_game_is_possible() {
    assert!(Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").is_possible());
    assert!(Game::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").is_possible());
    assert!(!Game::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red").is_possible());
    assert!(!Game::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red").is_possible());
    assert!(Game::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").is_possible());
}

#[test]
fn power_returns_power_of_minimum_cubes() {
    assert_eq!(Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").power(), 48);
    assert_eq!(Game::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue").power(), 12);
    assert_eq!(Game::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red").power(), 1560);
    assert_eq!(Game::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red").power(), 630);
    assert_eq!(Game::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green").power(), 36);
}
