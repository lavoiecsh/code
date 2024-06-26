use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::ops::{MulAssign, RangeBounds};

use itertools::Itertools;

use crate::Integer;
use crate::truncating::{LeftTruncatingDigits, RightTruncatingDigits};

pub trait Digitable: Integer {
    fn as_binary(self) -> Digits<Self>;
    fn as_decimal(self) -> Digits<Self>;
}

#[derive(Clone)]
pub struct Digits<T: Integer> {
    base: T,
    digits: VecDeque<T>,
}

impl<T: Integer> Debug for Digits<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}b{}", self.number(), self.base))
    }
}

impl<T: Integer> Digits<T> {
    pub fn from_number(base: T, number: T) -> Self {
        let mut digits = VecDeque::new();
        let mut n = number;
        while n > T::zero() {
            digits.push_front(n % base);
            n /= base;
        }
        Self { base, digits }
    }

    pub(crate) fn from_digits(base: T, digits: VecDeque<T>) -> Self {
        Self { base, digits }
    }

    pub fn number(&self) -> T {
        self.digits.iter().cloned().reduce(|acc, cur| acc * self.base + cur).unwrap_or(T::zero())
    }
    
    pub fn number_in_base(&self, base: T) -> T {
        self.digits.iter().cloned().reduce(|acc, cur| acc * base + cur).unwrap_or(T::zero())
    }

    pub fn base(&self) -> T {
        self.base
    }

    pub fn rotate_left(&self) -> Self {
        let mut digits: VecDeque<T> = self.digits.iter().skip(1).cloned().collect();
        digits.push_back(self.digits[0]);
        Self { base: self.base, digits }
    }

    pub fn truncate_left(&self) -> LeftTruncatingDigits<T> {
        LeftTruncatingDigits::new(self)
    }

    pub fn truncate_right(&self) -> RightTruncatingDigits<T> {
        RightTruncatingDigits::new(self)
    }

    pub fn permutations(&self) -> Vec<Self> {
        self.digits.iter()
            .cloned()
            .permutations(self.digits.len())
            .map(|p| Self::from_digits(self.base, VecDeque::from(p)))
            .collect()
    }

    pub fn concatenate(&mut self, other: Self) {
        self.digits.extend(other.digits)
    }

    pub fn get(&self, index: usize) -> Option<T> {
        self.digits.get(index).copied()
    }
    
    pub fn set(&mut self, index: usize, new_value: T) {
        self.digits[index] = new_value;
    }

    pub fn range<R: RangeBounds<usize>>(&self, range: R) -> Self {
        Self::from_digits(self.base, self.digits.range(range).cloned().collect())
    }

    pub fn len(&self) -> usize {
        self.digits.len()
    }

    pub fn is_empty(&self) -> bool {
        self.digits.is_empty()
    }

    pub fn rev(&self) -> Self {
        Self::from_digits(self.base, self.digits.iter().cloned().rev().collect())
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.digits.iter()
    }

    pub fn is_palindromic(&self) -> bool {
        self.digits.iter().zip(self.digits.iter().rev()).all(|(f, b)| f == b)
    }

    pub fn is_pandigital(&self) -> bool {
        let unique_digits = self.digits.iter().cloned().collect::<HashSet<T>>();
        if unique_digits.len() != self.digits.len() { return false; }
        let mut d = T::one();
        while d < self.base {
            if !unique_digits.contains(&d) { return false; }
            d += T::one();
        }
        true
    }
    
    pub fn same_digits(&self, other: &Self) -> bool {
        self.digits.iter().cloned().sorted().collect::<Vec<T>>() == 
            other.digits.iter().cloned().sorted().collect::<Vec<T>>()
    }
}

impl <T: Integer> MulAssign<T> for Digits<T> {
    fn mul_assign(&mut self, rhs: T) {
        let mut tmp = T::zero();
        for i in (0..self.len()).rev() {
            tmp += self.digits[i] * rhs;
            self.digits[i] = tmp % self.base;
            tmp /= self.base;
        }
        while tmp > T::zero() {
            self.digits.push_front(tmp % self.base);
            tmp /= self.base;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_decimal_digits_for_a_number() {
        let n = 12345u64;
        let d = n.as_decimal();
        assert_eq!(d.number(), n);
        assert_eq!(d.base(), 10);
        assert_eq!(d.digits, vec!(1, 2, 3, 4, 5));
    }

    #[test]
    fn returns_binary_digits_for_a_number() {
        let n = 27u64;
        let b = n.as_binary();
        assert_eq!(b.number(), n);
        assert_eq!(b.base(), 2);
        assert_eq!(b.digits, vec!(1, 1, 0, 1, 1));
    }
}
