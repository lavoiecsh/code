use crate::libs::digitable::Digitable;

pub fn p0030_solver() -> String {
    digit_fifth_power(5).iter().sum::<u64>().to_string()
}

fn digit_fifth_power(power: u32) -> Vec<u64> {
    let mut numbers = vec!();
    for n in 10..10000000 {
        if n == n.as_decimal().iter().map(|d| d.pow(power)).sum() {
            numbers.push(n);
        }
    }
    numbers
}

#[test]
fn finds_sums_of_nth_powers_of_digits() {
    assert_eq!(digit_fifth_power(4), vec!(1634, 8208, 9474));
}
