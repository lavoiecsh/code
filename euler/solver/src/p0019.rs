pub fn p0019_solver() -> String {
    counting_sundays().to_string()
}

fn counting_sundays() -> usize {
    let mut current = 1;
    // get to January 1st 1901
    for m in 0..12 {
        current += month_length(m, 1900);
    }
    current %= 7;
    let mut sunday_count = if current == 0 { 1 } else { 0 };
    for y in 1901..=2000 {
        for m in 0..12 {
            current += month_length(m, y);
            current %= 7;
            if current == 0 { sunday_count += 1; }
        }
    }
    sunday_count
}

fn month_length(month: u8, year: u16) -> usize {
    match month {
        0 | 2 | 4 | 6 | 7 | 9 | 11 => 31,
        3 | 5 | 8 | 10 => 30,
        1 => if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 },
        _ => panic!("unknown month {month}")
    }
}
