use crate::solver::AdventSolver;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

pub struct Advent2024Day22Solver {
    buyers: Vec<Buyer>,
}

impl Advent2024Day22Solver {
    pub fn new(input: &str) -> Self {
        Self {
            buyers: input.lines().map(Buyer::from).collect(),
        }
    }
}

impl AdventSolver for Advent2024Day22Solver {
    fn solve_part1(&self) -> usize {
        self.buyers
            .iter()
            .map(|buyer| buyer.secret_numbers().nth(2000).unwrap())
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut sequence_sums = HashMap::new();
        self.buyers
            .iter()
            .flat_map(|buyer| buyer.sequences())
            .for_each(|(sequence, value)| {
                sequence_sums
                    .entry(sequence)
                    .and_modify(|v| *v += value)
                    .or_insert(value);
            });
        sequence_sums.into_values().max().unwrap()
    }
}

struct Buyer {
    initial_secret_number: usize,
}

impl Buyer {
    fn new(initial_secret_number: usize) -> Self {
        Self {
            initial_secret_number,
        }
    }

    fn secret_numbers(&self) -> impl Iterator<Item = usize> {
        BuyerIter::new(self.initial_secret_number)
    }

    fn prices(&self) -> impl Iterator<Item = i8> {
        BuyerIter::new(self.initial_secret_number).map(|sn| (sn % 10) as i8)
    }

    fn sequences(&self) -> HashMap<(i8, i8, i8, i8), usize> {
        let mut sequences = HashMap::new();
        let mut prices = self.prices();
        let first_five = [
            prices.next().unwrap(),
            prices.next().unwrap(),
            prices.next().unwrap(),
            prices.next().unwrap(),
            prices.next().unwrap(),
        ];
        let mut sequence = VecDeque::from([
            first_five[1] - first_five[0],
            first_five[2] - first_five[1],
            first_five[3] - first_five[2],
            first_five[4] - first_five[3],
        ]);
        let mut last = first_five[4];
        sequences.insert(
            sequence.iter().cloned().collect_tuple().unwrap(),
            last as usize,
        );
        for _ in 4..2000 {
            let next = prices.next().unwrap();
            sequence.pop_front();
            sequence.push_back(next - last);
            sequences
                .entry(sequence.iter().cloned().collect_tuple().unwrap())
                .or_insert(next as usize);
            last = next;
        }
        sequences
    }
}

struct BuyerIter {
    secret_number: usize,
}

impl BuyerIter {
    fn new(secret_number: usize) -> Self {
        Self { secret_number }
    }
}

static PRUNE_CONSTANT: usize = 16777216;

impl Iterator for BuyerIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.secret_number;
        self.secret_number ^= self.secret_number << 6;
        self.secret_number %= PRUNE_CONSTANT;
        self.secret_number ^= self.secret_number >> 5;
        self.secret_number %= PRUNE_CONSTANT;
        self.secret_number ^= self.secret_number << 11;
        self.secret_number %= PRUNE_CONSTANT;
        Some(prev)
    }
}

impl From<&str> for Buyer {
    fn from(value: &str) -> Self {
        Self::new(value.parse().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn generates_secret_numbers_correctly() {
        let buyer = Buyer::new(123);
        assert_eq!(
            buyer.secret_numbers().take(11).collect_vec(),
            vec![
                123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
                7753432, 5908254,
            ]
        );
    }

    #[test]
    fn finds_simulated_secret_numbers() {
        let solver = Advent2024Day22Solver {
            buyers: vec![1, 10, 100, 2024].into_iter().map(Buyer::new).collect(),
        };
        assert_eq!(solver.solve_part1(), 37327623);
    }

    #[test]
    fn finds_sequence_correctly() {
        assert_eq!(Buyer::new(123).sequences().get(&(-1, -1, 0, 2)), Some(&6));
        let sequence = (-2, 1, -1, 3);
        assert_eq!(Buyer::new(1).sequences().get(&sequence), Some(&7));
        assert_eq!(Buyer::new(2).sequences().get(&sequence), Some(&7));
        assert_eq!(Buyer::new(3).sequences().get(&sequence), None);
        assert_eq!(Buyer::new(2024).sequences().get(&sequence), Some(&9));
    }

    #[test]
    fn finds_sequence_netting_most_bananas() {
        let solver = Advent2024Day22Solver {
            buyers: vec![1, 2, 3, 2024].into_iter().map(Buyer::new).collect(),
        };
        assert_eq!(solver.solve_part2(), 23);
    }
}
