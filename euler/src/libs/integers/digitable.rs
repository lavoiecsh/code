use crate::libs::integers::integer::Integer;

pub trait Digitable: Integer {
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

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.digits.iter()
    }
}

macro_rules! impl_digitable {
    ($T:ty) => {
        impl Digitable for $T {
            fn as_decimal(self) -> Digits<$T> {
                Digits::new(self, 10)
            }
        }
    }
}

impl_digitable!(u64);
