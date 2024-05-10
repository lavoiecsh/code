use std::cmp::Ordering;
use std::iter::Sum;
use num_integer::Integer;
use crate::libs::integers::integer_divisible::Divisible;

#[derive(PartialEq, Debug)]
pub enum PerfectVariant<T> {
    Perfect(T),
    Deficient(T, T),
    Abundant(T, T),
}

pub trait Perfect: Integer + Divisible + Sum + Copy {
    fn perfect_variant(&self) -> PerfectVariant<Self> {
        let divisor_sum: Self = self.proper_divisors().sum();
        match divisor_sum.cmp(self) {
            Ordering::Equal => PerfectVariant::Perfect(*self),
            Ordering::Greater => PerfectVariant::Abundant(*self, divisor_sum),
            Ordering::Less => PerfectVariant::Deficient(*self, divisor_sum),
        }
    }

    fn is_perfect(&self) -> bool {
        matches!(self.perfect_variant(), PerfectVariant::Perfect(_))
    }

    fn is_deficient(&self) -> bool {
        matches!(self.perfect_variant(), PerfectVariant::Deficient(_, _))
    }

    fn is_abundant(&self) -> bool {
        matches!(self.perfect_variant(), PerfectVariant::Abundant(_, _))
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
