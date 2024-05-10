pub fn p0026_solver() -> String {
    reciprocal_cycles(1000).to_string()
}

fn reciprocal_cycles(max: u32) -> u32 {
    (2..max)
        .map(|d| (d, reciprocal_cycle_length(d)))
        .max_by_key(|&(_, l)| l)
        .unwrap()
        .0
}

fn reciprocal_cycle_length(d: u32) -> usize {
    let mut remainders = vec!();
    let mut remainder = 1;
    while remainder != 0 {
        let factor = remainder / d;
        remainder -= factor * d;
        remainder *= 10;
        if let Some(i) = remainders.iter().position(|&r| r == remainder) {
            return remainders.len() - i;
        } else {
            remainders.push(remainder);
        }
    }
    0
}

#[test]
fn finds_number_with_longest_reciprocal_cycle() {
    assert_eq!(reciprocal_cycles(10), 7);
}
