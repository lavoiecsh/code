pub fn p0017_solver() -> String {
    (1..=1000)
        .map(number_letter_counts)
        .sum::<usize>()
        .to_string()
}

#[allow(clippy::manual_range_patterns)] // expliciting 4, 5, or 6 instead of 4..=6
fn number_letter_counts(n: u64) -> usize {
    let thousands = n / 1000;
    let hundreds = (n / 100) % 10;
    let tens = (n / 10) % 10;
    let ones = n % 10;
    let mut count = 0;
    if thousands > 0 {
        count += 8 + digit_count(thousands);
    }
    if hundreds > 0 {
        count += 7 + digit_count(hundreds);
        if tens > 0 || ones > 0 {
            count += 3;
        }
    }
    count += match tens {
        0 => digit_count(ones),
        1 => match ones {
            0 => 3,
            1 | 2 => 6,
            5 | 6 => 7,
            3 | 4 | 8 | 9 => 8,
            7 => 9,
            _ => 0,
        },
        4 | 5 | 6 => 5 + digit_count(ones),
        2 | 3 | 8 | 9 => 6 + digit_count(ones),
        7 => 7 + digit_count(ones),
        _ => 0,
    };
    count
}

fn digit_count(n: u64) -> usize {
    match n {
        1 | 2 | 6 => 3,
        4 | 5 | 9 => 4,
        3 | 7 | 8 => 5,
        _ => 0,
    }
}

#[test]
fn counts_letters_in_written_version() {
    assert_eq!(number_letter_counts(1), 3);
    assert_eq!(number_letter_counts(2), 3);
    assert_eq!(number_letter_counts(3), 5);
    assert_eq!(number_letter_counts(4), 4);
    assert_eq!(number_letter_counts(5), 4);
    assert_eq!(number_letter_counts(342), 23);
    assert_eq!(number_letter_counts(115), 20);
}
