pub fn sieve(max: usize) -> Vec<bool> {
    let mut primes = vec!();
    primes.resize(max+1, true);
    for q in (4..=max).step_by(2) {
        primes[q] = false;
    }
    for p in (3..=max).step_by(2) {
        if !primes[p] { continue; }
        for q in (p+p..=max).step_by(p) {
            primes[q] = false;
        }
    }
    primes
}
