use std::fmt::{Debug, Display};
use std::hash::Hash;
use num_integer::Roots;
use num_traits::{NumAssignOps, NumOps};

pub trait Integer: num_integer::Integer + Copy + Hash + Display + NumAssignOps + NumOps + Roots + Debug {
    fn two() -> Self {
        Self::one() + Self::one()
    }
}

macro_rules! impl_integer {
    ($T:ty) => {
        impl Integer for $T {

        }
    }
}

impl_integer!(u32);
impl_integer!(u64);
impl_integer!(u128);
impl_integer!(usize);
