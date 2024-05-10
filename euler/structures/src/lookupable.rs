pub trait Lookupable {
    fn to_lookup(&self) -> Vec<bool>;
    fn to_sized_lookup(&self, size: usize) -> Vec<bool>;
}

impl Lookupable for Vec<usize> {
    fn to_lookup(&self) -> Vec<bool> {
        to_lookup(self)
    }

    fn to_sized_lookup(&self, size: usize) -> Vec<bool> {
        to_sized_lookup(self, size)
    }
}

impl Lookupable for [usize] {
    fn to_lookup(&self) -> Vec<bool> {
        to_lookup(self)
    }

    fn to_sized_lookup(&self, size: usize) -> Vec<bool> {
        to_sized_lookup(self, size)
    }
}

fn to_lookup(numbers: &[usize]) -> Vec<bool> {
    let max = *numbers.iter().max().unwrap();
    to_sized_lookup(numbers, max + 1)
}

fn to_sized_lookup(numbers: &[usize], size: usize) -> Vec<bool> {
    let mut lookup = vec!();
    lookup.resize(size, false);
    numbers.iter().for_each(|&n| lookup[n] = true);
    lookup
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn converts_a_list_of_numbers_to_a_lookup_vector() {
        assert_eq!(vec![2, 4, 5].to_lookup(), vec!(false, false, true, false, true, true));
        assert_eq!([2, 4, 5].to_lookup(), vec!(false, false, true, false, true, true));
    }
    
    #[test]
    fn converts_a_list_of_numbers_to_a_sized_lookup_vector() {
        assert_eq!(vec![1, 3, 6].to_sized_lookup(8), vec!(false, true, false, true, false, false, true, false));
        assert_eq!([1, 3, 6].to_sized_lookup(8), vec!(false, true, false, true, false, false, true, false));
    }
}