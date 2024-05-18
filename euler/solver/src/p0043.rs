use integers::digits::Digits;
use integers::Integer;

pub fn p0043_solver() -> String {
    sub_string_divisibility().sum::<u64>().to_string()
}

fn sub_string_divisibility() -> impl Iterator<Item=u64> {
    1234567890u64.as_decimal()
        .permutations()
        .into_iter()
        .filter(is_sub_string_divisible)
        .map(|d| d.number())
}

fn is_sub_string_divisible(digits: &Digits<u64>) -> bool {
    digits.get(0).unwrap() != 0 &&
        digits.slice(1..=3).unwrap().number() % 2 == 0 &&
        digits.slice(2..=4).unwrap().number() % 3 == 0 &&
        digits.slice(3..=5).unwrap().number() % 5 == 0 &&
        digits.slice(4..=6).unwrap().number() % 7 == 0 &&
        digits.slice(5..=7).unwrap().number() % 11 == 0 &&
        digits.slice(6..=8).unwrap().number() % 13 == 0 &&
        digits.slice(7..=9).unwrap().number() % 17 == 0
}
