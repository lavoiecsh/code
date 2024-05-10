pub fn p0020_solver() -> String {
    factorial_digit_sum(100).to_string()
}

fn factorial_digit_sum(n: u128) -> u128 {
    let mut factorial = vec!(1);
    for product in 2..=n {
        let mut carry = 0;
        for f in &mut factorial {
            *f *= product;
            *f += carry % 10;
            carry /= 10;
            carry += *f / 10;
            *f %= 10;
        }
        while carry > 0 {
            factorial.push(carry % 10);
            carry /= 10;
        }
    }
    factorial.iter().sum()
}

#[test]
fn finds_sum_of_digits_of_factorial() {
    assert_eq!(factorial_digit_sum(10), 27);
}
