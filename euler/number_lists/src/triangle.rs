use integers::Integer;

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

pub fn is_triangle<T: Triangle>(n: T) -> bool {
    n.is_triangle()
}

impl<T: Integer> Triangle for T {
    fn is_triangle(&self) -> bool {
        let s = (*self * Self::two()).sqrt();
        s * (s + Self::one()) / Self::two() == *self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn checks_if_a_number_is_triangle() {
        assert!(55u64.is_triangle());
        assert!(!56u16.is_triangle());
    }

    #[test]
    fn generates_nth_triangle_numbers() {
        assert_eq!(nth_triangle(10), 55);
        assert_eq!(nth_triangle(100), 5050);
    }

    #[test]
    fn generates_list_of_triangle_numbers() {
        assert_eq!(triangles().take(10).collect::<Vec<u64>>(), vec!(1, 3, 6, 10, 15, 21, 28, 36, 45, 55));
    }
}
