use integers::Integer;

pub trait Lychrel: Integer {
    /// Check whether an integer is a Lychrel number.
    /// Lychrel numbers have no end, only `iterations` are evaluated to determine.
    /// ```rust
    /// use number_theory::Lychrel;
    ///
    /// assert!(!349.is_lychrel(10));
    /// assert!(349.is_lychrel(2));
    /// assert!(196u128.is_lychrel(50));
    /// assert!(10677u128.is_lychrel(50));
    /// assert!(!10677u128.is_lychrel(55));
    /// ```
    fn is_lychrel(&self, iterations: usize) -> bool {
        let mut number = *self;
        for _ in 0..iterations {
            number += number.as_decimal().rev().number();
            if number.as_decimal().is_palindromic() { return false; }
        }
        true
    }
}

impl<T: Integer> Lychrel for T {}
