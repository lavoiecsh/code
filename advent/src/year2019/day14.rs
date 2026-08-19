use crate::solver::AdventSolver;
use std::collections::HashMap;

pub struct Advent2019Day14Solver {
    nanofactory: Nanofactory,
}

impl Advent2019Day14Solver {
    pub fn new(input: &str) -> Self {
        let reaction_list = input
            .lines()
            .filter_map(|l| l.split_once(" => "))
            .map(|(from, to)| {
                let result = Element::from(to);
                (
                    result.name.clone(),
                    Reaction {
                        result,
                        requirements: from.split(", ").map(Element::from).collect(),
                    },
                )
            })
            .collect();
        Self {
            nanofactory: Nanofactory::new(reaction_list),
        }
    }
}

const TRILLION: u64 = 1_000_000_000_000;

impl AdventSolver for Advent2019Day14Solver {
    fn solve_part1(&self) -> usize {
        self.nanofactory.ores_required_for_fuel(1) as usize
    }

    fn solve_part2(&self) -> usize {
        let mut min_fuels = TRILLION / self.nanofactory.ores_required_for_fuel(1);
        let mut max_fuels = TRILLION;
        let mut fuels = (min_fuels + max_fuels) / 2;
        while fuels != min_fuels {
            let test = self.nanofactory.ores_required_for_fuel(fuels);
            if test > TRILLION {
                max_fuels = fuels;
            } else {
                min_fuels = fuels;
            }
            fuels = (min_fuels + max_fuels) / 2;
        }
        fuels as usize
    }
}

struct Nanofactory {
    reactions: HashMap<String, Reaction>,
}

#[derive(Clone)]
struct Reaction {
    result: Element,
    requirements: Vec<Element>,
}

#[derive(Clone)]
struct Element {
    count: u64,
    name: String,
}

impl Nanofactory {
    fn new(reactions: HashMap<String, Reaction>) -> Self {
        Self { reactions }
    }

    fn ores_required_for_fuel(&self, fuel_count: u64) -> u64 {
        let ore = String::from("ORE");
        let mut remaining_reactions = self.reactions.clone();
        let mut needed: HashMap<String, u64> = HashMap::new();
        needed.insert(String::from("FUEL"), fuel_count);
        while let Some(reactant) = needed.keys()
            .filter(|&k| k != &ore)
            .cloned()
            .find(|n| remaining_reactions.iter().all(|r| r.1.is_element_safe(n))) {
            let count = needed.remove(&reactant).unwrap();
            let reaction = remaining_reactions.remove(&reactant).unwrap();
            let multiple = count / reaction.result.count + (count % reaction.result.count).min(1);
            reaction.requirements.iter().for_each(|r| {
                *needed.entry(r.name.clone()).or_default() += r.count * multiple;
            });
        }
        assert_eq!(1, needed.len());
        *needed.get(&ore).unwrap()
    }
}

impl Reaction {
    fn is_element_safe(&self, element: &String) -> bool {
        self.requirements.iter().all(|r| &r.name != element)
    }
}

impl From<&str> for Element {
    fn from(value: &str) -> Self {
        let (c, e) = value.split_once(" ").unwrap();
        Self {
            count: c.parse().unwrap(),
            name: e.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
";
    const EXAMPLE_2: &str = "
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
";

    const EXAMPLE_3: &str = "
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
";
    const EXAMPLE_4: &str = "
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
";
    const EXAMPLE_5: &str = "
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
";

    #[test]
    fn counts_ores_required() {
        assert_eq!(31, Advent2019Day14Solver::new(EXAMPLE_1).solve_part1());
        assert_eq!(165, Advent2019Day14Solver::new(EXAMPLE_2).solve_part1());
        assert_eq!(13312, Advent2019Day14Solver::new(EXAMPLE_3).solve_part1());
        assert_eq!(180697, Advent2019Day14Solver::new(EXAMPLE_4).solve_part1());
        assert_eq!(2210736, Advent2019Day14Solver::new(EXAMPLE_5).solve_part1());
    }

    #[test]
    fn counts_maximum_fuel_for_trillion_ore() {
        assert_eq!(82892753, Advent2019Day14Solver::new(EXAMPLE_3).solve_part2());
        assert_eq!(5586022, Advent2019Day14Solver::new(EXAMPLE_4).solve_part2());
        assert_eq!(460664, Advent2019Day14Solver::new(EXAMPLE_5).solve_part2());
    }
}
