use integers::Integer;

pub fn p0047_solver() -> String {
    distinct_prime_factors(4).next().unwrap().to_string()
}

fn distinct_prime_factors(size: usize) -> impl Iterator<Item=usize> {
    DistinctPrimeFactorIterator::new(size)
}

struct DistinctPrimeFactorIterator {
    size: usize,
    numbers: Vec<usize>,
}

impl DistinctPrimeFactorIterator {
    fn new(size: usize) -> Self {
        Self {
            size,
            numbers: vec!(1),
        }
    }

    fn find_next(&self, last: usize) -> usize {
        (last + 1..).find(|n| n.factorize().distinct_count() == self.size).unwrap()
    }

    fn fill(&mut self) {
        let mut current = self.find_next(*self.numbers.last().unwrap());
        self.numbers = vec!(current);
        while self.numbers.len() != self.size {
            let next = self.find_next(current);
            if next != current + 1 {
                current = next;
                self.numbers = vec!(current);
            } else {
                current = next;
                self.numbers.push(current);
            }
        }
    }
}

impl Iterator for DistinctPrimeFactorIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.fill();
        Some(self.numbers[0])
    }
}

#[test]
fn finds_consecutive_numbers_with_distinct_prime_factors() {
    assert_eq!(distinct_prime_factors(2).next().unwrap(), 14);
    assert_eq!(distinct_prime_factors(3).next().unwrap(), 644);
}