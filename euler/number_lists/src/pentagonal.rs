use integers::Integer;

use crate::polygonal::{Polygonal, PolygonalIterator};

pub trait Pentagonal: Integer {
    fn pentagonal_root(self) -> Option<Self>;
    fn is_pentagonal(self) -> bool;
    fn as_pentagonal(self) -> Self;
}

impl<T: Integer> Pentagonal for T {
    fn pentagonal_root(self) -> Option<Self> {
        self.polygonal_root(Self::from(5))
    }

    fn is_pentagonal(self) -> bool {
        self.is_polygonal(Self::from(5))
    }

    fn as_pentagonal(self) -> Self {
        self.as_polygonal(Self::from(5))
    }
}

pub fn pentagonals<T: Integer>() -> PolygonalIterator<T> {
    PolygonalIterator::new(T::from(5))
}

#[cfg(test)]
mod test {
    use std::ops::Not;

    use super::*;

    #[test]
    fn finds_pentagonal_root_of_number() {
        assert_eq!(22.pentagonal_root(), Some(4));
        assert_eq!(23.pentagonal_root(), None);
    }

    #[test]
    fn checks_if_a_number_is_pentagonal() {
        assert!(22.is_pentagonal());
        assert!(23.is_pentagonal().not());
    }

    #[test]
    fn generates_nth_pentagonal_numbers() {
        assert_eq!(4.as_pentagonal(), 22);
        assert_eq!(7.as_pentagonal(), 70);
    }

    #[test]
    fn generates_list_of_pentagonal_numbers() {
        assert_eq!(pentagonals().take(10).collect::<Vec<u64>>(), vec!(1, 5, 12, 22, 35, 51, 70, 92, 117, 145));
    }
}