pub fn p0007_solver() -> String {
    nth_prime(10_001).to_string()
}

fn nth_prime(n: usize) -> u64 {
    let mut primes = vec!(2, 3);
    let mut last = 3;
    while primes.len() < n {
        last += 2;
        while is_not_prime(last, &primes) {
            last += 2;
        }
        primes.push(last);
    }
    primes[n-1]
}

fn is_not_prime(n: u64, primes: &Vec<u64>) -> bool {
    primes.iter().any(|p| n % p == 0)
}

#[test]
fn finds_nth_prime() {
    assert_eq!(nth_prime(6), 13);
    assert_eq!(nth_prime(10), 29);
}
