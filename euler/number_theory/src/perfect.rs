use std::cmp::Ordering;

use integers::Integer;
use PerfectVariant::*;

#[derive(PartialEq, Debug)]
pub enum PerfectVariant<T> {
    Perfect(T),
    Deficient(T, T),
    Abundant(T, T),
}

pub trait Perfect: Integer {
    fn perfect_variant(&self) -> PerfectVariant<Self> {
        let divisor_sum: Self = self.proper_divisors().into_iter().sum();
        match divisor_sum.cmp(self) {
            Ordering::Equal => Perfect(*self),
            Ordering::Greater => Abundant(*self, divisor_sum),
            Ordering::Less => Deficient(*self, divisor_sum),
        }
    }

    fn is_perfect(&self) -> bool {
        matches!(self.perfect_variant(), Perfect(_))
    }

    fn is_deficient(&self) -> bool {
        matches!(self.perfect_variant(), Deficient(_, _))
    }

    fn is_abundant(&self) -> bool {
        matches!(self.perfect_variant(), Abundant(_, _))
    }

    fn amicable_pair(&self) -> Option<Self> {
        match self.perfect_variant() {
            Perfect(_) => None,
            Abundant(_, first) => match first.perfect_variant() {
                Perfect(_) => None,
                Abundant(_, _) => None,
                Deficient(_, second) => if second == *self { Some(first) } else { None },
            },
            Deficient(_, first) => match first.perfect_variant() {
                Perfect(_) => None,
                Deficient(_, _) => None,
                Abundant(_, second) => if second == *self { Some(first) } else { None },
            },
        }
    }
}

impl<T: Integer> Perfect for T {}