use std::iter::repeat;

use crate::solver::AdventSolver;

type Algorithm = Vec<char>;
type Image = Vec<Vec<char>>;

pub struct Advent2021Day20Solver {
    algorithm: Algorithm,
    image: Image,
}

impl Advent2021Day20Solver {
    pub fn new(input: String) -> Self {
        let mut iter = input.trim().lines();
        let algorithm = iter.next().unwrap().to_string().chars().collect();
        iter.next();
        Self { algorithm, image: iter.map(|l| l.chars().collect()).collect() }
    }

    fn execute(&self, input: &Image, infinity_char: char) -> Image {
        let augmented_input = augment(input, infinity_char);
        let mut output: Image = Vec::new();
        for y in 0..augmented_input.len() {
            output.push(Vec::new());
            for x in 0..augmented_input[0].len() {
                let index = calc_index(&augmented_input, y, x, infinity_char);
                output[y].push(self.algorithm[index]);
            }
        }
        output
    }
}

fn augment(input: &Image, infinity_char: char) -> Image {
    let extension: usize = 1;
    let mut output: Image = Vec::new();
    for _ in 0..extension {
        output.push(repeat(infinity_char).take(input[0].len()+extension*2).collect());
    }
    output.extend(input.iter().map(|l| {
        let mut new_line = Vec::new();
        new_line.extend(repeat(infinity_char).take(extension));
        new_line.extend(l.iter());
        new_line.extend(repeat(infinity_char).take(extension));
        new_line
    }));
    for _ in 0..extension {
        output.push(repeat(infinity_char).take(input[0].len()+extension*2).collect());
    }
    output
}

fn calc_index(input: &Image, y: usize, x: usize, infinity_char: char) -> usize {
    let mut index: usize = 0;
    for y2 in y as isize-1..=y as isize+1 {
        for x2 in x as isize-1..=x as isize+1 {
            if y2 == -1 || x2 == -1 || y2 as usize == input.len() || x2 as usize == input[0].len() {
                index = index * 2 + if infinity_char == '#' { 1 } else { 0 };
            } else {
                index = index * 2 + if input[y2 as usize][x2 as usize] == '#' { 1 } else { 0 };
            }
        }
    }
    index
}

impl AdventSolver for Advent2021Day20Solver {
    fn day(&self) -> usize { 20 }
    fn year(&self) -> usize { 2021 }

    fn solve_part1(&self) -> usize {
        let first = self.execute(&self.image, '.');
        let second = self.execute(&first, '#');
        second.iter().fold(0, |acc, l| acc + l.iter().fold(0, |acc,c|acc + if *c == '#' { 1 } else { 0 }))
    }

    fn solve_part2(&self) -> usize {
        let mut image = self.image.clone();
        for i in 0..50 {
            image = self.execute(&image, if i % 2 == 0 { '.' } else { '#' });
        }
        image.iter().fold(0, |acc, l| acc + l.iter().fold(0, |acc,c|acc + if *c == '#' { 1 } else { 0 }))
    }
}
