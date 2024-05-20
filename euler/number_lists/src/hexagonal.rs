use integers::Integer;

use crate::polygonal::{Polygonal, PolygonalIterator};

pub trait Hexagonal: Integer {
    fn hexagonal_root(self) -> Option<Self>;
    fn is_hexagonal(self) -> bool;
    fn as_hexagonal(self) -> Self;
}

impl<T: Integer> Hexagonal for T {
    fn hexagonal_root(self) -> Option<Self> {
        self.polygonal_root(Self::from(6))
    }

    fn is_hexagonal(self) -> bool {
        self.is_polygonal(Self::from(6))
    }

    fn as_hexagonal(self) -> Self {
        self.as_polygonal(Self::from(6))
    }
}

pub fn hexagonals<T: Integer>() -> PolygonalIterator<T> {
    PolygonalIterator::new(T::from(6))
}

#[cfg(test)]
mod test {
    use std::ops::Not;

    use super::*;

    #[test]
    fn finds_hexagonal_root_of_number() {
        assert_eq!(28.hexagonal_root(), Some(4));
        assert_eq!(29.hexagonal_root(), None);
    }
    
    #[test]
    fn checks_if_a_number_is_hexagonal() {
        assert!(28.is_hexagonal());
        assert!(29.is_hexagonal().not());
    }
    
    #[test]
    fn generates_nth_hexagonal_numbers() {
        assert_eq!(4.as_hexagonal(), 28);
        assert_eq!(5.as_hexagonal(), 45);
    }
    
    #[test]
    fn generates_list_of_hexagonal_numbers() {
        assert_eq!(hexagonals().take(5).collect::<Vec<u32>>(), vec!(1, 6, 15, 28, 45));
    }
}