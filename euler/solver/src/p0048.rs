pub fn p0048_solver() -> String {
    self_powers(1000).to_string()
}

fn self_powers(max: u64) -> u64 {
    const MOD: u64 = 10_000_000_000;
    let mut sum = 0;
    for n in 1..=max {
        let mut p = n;
        for _ in 1..n {
            p *= n;
            p %= MOD;
        }
        sum += p;
        sum %= MOD;
    }
    sum
}

#[test]
fn finds_last_ten_digits_of_self_power_sum() {
    assert_eq!(self_powers(10), 0_405_071_317);
}