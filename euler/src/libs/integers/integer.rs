use std::fmt::Display;
use std::hash::Hash;

pub trait Integer: num_integer::Integer + Copy + Hash + Display + num_traits::NumAssignOps {

}

macro_rules! impl_integer {
    ($T:ty) => {
        impl Integer for $T {

        }
    }
}

impl_integer!(u64);
impl_integer!(u128);
impl_integer!(usize);
