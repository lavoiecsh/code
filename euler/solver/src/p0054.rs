use Ordering::*;
use std::cmp::Ordering;
use std::fs::read_to_string;

use itertools::Itertools;

use Hand::*;

pub fn p0054_solver() -> String {
    poker_hands(&read_to_string("input/0054_poker.txt").unwrap()).0.to_string()
}

fn poker_hands(input: &str) -> (usize, usize) {
    input.lines()
        .map(Game::from)
        .map(|g| g.player1.cmp(&g.player2))
        .fold((0, 0), |(a,b),c| match c { Less => (a,b+1), Greater => (a+1,b), Equal => (a,b) })
}

struct Game {
    player1: Hand,
    player2: Hand,
}

#[derive(Eq, PartialEq)]
enum Hand {
    HighCard(Vec<u8>),
    OnePair(u8, Vec<u8>),
    TwoPair(u8, u8, u8),
    ThreeOfAKind(u8, Vec<u8>),
    Straight(u8),
    Flush(Vec<u8>),
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush(u8),
    RoyalFlush,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (RoyalFlush, RoyalFlush) => Equal,
            (RoyalFlush, _) => Greater,
            (_, RoyalFlush) => Less,
            (StraightFlush(a), StraightFlush(b)) => a.cmp(b),
            (StraightFlush(_), _) => Greater,
            (_, StraightFlush(_)) => Less,
            (FourOfAKind(a1, a2), FourOfAKind(b1, b2)) => a1.cmp(b1).then_with(|| a2.cmp(b2)),
            (FourOfAKind(_, _), _) => Greater,
            (_, FourOfAKind(_, _)) => Less,
            (FullHouse(at, ap), FullHouse(bt, bp)) => at.cmp(bt).then_with(|| ap.cmp(bp)),
            (FullHouse(_, _), _) => Greater,
            (_, FullHouse(_, _)) => Less,
            (Flush(a), Flush(b)) => (0..5).map(|i| a[i].cmp(&b[i])).find(|&o| o != Equal).unwrap(),
            (Flush(_), _) => Greater,
            (_, Flush(_)) => Less,
            (Straight(a), Straight(b)) => a.cmp(b),
            (Straight(_), _) => Greater,
            (_, Straight(_)) => Less,
            (ThreeOfAKind(at, a), ThreeOfAKind(bt, b)) => at.cmp(bt).then_with(|| a[0].cmp(&b[0])).then_with(|| a[1].cmp(&b[1])),
            (ThreeOfAKind(_, _), _) => Greater,
            (_, ThreeOfAKind(_, _)) => Less,
            (TwoPair(ap1, ap2, a), TwoPair(bp1, bp2, b)) => ap1.cmp(bp1).then_with(|| ap2.cmp(bp2)).then_with(|| a.cmp(b)),
            (TwoPair(_, _, _), _) => Greater,
            (_, TwoPair(_, _, _)) => Less,
            (OnePair(ap, a), OnePair(bp, b)) => ap.cmp(bp).then_with(|| (0..3).map(|i| a[i].cmp(&b[i])).find(|&o| o != Equal).unwrap()),
            (OnePair(_, _), _) => Greater,
            (_, OnePair(_, _)) => Less,
            (HighCard(a), HighCard(b)) => (0..5).map(|i| a[i].cmp(&b[i])).find(|&o| o != Equal).unwrap(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct Card {
    suit: char,
    value: u8,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let cards: Vec<Card> = value.split(' ').map(Card::from).collect();
        Self {
            player1: Hand::from(cards.iter().take(5).cloned().collect::<Vec<Card>>()),
            player2: Hand::from(cards.iter().skip(5).take(5).cloned().collect::<Vec<Card>>()),
        }
    }
}

impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Self {
        let values = cards.iter().map(|c| c.value).sorted().rev().collect::<Vec<u8>>();
        let flush = cards.iter().map(|c| c.suit).all_equal();
        if values[0] == values[1] + 1 && values[1] == values[2] + 1 && values[2] == values[3] + 1 && values[3] == values[4] + 1 {
            return if flush {
                if values[0] == 14 {
                    RoyalFlush
                } else {
                    StraightFlush(values[0])
                }
            } else {
                Straight(values[0])
            }
        }
        if flush {
            return Flush(values);
        }
        let mut value_counts = [0; 15];
        for value in &values {
            value_counts[*value as usize] += 1;
        }
        if let Some((v, _)) = value_counts.iter().find_position(|&v| v == &4) {
            return FourOfAKind(v as u8, value_counts.iter().find_position(|&v| v == &1).unwrap().0 as u8);
        }
        let triple = value_counts.iter().find_position(|&v| v == &3);
        let pair = value_counts.iter().find_position(|&v| v == &2);
        if let Some((t, _)) = triple {
            return if let Some((p, _)) = pair {
                FullHouse(t as u8, p as u8)
            } else {
                ThreeOfAKind(t as u8, values.into_iter().filter(|&v| v != t as u8).collect())
            }
        }
        if let Some((p, _)) = pair {
            return if let Some((p2, _)) = value_counts.iter().enumerate().find(|&(i, v)| v == &2 && i != p) {
                TwoPair(p as u8, p2 as u8, values.into_iter().find(|&v| v != p as u8 && v != p2 as u8).unwrap())
            } else {
                OnePair(p as u8, values.into_iter().filter(|&v| v != p as u8).collect())
            }
        }
        HighCard(values)
    }
}

impl From<&str> for Card {
    fn from(text: &str) -> Self {
        let mut chars = text.chars();
        Self { value: text_to_card_value(chars.next().unwrap()), suit: chars.next().unwrap() }
    }
}

fn text_to_card_value(text: char) -> u8 {
    match text {
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
        _ => panic!("unknown card value {text}"),
    }
}

#[test]
fn counts_wins_by_both_players() {
    let hands = "\
5H 5C 6S 7S KD 2C 3S 8S 8D TD
5D 8C 9S JS AC 2C 5C 7D 8S QH
2D 9C AS AH AC 3D 6D 7D TD QD
4D 6S 9H QH QC 3D 6D 7H QD QS
2H 2D 4C 4D 4S 3C 3D 3S 9S 9D
";
    assert_eq!(poker_hands(hands), (3, 2));
}
