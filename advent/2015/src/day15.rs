use std::fs;
use regex::Regex;

const FILENAME: &str = "inputs/day15.txt";

struct Ingredient {
    name: String,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

fn read_input() -> Vec<Ingredient> {
    let re = Regex::new(r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
    fs::read_to_string(FILENAME)
        .expect("error reading")
        .trim()
        .lines()
        .map(|l| {
            let m = re.captures(l).unwrap();
            let p = |n| m.get(n).unwrap().as_str().parse().unwrap();
            Ingredient {
                name: m.get(1).unwrap().as_str().to_string(),
                capacity: p(2),
                durability: p(3),
                flavor: p(4),
                texture: p(5),
                calories: p(6),
            }
        })
        .collect()
}

pub fn part1() -> usize {
    const TOTAL_TEASPOONS: usize = 100;
    const INGREDIENT_COUNT: usize = 4;
    let ingredients = read_input();
    let mut best_score: usize = 0;
    for t0 in 0..100 {
        for t1 in 0..(100-t0) {
            for t2 in 0..(100-t0-t1) {
                let t3: usize = 100 - t0 - t1 - t2;
                let t: [usize; 4] = [t0,t1,t2,t3];
                let capacity: isize = (0..INGREDIENT_COUNT).map(|i| ingredients[i].capacity * t[i] as isize).sum();
                let durability: isize = (0..INGREDIENT_COUNT).map(|i| ingredients[i].durability * t[i] as isize).sum();
                let flavor: isize = (0..INGREDIENT_COUNT).map(|i| ingredients[i].flavor * t[i] as isize).sum();
                let texture: isize = (0..INGREDIENT_COUNT).map(|i| ingredients[i].texture * t[i] as isize).sum();
                if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
                    continue;
                }
                let score: usize = capacity as usize * durability as usize * flavor as usize * texture as usize;
                if score > best_score {
                    best_score = score;
                }
            }
        }
    }
    best_score
}

pub fn part2() -> usize {
    const TOTAL_TEASPOONS: usize = 100;
    const INGREDIENT_COUNT: usize = 4;
    let ingredients = read_input();
    let mut best_score: usize = 0;
    for t0 in 0..100 {
        for t1 in 0..(100-t0) {
            for t2 in 0..(100-t0-t1) {
                let t3: usize = 100 - t0 - t1 - t2;
                let t: [usize; 4] = [t0,t1,t2,t3];
                let capacity: isize = (0..INGREDIENT_COUNT).map(|i| ingredients[i].capacity * t[i] as isize).sum();
                let durability: isize = (0..INGREDIENT_COUNT).map(|i| ingredients[i].durability * t[i] as isize).sum();
                let flavor: isize = (0..INGREDIENT_COUNT).map(|i| ingredients[i].flavor * t[i] as isize).sum();
                let texture: isize = (0..INGREDIENT_COUNT).map(|i| ingredients[i].texture * t[i] as isize).sum();
                let calories: isize = (0..INGREDIENT_COUNT).map(|i| ingredients[i].calories * t[i] as isize).sum();
                if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 || calories != 500 {
                    continue;
                }
                let score: usize = capacity as usize * durability as usize * flavor as usize * texture as usize;
                if score > best_score {
                    best_score = score;
                }
            }
        }
    }
    best_score
}
