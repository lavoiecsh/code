#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

use std::iter::repeat;

const FILENAME: &str = "inputs/day20.txt";

type Algorithm = Vec<char>;
type Image = Vec<Vec<char>>;

#[allow(dead_code)]
fn pp(image: &Image) -> Vec<String> {
    image.iter().map(|l|l.iter().collect::<String>()).collect::<Vec<String>>()
}

#[allow(dead_code)]
fn read_input() -> (Algorithm, Image) {
    let input = fs::read_to_string(FILENAME).expect("error reading");
    let mut iter = input.trim().lines();
    let algorithm = iter.next().unwrap().to_string();
    iter.next();
    (algorithm.chars().collect(), iter.map(|l|l.chars().collect()).collect())
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let (algorithm, image) = read_input();
    let first = execute(&algorithm, &image, '.');
    let second = execute(&algorithm, &first, '#');
    second.iter().fold(0, |acc, l| acc + l.iter().fold(0, |acc,c|acc + if *c == '#' { 1 } else { 0 }))
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let (algorithm, mut image) = read_input();
    for i in 0..50 {
        image = execute(&algorithm, &image, if i % 2 == 0 { '.' } else { '#' });
    }
    image.iter().fold(0, |acc, l| acc + l.iter().fold(0, |acc,c|acc + if *c == '#' { 1 } else { 0 }))
}

fn execute(algorithm: &Algorithm, input: &Image, infinity_char: char) -> Image {
    let augmented_input = augment(input, infinity_char);
    let mut output: Image = Vec::new();
    for y in 0..augmented_input.len() {
        output.push(Vec::new());
        for x in 0..augmented_input[0].len() {
            let index = calc_index(&augmented_input, y, x, infinity_char);
            output[y].push(algorithm[index]);
        }
    }
    output
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
