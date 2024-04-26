use itertools::Itertools;
use crate::libs::integers::integer::Integer;

pub trait Digitable: Integer {
    fn as_binary(self) -> Digits<Self>;
    fn as_decimal(self) -> Digits<Self>;
}

pub struct Digits<T : Integer> {
    number: T,
    base: T,
    digits: Vec<T>,
}

impl <T : Integer> Digits<T> {
    fn new(number: T, base: T) -> Self {
        let mut digits = vec!();
        let mut n = number;
        while n > T::zero() {
            digits.push(n % base);
            n /= base;
        }
        digits.reverse();
        Self { number, base, digits }
    }

    pub fn number(&self) -> T {
        self.number
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

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.digits.iter()
    }

    pub fn into_iter(self) -> impl Iterator<Item=T> {
        self.digits.into_iter()
    }

    pub fn is_palindromic(&self) -> bool {
        self.digits.iter().zip(self.digits.iter().rev()).all(|(f,b)| f == b)
    }
}

macro_rules! impl_digitable {
    ($T:ty) => {
        impl Digitable for $T {
            fn as_binary(self) -> Digits<$T> {
                Digits::new(self, 2)
            }

            fn as_decimal(self) -> Digits<$T> {
                Digits::new(self, 10)
            }
        }
    }
}

impl_digitable!(u64);
impl_digitable!(u128);
impl_digitable!(usize);
