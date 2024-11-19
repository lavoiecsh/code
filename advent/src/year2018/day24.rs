use crate::solver::AdventSolver;
use itertools::Itertools;
use regex::{Match, Regex};

pub struct Advent2018Day24Solver {
    units: Vec<Unit>,
}

impl Advent2018Day24Solver {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"(\d+) units each with (\d+) hit points (.*)with an attack that does (\d+) (\w+) damage at initiative (\d+)").unwrap();
        let parse = |c: Option<Match>| c.unwrap().as_str().parse().unwrap();
        let mut immune_system = true;
        let mut units = Vec::new();
        for line in input.lines() {
            if let Some(cap) = re.captures(line) {
                let wi = cap
                    .get(3)
                    .unwrap()
                    .as_str()
                    .split(';')
                    .filter(|s| !s.trim().is_empty())
                    .collect_vec();
                units.push(Unit {
                    immune_system,
                    count: parse(cap.get(1)),
                    hit_points: parse(cap.get(2)),
                    weaknesses: wi
                        .iter()
                        .find(|wi2| wi2.chars().nth(1).unwrap() == 'w')
                        .unwrap_or(&"")
                        .split(' ')
                        .map(DamageType::try_from)
                        .filter_map(Result::ok)
                        .collect(),
                    immunities: wi
                        .iter()
                        .find(|wi2| wi2.chars().nth(1).unwrap() == 'i')
                        .unwrap_or(&"")
                        .split(' ')
                        .map(DamageType::try_from)
                        .filter_map(Result::ok)
                        .collect(),
                    attack: parse(cap.get(4)),
                    attack_type: DamageType::try_from(cap.get(5).unwrap().as_str()).unwrap(),
                    initiative: parse(cap.get(6)),
                });
            } else {
                match line {
                    "Immune System:" => immune_system = true,
                    "Infection:" => immune_system = false,
                    _ => {}
                }
            }
        }
        Self { units }
    }

    fn run_single(&self) -> Combat {
        let mut combat = Combat::new(&self.units, 0);
        combat.run();
        combat
    }

    fn find_boost(&self) -> (Combat, usize) {
        for boost in 0.. {
            let mut combat = Combat::new(&self.units, boost);
            combat.run();
            if combat.immune_system_won() {
                return (combat, boost);
            }
        }
        unreachable!("no solution found")
    }
}

impl AdventSolver for Advent2018Day24Solver {
    fn solve_part1(&self) -> usize {
        self.run_single().unit_count()
    }

    fn solve_part2(&self) -> usize {
        self.find_boost().0.unit_count()
    }
}

#[derive(Debug)]
struct Combat {
    units: Vec<Unit>,
}

impl Combat {
    fn new(units: &[Unit], boost: usize) -> Self {
        Self {
            units: units.iter().map(|u| u.boost(boost)).collect(),
        }
    }

    fn is_done(&self) -> bool {
        self.units.iter().map(|u| u.immune_system).all_equal()
    }

    fn immune_system_won(&self) -> bool {
        self.units.iter().all(|u| u.immune_system)
    }

    fn unit_count(&self) -> usize {
        self.units.iter().map(|u| u.count).sum()
    }

    fn run(&mut self) {
        let mut prev = 0;
        while !self.is_done() && self.unit_count() != prev {
            prev = self.unit_count();
            self.fight();
        }
    }

    fn fight(&mut self) {
        let mut selected_attacks = vec![];
        for attacker in self.units.iter().sorted_by(|a, b| {
            b.effective_power()
                .cmp(&a.effective_power())
                .then(b.initiative.cmp(&a.initiative))
        }) {
            if let Some(target) = self.select_target(attacker, &selected_attacks) {
                selected_attacks.push((attacker.initiative, target));
            }
        }
        for &(a, d) in selected_attacks.iter().sorted_by_key(|(a, _)| a).rev() {
            self.attack(a, d);
        }
    }

    fn select_target(&self, attacker: &Unit, selected_attacks: &[(usize, usize)]) -> Option<usize> {
        self.units
            .iter()
            .filter(|u| {
                u.immune_system != attacker.immune_system
                    && selected_attacks.iter().all(|&(_, d)| u.initiative != d)
            })
            .map(|u| (attacker.damage_to(u), u.effective_power(), u.initiative))
            .filter(|&(d, _, _)| d != 0)
            .max_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)))
            .map(|(_, _, i)| i)
    }

    fn attack(&mut self, attacker_initiative: usize, defender_initiative: usize) {
        let optional_attacker = self
            .units
            .iter()
            .find(|u| u.initiative == attacker_initiative);
        if optional_attacker.is_none() {
            return;
        }
        let attacker = optional_attacker.unwrap();
        let defender_index = self
            .units
            .iter()
            .position(|u| u.initiative == defender_initiative)
            .unwrap();
        let damage = attacker.damage_to(&self.units[defender_index]);
        self.units[defender_index].take_damage(damage);
        if self.units[defender_index].count == 0 {
            self.units.swap_remove(defender_index);
        }
    }
}

#[derive(Clone, Debug)]
struct Unit {
    immune_system: bool,
    count: usize,
    hit_points: usize,
    weaknesses: Vec<DamageType>,
    immunities: Vec<DamageType>,
    attack: usize,
    attack_type: DamageType,
    initiative: usize,
}

impl Unit {
    fn boost(&self, boost: usize) -> Self {
        let mut clone = self.clone();
        if clone.immune_system {
            clone.attack += boost;
        }
        clone
    }

    fn effective_power(&self) -> usize {
        self.count * self.attack
    }

    fn damage_to(&self, defender: &Self) -> usize {
        if defender.weaknesses.contains(&self.attack_type) {
            self.effective_power() * 2
        } else if defender.immunities.contains(&self.attack_type) {
            0
        } else {
            self.effective_power()
        }
    }

    fn take_damage(&mut self, damage: usize) {
        let units_lost = damage / self.hit_points;
        if units_lost > self.count {
            self.count = 0;
        } else {
            self.count -= units_lost;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DamageType {
    Cold,
    Fire,
    Slashing,
    Bludgeoning,
    Radiation,
}

impl TryFrom<&str> for DamageType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.chars().nth(0).unwrap_or('x') {
            'c' => Ok(DamageType::Cold),
            'f' => Ok(DamageType::Fire),
            's' => Ok(DamageType::Slashing),
            'b' => Ok(DamageType::Bludgeoning),
            'r' => Ok(DamageType::Radiation),
            _ => Err(format!("Unknown damage type: {}", value)),
        }
    }
}

// impl From<&str> for DamageType {
//     fn from(value: &str) -> Self {
//         match value.chars().nth(0).unwrap() {
//             'c' => DamageType::Cold,
//             'f' => DamageType::Fire,
//             's' => DamageType::Slashing,
//             'b' => DamageType::Bludgeoning,
//             'r' => DamageType::Radiation,
//             _ => unreachable!("invalid damage type {value}")
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
";

    #[test]
    fn finds_winning_side() {
        let solver = Advent2018Day24Solver::new(EXAMPLE);
        let combat = solver.run_single();
        let unit_1 = combat.units.iter().find(|u| u.initiative == 1).unwrap();
        let unit_4 = combat.units.iter().find(|u| u.initiative == 4).unwrap();
        assert!(!unit_1.immune_system);
        assert!(!unit_4.immune_system);
        assert_eq!(unit_1.count, 782);
        assert_eq!(unit_4.count, 4434);
    }

    #[test]
    fn finds_smallest_boost() {
        let solver = Advent2018Day24Solver::new(EXAMPLE);
        let (combat, boost) = solver.find_boost();
        assert!(combat.immune_system_won());
        assert_eq!(boost, 1570);
        assert_eq!(combat.units.len(), 1);
        assert_eq!(combat.units[0].count, 51);
        assert_eq!(combat.units[0].initiative, 3);
    }
}
