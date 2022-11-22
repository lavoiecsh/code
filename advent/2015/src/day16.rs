use std::collections::HashMap;
use std::fs;
use regex::Regex;
use crate::problem_solver::ProblemSolver;

pub struct Problem16Solver {
    aunts: Vec<Aunt>
}
impl Problem16Solver {
    pub fn new() -> Self {
        let re = Regex::new(r"Sue (\d+): (.*)").unwrap();
        Self {
            aunts: fs::read_to_string("inputs/day16.txt")
                .expect("error reading")
                .trim()
                .lines()
                .map(|l| {
                    let m = re.captures(l).unwrap();
                    let compounds: HashMap<&str, usize> = m.get(2).unwrap().as_str().split(", ")
                        .map(|c| {
                            let mut s = c.split(": ");
                            (s.next().unwrap(), s.next().unwrap().parse().unwrap())
                        })
                        .collect();
                    Aunt {
                        number: m.get(1).unwrap().as_str().parse().unwrap(),
                        children: compounds.get("children").map(|n| *n),
                        cats: compounds.get("cats").map(|n| *n),
                        samoyeds: compounds.get("samoyeds").map(|n| *n),
                        pomeranians: compounds.get("pomeranians").map(|n| *n),
                        akitas: compounds.get("akitas").map(|n| *n),
                        vizslas: compounds.get("vizslas").map(|n| *n),
                        goldfish: compounds.get("goldfish").map(|n| *n),
                        trees: compounds.get("trees").map(|n| *n),
                        cars: compounds.get("cars").map(|n| *n),
                        perfumes: compounds.get("perfumes").map(|n| *n),
                    }
                })
                .collect()
        }
    }
}

impl ProblemSolver for Problem16Solver {
    fn solve_part1(&self) -> usize {
        let comp = |a: Option<usize>, b: Option<usize>| a.is_none() || a.unwrap() == b.unwrap();
        self.aunts
            .iter()
            .filter(|aunt| comp(aunt.children, ANALYSIS.children) &&
                comp(aunt.cats, ANALYSIS.cats) &&
                comp(aunt.samoyeds, ANALYSIS.samoyeds) &&
                comp(aunt.pomeranians, ANALYSIS.pomeranians) &&
                comp(aunt.akitas, ANALYSIS.akitas) &&
                comp(aunt.vizslas, ANALYSIS.vizslas) &&
                comp(aunt.goldfish, ANALYSIS.goldfish) &&
                comp(aunt.trees, ANALYSIS.trees) &&
                comp(aunt.cars, ANALYSIS.cars) &&
                comp(aunt.perfumes, ANALYSIS.perfumes)
            )
            .map(|aunt| aunt.number)
            .next()
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        let comp = |a: Option<usize>, b: Option<usize>, c: fn(&usize, &usize) -> bool| a.is_none() || c(&a.unwrap(), &b.unwrap());
        self.aunts
            .iter()
            .filter(|aunt| comp(aunt.children, ANALYSIS.children, usize::eq) &&
                comp(aunt.cats, ANALYSIS.cats, usize::gt) &&
                comp(aunt.samoyeds, ANALYSIS.samoyeds, usize::eq) &&
                comp(aunt.pomeranians, ANALYSIS.pomeranians, usize::lt) &&
                comp(aunt.akitas, ANALYSIS.akitas, usize::eq) &&
                comp(aunt.vizslas, ANALYSIS.vizslas, usize::eq) &&
                comp(aunt.goldfish, ANALYSIS.goldfish, usize::lt) &&
                comp(aunt.trees, ANALYSIS.trees, usize::gt) &&
                comp(aunt.cars, ANALYSIS.cars, usize::eq) &&
                comp(aunt.perfumes, ANALYSIS.perfumes, usize::eq)
            )
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
