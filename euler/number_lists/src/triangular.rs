use integers::Integer;
use crate::polygonal::{Polygonal, PolygonalIterator};

pub trait Triangular: Polygonal {
    fn triangular_root(self) -> Option<Self>;
    fn is_triangular(self) -> bool;
    fn as_triangular(self) -> Self;
}

impl<T: Integer> Triangular for T {
    fn triangular_root(self) -> Option<Self> {
        let a = self * Self::from(8) + Self::one();
        let b = a.sqrt();
        if b * b != a { return None; }
        let c = b - Self::one();
        if c % Self::two() == Self::zero() { Some(c / Self::two()) } else { None }
    }
    
    fn is_triangular(self) -> bool {
        self.triangular_root().is_some()
    }

    fn as_triangular(self) -> Self {
        (self * self + self) / Self::from(2)
    }
}

pub fn triangulars<T: Integer>() -> impl Iterator<Item=T> {
    PolygonalIterator::new(T::from(3))
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn finds_triangular_root_of_number() {
        assert_eq!(55.triangular_root(), Some(10));
        assert_eq!(56.triangular_root(), None);
    }

    #[test]
    fn checks_if_a_number_is_triangle() {
        assert!(55.is_triangular());
        assert!(!56.is_triangular());
    }

    #[test]
    fn generates_nth_triangle_numbers() {
        assert_eq!(10u32.as_triangular(), 55);
        assert_eq!(100.as_triangular(), 5050);
    }

    #[test]
    fn generates_list_of_triangle_numbers() {
        assert_eq!(triangulars().take(10).collect::<Vec<u64>>(), vec!(1, 3, 6, 10, 15, 21, 28, 36, 45, 55));
    }
}
