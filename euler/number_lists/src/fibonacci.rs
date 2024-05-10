use integers::Integer;

pub fn fibonacci<T: Integer>() -> impl Iterator<Item=T> {
    FibonacciIterator::new()
}

struct FibonacciIterator<T: Integer> {
    a: T,
    b: T,
}

impl<T: Integer> FibonacciIterator<T> {
    fn new() -> Self {
        Self { a: T::one(), b: T::zero() }
    }
}

impl<T: Integer> Iterator for FibonacciIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.a + self.b;
        self.a = self.b;
        self.b = tmp;
        Some(self.b)
    }
}