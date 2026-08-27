use crate::solver::AdventSolver;

const ZERO: i8 = '0' as i8;

pub struct Advent2019Day16Solver {
    data: Vec<i8>,
}

impl Advent2019Day16Solver {
    pub fn new(input: &str) -> Self {
        Self {
            data: input.chars().map(|c| c as i8 - ZERO).collect(),
        }
    }
}

fn undigit(iter: impl Iterator<Item = i8>) -> usize {
    iter.fold(0, |a, c| a * 10 + c as usize)
}

impl AdventSolver for Advent2019Day16Solver {
    fn solve_part1(&self) -> usize {
        let mut decoder = FFTDecoder::new(self.data.clone(), false);
        for _ in 0..100 {
            decoder.step();
        }
        undigit(decoder.values.iter().take(8).cloned())
    }

    fn solve_part2(&self) -> usize {
        let offset = undigit(self.data.iter().take(7).cloned());
        let mut data: Vec<i8> = vec![];
        for _ in 0..10000 {
            data.extend(&self.data);
        }
        let mut decoder = FFTDecoder::new(data.into_iter().skip(offset).collect(), true);
        for _ in 0..100 {
            decoder.step();
        }
        undigit(decoder.values.iter().take(8).cloned())
    }
}

struct FFTDecoder {
    values: Vec<i8>,
    summing: bool,
}

impl FFTDecoder {
    fn new(values: Vec<i8>, summing: bool) -> Self {
        Self {
            values,
            summing,
        }
    }

    fn step(&mut self) {
        if self.summing {
            let mut next_values = vec![];
            let mut running_sum = 0;
            for i in (0..self.values.len()).rev() {
                running_sum += self.values[i] as i32;
                next_values.push((running_sum % 10) as i8);
            }
            next_values.reverse();
            self.values = next_values;
        } else {
            self.values = (0..self.values.len())
                .map(|i| self.compute_value(i))
                .collect();
        }
    }

    fn compute_value(&self, index: usize) -> i8 {
        (self
            .values
            .iter()
            .zip(pattern(index))
            .filter(|&(_, p)| p != 0)
            .map(|(v, p)| (v * p) as i32 % 10)
            .sum::<i32>()
            .abs()
            % 10) as i8
    }
}

fn pattern(i: usize) -> impl Iterator<Item = i8> {
    PhasePatternIterator::new(i)
}

const PHASE_PATTERN: [i8; 4] = [0, 1, 0, -1];
struct PhasePatternIterator {
    index: usize,
    current_value: usize,
    current_count: usize,
}

impl PhasePatternIterator {
    fn new(index: usize) -> Self {
        Self {
            index,
            current_value: 0,
            current_count: 1,
        }
    }
}

impl Iterator for PhasePatternIterator {
    type Item = i8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_count > self.index {
            self.current_value += 1;
            self.current_value %= PHASE_PATTERN.len();
            self.current_count = 0;
        }

        self.current_count += 1;
        Some(PHASE_PATTERN[self.current_value])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn iterates_phase() {
        assert_eq!(
            vec![1, 0, -1, 0, 1, 0, -1, 0],
            pattern(0).take(8).collect_vec()
        );
        assert_eq!(
            vec![0, 1, 1, 0, 0, -1, -1, 0],
            pattern(1).take(8).collect_vec()
        );
    }

    #[test]
    fn computes_fft() {
        assert_eq!(
            24176176,
            Advent2019Day16Solver::new("80871224585914546619083218645595").solve_part1()
        );
        assert_eq!(
            73745418,
            Advent2019Day16Solver::new("19617804207202209144916044189917").solve_part1()
        );
        assert_eq!(
            52432133,
            Advent2019Day16Solver::new("69317163492948606335995924319873").solve_part1()
        );
    }

    #[test]
    fn computes_real_signal() {
        assert_eq!(
            84462026,
            Advent2019Day16Solver::new("03036732577212944063491565474664").solve_part2()
        );
        assert_eq!(
            78725270,
            Advent2019Day16Solver::new("02935109699940807407585447034323").solve_part2()
        );
        assert_eq!(
            53553731,
            Advent2019Day16Solver::new("03081770884921959731165446850517").solve_part2()
        );
    }
}
