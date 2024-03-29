pub fn p0002_solver() -> usize {
    even_fibonacci_numbers(4000000)
}

fn even_fibonacci_numbers(under: usize) -> usize {
    let mut a = 1;
    let mut b = 2;
    let mut sum = 0;
    while b < under {
        if b % 2 == 0 { sum += b; }
        let c = a + b;
        a = b;
        b = c;
    }
    sum
}

#[test]
fn computes_sum_of_even_fibonacci_numbers() {
    assert_eq!(even_fibonacci_numbers(100), 2 + 8 + 34);
}
