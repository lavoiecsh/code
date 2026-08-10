use itertools::Itertools;
use crate::solver::AdventSolver;

pub struct Advent2019Day08Solver {
    image: Image,
}

impl Advent2019Day08Solver {
    pub fn new(input: &str) -> Self {
        let zero = '0' as u8;
        let digits: Vec<u8> = input.chars().map(|c| (c as u8) - zero).collect();
        let mut layers = Vec::new();
        let mut pixels = Vec::new();
        let mut row = Vec::new();
        for digit in digits {
            row.push(digit);
            if row.len() == 25 {
                pixels.push(row);
                row = Vec::new();
                if pixels.len() == 6 {
                    layers.push(Layer { pixels });
                    pixels = Vec::new();
                }
            }
        }
        Self {
            image: Image { layers },
        }
    }
}

impl AdventSolver for Advent2019Day08Solver {
    fn solve_part1(&self) -> usize {
        let best_layer = self.image.layers.iter()
            .min_by_key(|l| l.digit_count(0))
            .unwrap();
        best_layer.digit_count(1) * best_layer.digit_count(2)
    }

    fn solve_part2_string(&self) -> String {
        let decoded = self.image.decode();
        decoded.iter()
            .map(|r| r.iter().map(|&p| if p == 0 { ' ' } else { '#' }).collect::<String>())
            .join("\n")
    }
}

struct Image {
    layers: Vec<Layer>,
}

impl Image {
    fn decode(&self) -> Vec<Vec<u8>> {
        let mut decoded = vec![vec![2; 25]; 6];
        for layer in &self.layers {
            for row in 0..6 {
                for col in 0..25 {
                    if decoded[row][col] == 2 {
                        decoded[row][col] = layer.pixels[row][col];
                    }
                }
            }
        }
        decoded
    }
}

struct Layer {
    pixels: Vec<Vec<u8>>,
}

impl Layer {
    fn digit_count(&self, digit: u8) -> usize {
        self.pixels.iter()
            .map(|r| r.iter().filter(|&&p| p == digit).count())
            .sum()
    }
}