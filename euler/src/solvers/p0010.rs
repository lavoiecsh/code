pub fn p0010_solver() -> String {
    summation_of_primes(2_000_000).to_string()
}

fn summation_of_primes(max: u64) -> u64 {
    let mut sieve: Vec<bool> = vec!();
    sieve.resize(max as usize, true);
    sieve[0] = false;
    sieve[1] = false;
    let mut i = 1;
    while i < max - 1 {
        i += 1;
        if !sieve[i as usize] {
            continue;
        }

        let mut j = i * 2;
        while j < max {
            sieve[j as usize] = false;
            j += i;
        }
    }

    sieve.into_iter()
        .enumerate()
        .filter_map(|(i,s)| if s { Some(i as u64) } else { None })
        .sum()
}

#[test]
fn finds_sum_of_primes_under_value() {
    assert_eq!(summation_of_primes(10), 17);
}
