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

    fn as_binary(&self) -> Digits<Self> {
        Digits::from_number(Self::two(), *self)
    }

    fn as_decimal(&self) -> Digits<Self> {
        Digits::from_number(Self::ten(), *self)
    }

    fn proper_divisors(&self) -> Vec<Self> {
        proper_divisors(*self)
    }

    fn factorize(&self) -> Factorized<Self> {
        Factorized::new(*self)
    }
}

macro_rules! impl_integer {
    ($T:ty) => {
        impl Integer for $T {
            fn two() -> Self {
                2
            }

            fn ten() -> Self {
                10
            }
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

fn proper_divisors<T: Integer>(n: T) -> Vec<T> {
    let mut divisors = vec!(T::one());
    let max = n.sqrt();
    let mut d = T::two();
    while d <= max {
        if n % d == T::zero() {
            divisors.push(d);
            if d * d != n {
                divisors.push(n / d);
            }
        }
        d += T::one();
    }
    divisors.sort();
    divisors
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_proper_divisors_of_a_number() {
        assert_eq!(12u32.proper_divisors(), vec!(1, 2, 3, 4, 6));
    }
}
