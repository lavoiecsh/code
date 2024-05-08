use std::collections::VecDeque;
use crate::libs::integers::digits::digitable::Digits;
use crate::libs::integers::integer::Integer;

pub(crate) struct LeftTruncatingDigits<T: Integer> {
    base: T,
    digits: VecDeque<T>,
}

impl<T: Integer> LeftTruncatingDigits<T> {
    pub(crate) fn new(digits: &Digits<T>) -> Self {
        Self { base: digits.base(), digits: digits.iter().cloned().collect() }
    }
}

impl<T: Integer> Iterator for LeftTruncatingDigits<T> {
    type Item = Digits<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(_) = self.digits.pop_front() {
            if self.digits.is_empty() {
                None
            } else {
                Some(Digits::from_digits(self.base, self.digits.iter().cloned().collect()))
            }
        } else {
            None
        }
    }
}

pub(crate) struct RightTruncatingDigits<T: Integer> {
    base: T,
    digits: VecDeque<T>,
}

impl<T: Integer> RightTruncatingDigits<T> {
    pub(crate) fn new(digits: &Digits<T>) -> Self {
        Self { base: digits.base(), digits: digits.iter().cloned().collect() }
    }
}

impl<T: Integer> Iterator for RightTruncatingDigits<T> {
    type Item = Digits<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(_) = self.digits.pop_back() {
            if self.digits.is_empty() {
                None
            } else {
                Some(Digits::from_digits(self.base, self.digits.iter().cloned().collect()))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::libs::integers::digits::digitable::Digitable;
    use super::*;

    #[test]
    fn truncates_left_to_right() {
        let n = 12345u64;
        let d = n.as_decimal();
        let t = d.truncate_left();
        assert_eq!(t.map(|t2| t2.number()).collect::<Vec<u64>>(), vec!(2345, 345, 45, 5));
    }

    #[test]
    fn truncates_right_to_left() {
        let n = 12345u64;
        let d = n.as_decimal();
        let t = d.truncate_right();
        assert_eq!(t.map(|t2| t2.number()).collect::<Vec<u64>>(), vec!(1234, 123, 12, 1));
    }
}
