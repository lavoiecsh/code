use std::collections::HashMap;

use regex::Regex;

use crate::solver::AdventSolver;

pub struct Advent2015Day16Solver {
    aunts: Vec<Aunt>,
}

impl Advent2015Day16Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"Sue (\d+): (.*)").unwrap();
        Self {
            aunts: input
                .lines()
                .map(|l| {
                    let m = re.captures(l).unwrap();
                    let compounds: HashMap<&str, usize> = m
                        .get(2)
                        .unwrap()
                        .as_str()
                        .split(", ")
                        .map(|c| {
                            let mut s = c.split(": ");
                            (s.next().unwrap(), s.next().unwrap().parse().unwrap())
                        })
                        .collect();
                    Aunt {
                        number: m.get(1).unwrap().as_str().parse().unwrap(),
                        children: compounds.get("children").copied(),
                        cats: compounds.get("cats").copied(),
                        samoyeds: compounds.get("samoyeds").copied(),
                        pomeranians: compounds.get("pomeranians").copied(),
                        akitas: compounds.get("akitas").copied(),
                        vizslas: compounds.get("vizslas").copied(),
                        goldfish: compounds.get("goldfish").copied(),
                        trees: compounds.get("trees").copied(),
                        cars: compounds.get("cars").copied(),
                        perfumes: compounds.get("perfumes").copied(),
                    }
                })
                .collect(),
        }
    }
}

impl AdventSolver for Advent2015Day16Solver {
    fn solve_part1(&self) -> usize {
        let comp = |a: Option<usize>, b: Option<usize>| a.is_none() || a.unwrap() == b.unwrap();
        self.aunts
            .iter()
            .filter(|aunt| {
                comp(aunt.children, ANALYSIS.children)
                    && comp(aunt.cats, ANALYSIS.cats)
                    && comp(aunt.samoyeds, ANALYSIS.samoyeds)
                    && comp(aunt.pomeranians, ANALYSIS.pomeranians)
                    && comp(aunt.akitas, ANALYSIS.akitas)
                    && comp(aunt.vizslas, ANALYSIS.vizslas)
                    && comp(aunt.goldfish, ANALYSIS.goldfish)
                    && comp(aunt.trees, ANALYSIS.trees)
                    && comp(aunt.cars, ANALYSIS.cars)
                    && comp(aunt.perfumes, ANALYSIS.perfumes)
            })
            .map(|aunt| aunt.number)
            .next()
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        let comp = |a: Option<usize>, b: Option<usize>, c: fn(&usize, &usize) -> bool| {
            a.is_none() || c(&a.unwrap(), &b.unwrap())
        };
        self.aunts
            .iter()
            .filter(|aunt| {
                comp(aunt.children, ANALYSIS.children, usize::eq)
                    && comp(aunt.cats, ANALYSIS.cats, usize::gt)
                    && comp(aunt.samoyeds, ANALYSIS.samoyeds, usize::eq)
                    && comp(aunt.pomeranians, ANALYSIS.pomeranians, usize::lt)
                    && comp(aunt.akitas, ANALYSIS.akitas, usize::eq)
                    && comp(aunt.vizslas, ANALYSIS.vizslas, usize::eq)
                    && comp(aunt.goldfish, ANALYSIS.goldfish, usize::lt)
                    && comp(aunt.trees, ANALYSIS.trees, usize::gt)
                    && comp(aunt.cars, ANALYSIS.cars, usize::eq)
                    && comp(aunt.perfumes, ANALYSIS.perfumes, usize::eq)
            })
            .map(|aunt| aunt.number)
            .next()
            .unwrap()
    }
}

struct Aunt {
    number: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

const ANALYSIS: Aunt = Aunt {
    number: 0,
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};
