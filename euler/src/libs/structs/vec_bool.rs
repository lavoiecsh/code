pub fn to_vec_bool_lookup(numbers: &Vec<usize>) -> Vec<bool> {
    let max = *numbers.iter().max().unwrap();
    to_vec_bool_lookup_sized(numbers, max)
}

pub fn to_vec_bool_lookup_sized(numbers: &Vec<usize>, size: usize) -> Vec<bool> {
    let mut lookup = vec!();
    lookup.resize(size, false);
    numbers.iter().for_each(|&n| lookup[n] = true);
    lookup
}
