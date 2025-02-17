use crate::solver::AdventSolver;
use std::collections::HashSet;

pub struct Advent2015Day22Solver {}

impl Advent2015Day22Solver {
    pub fn new(_input: &str) -> Self {
        // todo read actual input instead of hard-coding
        Self {}
    }
}

impl AdventSolver for Advent2015Day22Solver {
    fn solve_part1(&self) -> usize {
        let mut states: HashSet<State> = HashSet::new();
        states.insert(State {
            player: 50,
            mana: 500,
            boss: 58,
            damage: 9,
            shield: 0,
            poison: 0,
            recharge: 0,
            spent: 0,
            hard: false,
        });
        while states.iter().any(|s| !s.is_over()) {
            states = states.iter().flat_map(|s| s.next_states()).collect();
        }
        states
            .iter()
            .filter(|s| s.boss == 0)
            .map(|s| s.spent)
            .min()
            .unwrap()
    }

    fn solve_part2(&self) -> usize {
        let mut states: HashSet<State> = HashSet::new();
        states.insert(State {
            player: 50,
            mana: 500,
            boss: 58,
            damage: 9,
            shield: 0,
            poison: 0,
            recharge: 0,
            spent: 0,
            hard: true,
        });
        while states.iter().any(|s| !s.is_over()) {
            states = states.iter().flat_map(|s| s.next_states()).collect();
        }
        states
            .iter()
            .filter(|s| s.boss == 0)
            .map(|s| s.spent)
            .min()
            .unwrap()
    }
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct State {
    player: usize,
    mana: usize,
    shield: usize,
    recharge: usize,
    boss: usize,
    damage: usize,
    poison: usize,
    spent: usize,
    hard: bool,
}

trait Spell {
    fn can_cast(&self, state: &State) -> bool;
    fn cast(&self, state: &mut State);
}

struct MagicMissileSpell {}

struct DrainSpell {}

struct ShieldSpell {}

struct PoisonSpell {}

struct RechargeSpell {}

impl Spell for MagicMissileSpell {
    fn can_cast(&self, state: &State) -> bool {
        state.mana > 53
    }

    fn cast(&self, state: &mut State) {
        state.mana -= 53;
        state.spent += 53;
        state.boss = clamped_minus(state.boss, 4);
    }
}

impl Spell for DrainSpell {
    fn can_cast(&self, state: &State) -> bool {
        state.mana > 73
    }

    fn cast(&self, state: &mut State) {
        state.mana -= 73;
        state.spent += 73;
        state.player += 2;
        state.boss = clamped_minus(state.boss, 2);
    }
}

impl Spell for ShieldSpell {
    fn can_cast(&self, state: &State) -> bool {
        state.mana > 113 && state.shield == 0
    }

    fn cast(&self, state: &mut State) {
        state.mana -= 113;
        state.spent += 113;
        state.shield = 6;
    }
}

impl Spell for PoisonSpell {
    fn can_cast(&self, state: &State) -> bool {
        state.mana > 173 && state.poison == 0
    }

    fn cast(&self, state: &mut State) {
        state.mana -= 173;
        state.spent += 173;
        state.poison = 6;
    }
}

impl Spell for RechargeSpell {
    fn can_cast(&self, state: &State) -> bool {
        state.mana > 229 && state.recharge == 0
    }

    fn cast(&self, state: &mut State) {
        state.mana -= 229;
        state.spent += 229;
        state.recharge = 5;
    }
}

impl State {
    fn is_over(&self) -> bool {
        self.player == 0 || self.boss == 0
    }

    fn start_of_turn(&mut self) {
        if self.is_over() {
            return;
        }

        if self.hard {
            self.player -= 1;
            if self.is_over() {
                return;
            }
        }

        if self.shield > 0 {
            self.shield -= 1;
        }

        if self.poison > 0 {
            self.poison -= 1;
            self.boss = clamped_minus(self.boss, 3);
        }

        if self.recharge > 0 {
            self.recharge -= 1;
            self.mana += 101;
        }
    }

    fn boss_attack(&mut self) {
        if self.is_over() {
            return;
        }

        let damage = if self.shield == 0 {
            self.damage
        } else if 7 > self.damage {
            1
        } else {
            self.damage - 7
        };
        self.player = clamped_minus(self.player, damage);
    }

    fn next_states(&self) -> Vec<State> {
        if self.is_over() {
            return vec![*self];
        }

        let spells: Vec<Box<dyn Spell>> = vec![
            Box::new(MagicMissileSpell {}),
            Box::new(DrainSpell {}),
            Box::new(ShieldSpell {}),
            Box::new(PoisonSpell {}),
            Box::new(RechargeSpell {}),
        ];

        spells
            .iter()
            .filter(|spell| spell.can_cast(self))
            .map(|spell| {
                let mut clone = *self;
                spell.cast(&mut clone);
                clone.start_of_turn();
                clone.boss_attack();
                clone.start_of_turn();
                clone
            })
            .collect()
    }
}

fn clamped_minus(before: usize, reduction: usize) -> usize {
    before.saturating_sub(reduction)
}
