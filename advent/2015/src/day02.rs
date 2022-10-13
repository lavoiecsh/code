use std::fs;

const FILENAME: &str = "inputs/day02.txt";

struct Box {
    l: usize,
    w: usize,
    h: usize,
}

impl Box {
    fn new(s: &str) -> Box {
        let mut dimensions = s.split("x");
        Box {
            l: dimensions.next().unwrap().parse().expect("error parsing"),
            w: dimensions.next().unwrap().parse().expect("error parsing"),
            h: dimensions.next().unwrap().parse().expect("error parsing"),
        }
    }

    fn area_part1(&self) -> usize {
        let side_areas: [usize; 3] = [self.l*self.w, self.w*self.h, self.h*self.l];
        side_areas.iter().map(|x|x*2).sum::<usize>() + side_areas.iter().min().unwrap()
    }

    fn area_part2(&self) -> usize {
        let mut sides: [usize; 3] = [self.l, self.w, self.h];
        sides.sort();
        sides.iter().map(|x|x*2).take(2).sum::<usize>() + self.volume()
    }

    fn volume(&self) -> usize {
        self.l * self.w * self.h
    }
}

fn read_input() -> Vec<Box> {
    fs::read_to_string(FILENAME)
        .expect("error reading")
        .trim()
        .lines()
        .map(Box::new)
        .collect()
}

pub fn part1() -> usize {
    read_input()
        .iter()
        .map(Box::area_part1)
        .sum()
}

pub fn part2() -> usize {
    read_input()
        .iter()
        .map(Box::area_part2)
        .sum()
}
