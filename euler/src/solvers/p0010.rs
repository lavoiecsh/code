pub fn p0010_solver() -> usize {
    summation_of_primes(2_000_000)
}

fn summation_of_primes(max: usize) -> usize {
    let mut sieve: Vec<bool> = vec!();
    sieve.resize(max, true);
    sieve[0] = false;
    sieve[1] = false;
    let mut i = 1;
    while i < max - 1 {
        i += 1;
        if !sieve[i] {
            continue;
        }

        let mut j = i * 2;
        while j < max {
            sieve[j] = false;
            j += i;
        }
    }

    sieve.into_iter()
        .enumerate()
        .filter_map(|(i,s)| if s { Some(i) } else { None })
        .sum()
}

#[test]
fn finds_sum_of_primes_under_value() {
    assert_eq!(summation_of_primes(10), 17);
}
