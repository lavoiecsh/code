pub fn p0016_solver() -> String {
    power_digit_sum(1000).to_string()
}

fn power_digit_sum(n: u32) -> u64 {
    let mut digits = vec!(1);
    for _ in 0..n {
        let mut carry = 0;
        for d in &mut digits {
            *d *= 2;
            *d += carry;
            carry = *d / 10;
            *d %= 10;
        }
        if carry != 0 {
            digits.push(carry);
        }
    }
    digits.into_iter().sum()
}

#[test]
fn computes_sum_of_digits_of_power_of_2() {
    assert_eq!(power_digit_sum(15), 26);
}
