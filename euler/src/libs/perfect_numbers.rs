use std::cmp::Ordering;
use std::iter::Sum;
use num_integer::Integer;
use crate::libs::integer_divisible::Divisible;

#[derive(PartialEq, Debug)]
pub enum PerfectVariant<T> {
    Perfect(T),
    Deficient(T, T),
    Abundant(T, T),
}

pub trait Perfect: Integer + Divisible + Sum + Copy {
    fn perfect_variant(&self) -> PerfectVariant<Self> {
        let divisor_sum: Self = self.proper_divisors().sum();
        match divisor_sum.cmp(&self) {
            Ordering::Equal => PerfectVariant::Perfect(*self),
            Ordering::Greater => PerfectVariant::Abundant(*self, divisor_sum),
            Ordering::Less => PerfectVariant::Deficient(*self, divisor_sum),
        }
    }

    fn is_perfect(&self) -> bool {
        if let PerfectVariant::Perfect(_) = self.perfect_variant() { true } else { false }
    }

    fn is_deficient(&self) -> bool {
        if let PerfectVariant::Deficient(_, _) = self.perfect_variant() { true } else { false }
    }

    fn is_abundant(&self) -> bool {
        if let PerfectVariant::Abundant(_, _) = self.perfect_variant() { true } else { false }
    }
}

macro_rules! impl_perfect {
    ($T:ty) => {
        impl Perfect for $T {

        }
    }
}

impl_perfect!(u64);
impl_perfect!(u128);
