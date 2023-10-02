use crate::solver::AdventSolver;

pub struct Advent2015Day21Solver {
    load_outs: Vec<LoadOut>
}

impl Advent2015Day21Solver {
    pub fn new(_input: String) -> Self {
        // todo read actual input instead of hard-coding
        let mut load_outs = Vec::new();
        for weapon in WEAPONS {
            load_outs.push(LoadOut { weapon: weapon.clone(), armor: None, ring1: None, ring2: None });
            for armor in ARMORS {
                load_outs.push(LoadOut { weapon, armor: Some(armor), ring1: None, ring2: None });
                for ring1 in RINGS {
                    load_outs.push(LoadOut { weapon, armor: Some(armor), ring1: Some(ring1), ring2: None });
                    for ring2 in RINGS {
                        if ring1.cost == ring2.cost {
                            continue;
                        }
                        load_outs.push(LoadOut { weapon, armor: Some(armor), ring1: Some(ring1), ring2: Some(ring2) });
                    }
                }
            }
            for ring1 in RINGS {
                load_outs.push(LoadOut { weapon, armor: None, ring1: Some(ring1), ring2: None });
                for ring2 in RINGS {
                    if ring1.cost == ring2.cost {
                        continue;
                    }
                    load_outs.push(LoadOut { weapon, armor: None, ring1: Some(ring1), ring2: Some(ring2) });
                }
            }
        }
        Self { load_outs }
    }
}

impl AdventSolver for Advent2015Day21Solver {
    fn day(&self) -> usize { 21 }
    fn year(&self) -> usize { 2015 }

    fn solve_part1(&self) -> usize {
        self.load_outs
            .iter()
            .filter(|lo| {
                let mut player = PLAYER;
                player.damage = lo.damage();
                player.armor = lo.armor();
                let mut boss = BOSS;
                fight(&mut player, &mut boss);
                player.hit_points > 0
            })
            .map(|lo| lo.cost())
            .min()
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        self.load_outs
            .iter()
            .filter(|lo| {
                let mut player = PLAYER;
                player.damage = lo.damage();
                player.armor = lo.armor();
                let mut boss = BOSS;
                fight(&mut player, &mut boss);
                player.hit_points == 0
            })
            .map(|lo| lo.cost())
            .max()
            .unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
struct Character {
    hit_points: usize,
    damage: usize,
    armor: usize,
}

#[derive(Debug, Copy, Clone)]
struct Equipment {
    cost: usize,
    damage: usize,
    armor: usize,
}

#[derive(Debug, Copy, Clone)]
struct LoadOut {
    weapon: Equipment,
    armor: Option<Equipment>,
    ring1: Option<Equipment>,
    ring2: Option<Equipment>,
}

impl LoadOut {
    fn cost(self: &Self) -> usize {
        self.calc(|e| e.cost)
    }

    fn damage(self: &Self) -> usize {
        self.calc(|e| e.damage)
    }

    fn armor(self: &Self) -> usize {
        self.calc(|e| e.armor)
    }

    fn calc(self: &Self, f: fn(&Equipment) -> usize) -> usize {
        f(&self.weapon) +
            self.armor.as_ref().map(f).unwrap_or(0) +
            self.ring1.as_ref().map(f).unwrap_or(0) +
            self.ring2.as_ref().map(f).unwrap_or(0)
    }
}
const PLAYER: Character = Character { hit_points: 100, damage: 0, armor: 0 };

const BOSS: Character = Character { hit_points: 109, damage: 8, armor: 2 };
const WEAPONS: [Equipment; 5] = [
    Equipment { cost: 8, damage: 4, armor: 0 },
    Equipment { cost: 10, damage: 5, armor: 0 },
    Equipment { cost: 25, damage: 6, armor: 0 },
    Equipment { cost: 40, damage: 7, armor: 0 },
    Equipment { cost: 74, damage: 8, armor: 0 },
];
const ARMORS: [Equipment; 5] = [
    Equipment { cost: 13, damage: 0, armor: 1 },
    Equipment { cost: 31, damage: 0, armor: 2 },
    Equipment { cost: 53, damage: 0, armor: 3 },
    Equipment { cost: 75, damage: 0, armor: 4 },
    Equipment { cost: 102, damage: 0, armor: 5 },
];

const RINGS: [Equipment; 6] = [
    Equipment { cost: 25, damage: 1, armor: 0 },
    Equipment { cost: 50, damage: 2, armor: 0 },
    Equipment { cost: 100, damage: 3, armor: 0 },
    Equipment { cost: 20, damage: 0, armor: 1 },
    Equipment { cost: 40, damage: 0, armor: 2 },
    Equipment { cost: 80, damage: 0, armor: 3 },
];

fn fight(player: &mut Character, boss: &mut Character) {
    while player.hit_points > 0 && boss.hit_points > 0 {
        attack(player, boss);
        if boss.hit_points == 0 {
            break;
        }
        attack(boss, player);
    }
}

fn attack(attacker: &Character, defender: &mut Character) {
    let damage = if defender.armor >= attacker.damage { 1 } else { attacker.damage - defender.armor };
    if damage > defender.hit_points {
        defender.hit_points = 0;
    } else {
        defender.hit_points -= damage;
    }
}
