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
        digits.range(1..=3).number() % 2 == 0 &&
        digits.range(2..=4).number() % 3 == 0 &&
        digits.range(3..=5).number() % 5 == 0 &&
        digits.range(4..=6).number() % 7 == 0 &&
        digits.range(5..=7).number() % 11 == 0 &&
        digits.range(6..=8).number() % 13 == 0 &&
        digits.range(7..=9).number() % 17 == 0
}
