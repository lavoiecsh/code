use itertools::Itertools;
use num_integer::{Integer, Roots};

pub trait Divisible: Integer {
    fn proper_divisors(&self) -> impl Iterator<Item=Self>;
}

macro_rules! impl_divisible {
    ($T:ty) => {
        impl Divisible for $T {
            fn proper_divisors(&self) -> impl Iterator<Item=Self> {
                let mut divisors = vec!(1);
                for d in 2..=self.sqrt() {
                    if self % d != 0 { continue; }
                    divisors.push(d);
                    if d * d != *self {
                        divisors.push(self / d);
                    }
                }
                divisors.into_iter().sorted()
            }
        }
    }
}

impl_divisible!(u64);
impl_divisible!(u128);

#[test]
fn computes_proper_divisors_of_a_number() {
    assert_eq!(12u64.proper_divisors().collect_vec(), vec!(1, 2, 3, 4, 6));
    assert_eq!(25u64.proper_divisors().collect_vec(), vec!(1, 5));
    assert_eq!(37u64.proper_divisors().collect_vec(), vec!(1));
}
