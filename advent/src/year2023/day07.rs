use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

use itertools::Itertools;

use crate::solver::AdventSolver;

pub struct Advent2023Day07Solver {
    hands: Vec<Hand>,
}

impl Advent2023Day07Solver {
    pub fn new(input: String) -> Self {
        Self { hands: input.lines().map(Hand::from).collect() }
    }
}

impl AdventSolver for Advent2023Day07Solver {
    fn solve_part1(&self) -> usize {
        self.hands.iter()
            .sorted_by_key(|h| h.ordering_value())
            .enumerate()
            .map(|(rank, hand)| hand.bid * (rank + 1))
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.hands.iter()
            .map(|h| h.to_joker_version())
            .sorted_by_key(|h| h.ordering_value())
            .enumerate()
            .map(|(rank, hand)| hand.bid * (rank + 1))
            .sum()
    }
}

type Card = char;

struct Hand {
    cards: Vec<Card>,
    bid: usize,
    value: HandValue,
}

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}  {:3} -> {}", self.cards.iter().join(""), self.bid, self.ordering_value()))
    }
}

impl Hand {
    fn ordering_value(&self) -> usize {
        self.value.value() * 10000000000 +
            to_card_value(self.cards[0]) * 100000000 +
            to_card_value(self.cards[1]) * 1000000 +
            to_card_value(self.cards[2]) * 10000 +
            to_card_value(self.cards[3]) * 100 +
            to_card_value(self.cards[4]) * 1
    }

    fn to_joker_version(&self) -> Self {
        let cards: Vec<Card> = self.cards.iter().map(|&c| if c == 'J' { 'X' } else { c }).collect();
        let value = "23456789TQKA".chars()
            .map(|r| cards.iter().map(|&c| if c == 'X' { r } else { c }).collect::<Vec<Card>>())
            .map(|h| HandValue::from(&h))
            .max_by_key(|h| h.value())
            .unwrap();
        Self {
            cards,
            bid: self.bid,
            value,
        }
    }
}

#[derive(Copy, Clone)]
enum HandValue {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandValue {
    fn value(&self) -> usize {
        *self as usize
    }
}

impl From<&Vec<Card>> for HandValue {
    fn from(value: &Vec<Card>) -> Self {
        let counts: Vec<(Card, usize)> = value.iter().cloned()
            .unique()
            .map(|card| (card, value.iter().filter(|&&v| card == v).count()))
            .sorted_by(|l,r| r.1.cmp(&l.1).then(card_compare(r.0, l.0)))
            .collect();
        match (counts.len(), counts[0].1) {
            (1, 5) => Self::FiveOfAKind,
            (2, 4) => Self::FourOfAKind,
            (2, 3) => Self::FullHouse,
            (3, 3) => Self::ThreeOfAKind,
            (3, 2) => Self::TwoPair,
            (4, 2) => Self::OnePair,
            (5, 1) => Self::HighCard,
            (l, c) => panic!("impossible hand {}: {l}, {c}", value.iter().join("")),
        }
    }
}

fn card_compare(left: Card, right: Card) -> Ordering {
    to_card_value(left).cmp(&to_card_value(right))
}

fn to_card_value(card: Card) -> usize {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        'X' => 1,
        c => panic!("unknown card {c}"),
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        let mut s = value.split(" ");
        let cards = s.next().unwrap().chars().collect();
        Self {
            value: HandValue::from(&cards),
            cards,
            bid: s.next().unwrap().parse().unwrap(),
        }
    }
}

#[cfg(test)]
fn test_solver_1() -> Advent2023Day07Solver {
    Advent2023Day07Solver::new(String::from("\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"))
}

#[test]
fn total_winnings() {
    let solver = test_solver_1();
    let ordered_hands: Vec<&Hand> = solver.hands.iter().sorted_by_key(|h| h.ordering_value()).collect();
    assert_eq!(ordered_hands.iter().map(|h| h.bid).collect::<Vec<usize>>(), vec!(765, 220, 28, 684, 483));
    assert_eq!(solver.solve_part1(), 6440);
}

#[test]
fn total_winnings_with_jokers() {
    let solver = test_solver_1();
    assert_eq!(solver.solve_part2(), 5905);
}
