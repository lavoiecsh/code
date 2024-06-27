pub mod factorized;
pub mod digits;
pub mod truncating;

use crate::digits::Digits;
use crate::factorized::Factorized;

pub trait Integer:
Copy +
std::hash::Hash +
std::fmt::Debug +
std::fmt::Display +
std::iter::Sum +
num_integer::Integer +
num_traits::NumAssignOps +
num_traits::NumOps +
num_integer::Roots +
{
    fn two() -> Self;
    fn ten() -> Self;
    fn from(n: u8) -> Self;

    fn as_binary(&self) -> Digits<Self> {
        Digits::from_number(Self::two(), *self)
    }

    fn as_decimal(&self) -> Digits<Self> {
        Digits::from_number(Self::from(10), *self)
    }

    fn factorize(&self) -> Factorized<Self> {
        Factorized::new(*self)
    }

    fn proper_divisors(&self) -> Vec<Self> {
        let n = *self;
        let mut divisors = vec!(Self::one());
        let max = n.sqrt();
        let mut d = Self::two();
        while d <= max {
            if n % d == Self::zero() {
                divisors.push(d);
                if d * d != n {
                    divisors.push(n / d);
                }
            }
            d += Self::one();
        }
        divisors.sort();
        divisors
    }
}

macro_rules! impl_integer {
    ($T:ty) => {
        impl Integer for $T {
            fn two() -> Self { 2 }
            fn ten() -> Self { 10 }
            fn from(n: u8) -> Self { n as Self }
        }
    }
}

impl_integer!(i8);
impl_integer!(i16);
impl_integer!(i32);
impl_integer!(i64);
impl_integer!(i128);
impl_integer!(isize);
impl_integer!(u8);
impl_integer!(u16);
impl_integer!(u32);
impl_integer!(u64);
impl_integer!(u128);
impl_integer!(usize);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_proper_divisors_of_a_number() {
        assert_eq!(12u32.proper_divisors(), vec!(1, 2, 3, 4, 6));
    }
}
