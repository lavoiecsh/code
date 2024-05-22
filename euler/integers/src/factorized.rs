use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use itertools::Itertools;
use crate::Integer;

pub struct Factorized<T: Integer> {
    base: T,
    exponent: T,
    factors: HashMap<T, T>,
}

impl<T: Integer> Factorized<T> {
    pub(crate) fn new(base: T) -> Self {
        let mut n = base;
        let mut factors = HashMap::new();
        while n > T::one() {
            let mut f = T::one() + T::one();
            while !(n % f).is_zero() {
                f += T::one();
            }
            *factors.entry(f).or_insert(T::zero()) += T::one();
            n /= f;
        }
        Self { base, exponent: T::one(), factors }
    }

    pub fn pow(&self, exponent: T) -> Self {
        Self {
            base: self.base,
            exponent: self.exponent * exponent,
            factors: self.factors.iter().map(|(&f, &c)| (f, c * exponent)).collect(),
        }
    }
    
    pub fn largest(&self) -> Option<T> {
        self.factors.keys().max().cloned()
    }
    
    pub fn distinct_count(&self) -> usize {
        self.factors.keys().count()
    }
}

impl<T: Integer> PartialEq for Factorized<T> {
    fn eq(&self, other: &Self) -> bool {
        self.factors == other.factors
    }
}

impl<T: Integer> Debug for Factorized<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} ^ {}: {}", self.base, self.exponent,
                                 self.factors.iter()
                                     .sorted_by_key(|(&f, _)| f)
                                     .map(|(&f, &c)| format!("({f} ^ {c})"))
                                     .join(" * ")))
    }
}
