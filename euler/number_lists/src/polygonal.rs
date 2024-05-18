use integers::Integer;

pub(crate) trait Polygonal: Integer {
    fn polygonal_root(self, n: Self) -> Option<Self>;
    fn is_polygonal(self, n: Self) -> bool;
    fn as_polygonal(self, n: Self) -> Self;
}

impl<T: Integer> Polygonal for T {
    fn polygonal_root(self, n: Self) -> Option<Self> {
        let base = n - Self::two();
        let a = self * Self::from(8) * base + Self::one();
        let b = a.sqrt();
        if b * b != a { return None; }
        let c = b + Self::one();
        let div = base * Self::two();
        if c % div == Self::zero() { Some(c / div) } else { None }
    }

    fn is_polygonal(self, n: Self) -> bool {
        self.polygonal_root(n).is_some()
    }

    fn as_polygonal(self, n: Self) -> Self {
        let n1 = n - Self::two();
        let n2 = n1 - Self::two();
        (n1 * self * self - n2 * self) / Self::two()
    }
}

pub(crate) struct PolygonalIterator<T: Integer> {
    base: T,
    last: T,
    total: T,
}

impl<T: Integer> PolygonalIterator<T> {
    pub(crate) fn new(n: T) -> Self {
        Self { base: n - T::two(), last: T::zero(), total: T::zero() }
    }
}

impl<T: Integer> Iterator for PolygonalIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.last += if self.last == T::zero() { T::one() } else { self.base };
        self.total += self.last;
        Some(self.total)
    }
}
