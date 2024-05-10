pub fn p0028_solver() -> String {
    number_spiral_diagonals(1001).to_string()
}

fn number_spiral_diagonals(size: usize) -> u64 {
    let mut current_size = 0;
    let mut current_value = 1u64;
    let mut sum = 1;
    while current_size < size - 1 {
        current_size += 2;
        for _ in 0..4 {
            current_value += current_size as u64;
            sum += current_value;
        }
    }
    sum
}

#[test]
fn computes_sum_of_numbers_on_diagonals_of_spiral() {
    assert_eq!(number_spiral_diagonals(5), 101);
}
