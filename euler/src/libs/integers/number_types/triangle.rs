use num_integer::Roots;
use num_traits::One;
use crate::libs::integers::integer::Integer;

pub fn nth_triangle<T: Integer>(n: T) -> T {
    n * (n + T::one()) / T::two()
}

pub fn triangles<T: Integer>() -> impl Iterator<Item=T> {
    TriangleIterator { total: T::zero(), last: T::zero() }
}

struct TriangleIterator<T: Integer> {
    total: T,
    last: T,
}

impl<T: Integer> Iterator for TriangleIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.last += T::one();
        self.total += self.last;
        Some(self.total)
    }
}

pub trait Triangle: Integer {
    fn is_triangle(&self) -> bool;
}

macro_rules! impl_triangle {
    ($T:ty) => {
        impl Triangle for $T {
            fn is_triangle(&self) -> bool {
                let s = (self * Self::two()).sqrt();
                s * (s + Self::one()) / Self::two() == *self
            }
        }
    }
}

pub fn is_triangle<T: Triangle>(n: T) -> bool {
    n.is_triangle()
}

impl_triangle!(u32);
impl_triangle!(u64);
impl_triangle!(u128);
impl_triangle!(usize);

#[cfg(test)]
mod test {
    use itertools::Itertools;
    use super::*;

    #[test]
    fn generates_nth_triangle_numbers() {
        let first_10_triangle_numbers = (1u32..=10).map(nth_triangle).collect_vec();
        assert_eq!(first_10_triangle_numbers, vec!(1, 3, 6, 10, 15, 21, 28, 36, 45, 55));
    }

    #[test]
    fn checks_if_a_number_is_triangle() {
        assert_eq!(55u64.is_triangle(), true);
        assert_eq!(56u64.is_triangle(), false);
    }
}
