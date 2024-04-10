pub fn p0025_solver() -> String {
    n_digit_fibonacci_number(1000).to_string()
}

fn n_digit_fibonacci_number(n: usize) -> usize {
    let mut stripped = 0u32;
    let mut a = 1u64;
    let mut b = 1u64;
    let mut i = 2;
    while b.ilog10() + stripped < n as u32 - 1 {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        if a > 1000000 {
            a /= 10;
            b /= 10;
            stripped += 1;
        }
    }
    i
}

#[test]
fn finds_index_of_first_fibonacci_number_with_number_of_digits() {
    assert_eq!(n_digit_fibonacci_number(2), 7);
    assert_eq!(n_digit_fibonacci_number(3), 12);
    assert_eq!(n_digit_fibonacci_number(4), 17);
    assert_eq!(n_digit_fibonacci_number(5), 21);
    assert_eq!(n_digit_fibonacci_number(6), 26);
}
