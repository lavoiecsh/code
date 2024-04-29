use itertools::Itertools;
use crate::libs::integers::digits::truncatable::{LeftTruncatingDigits, RightTruncatingDigits};
use crate::libs::integers::integer::Integer;

pub trait Digitable: Integer {
    fn as_binary(self) -> Digits<Self>;
    fn as_decimal(self) -> Digits<Self>;
}

pub struct Digits<T: Integer> {
    number: T,
    base: T,
    digits: Vec<T>,
}

impl<T: Integer> Digits<T> {
    fn from_number(base: T, number: T) -> Self {
        let mut digits = vec!();
        let mut n = number;
        while n > T::zero() {
            digits.push(n % base);
            n /= base;
        }
        digits.reverse();
        Self { number, base, digits }
    }

    pub fn from_digits(base: T, digits: Vec<T>) -> Self {
        Self {
            number: digits.iter().cloned().reduce(|acc, cur| acc * base + cur).unwrap_or(T::zero()),
            base,
            digits,
        }
    }

    pub fn number(&self) -> T {
        self.number
    }

    pub fn base(&self) -> T {
        self.base
    }

    pub fn rotate_left(&self) -> Self {
        let mut digits = self.digits.iter().skip(1).cloned().collect_vec();
        digits.push(self.digits[0]);
        Self {
            number: digits.iter().cloned().reduce(|acc, cur| acc * self.base + cur).unwrap(),
            base: self.base,
            digits,
        }
    }

    pub fn truncate_left(&self) -> LeftTruncatingDigits<T> {
        LeftTruncatingDigits::new(&self)
    }

    pub fn truncate_right(&self) -> RightTruncatingDigits<T> {
        RightTruncatingDigits::new(&self)
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.digits.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item=T> {
        self.digits.into_iter()
    }

    pub fn is_palindromic(&self) -> bool {
        self.digits.iter().zip(self.digits.iter().rev()).all(|(f, b)| f == b)
    }
}

macro_rules! impl_digitable {
    ($T:ty) => {
        impl Digitable for $T {
            fn as_binary(self) -> Digits<$T> {
                Digits::from_number(2, self)
            }

            fn as_decimal(self) -> Digits<$T> {
                Digits::from_number(10, self)
            }
        }
    }
}

impl_digitable!(u64);
impl_digitable!(u128);
impl_digitable!(usize);

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
