pub fn p0007_solver() -> usize {
    nth_prime(10001)
}

fn nth_prime(n: usize) -> usize {
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

fn is_not_prime(n: usize, primes: &Vec<usize>) -> bool {
    primes.iter().any(|p| n % p == 0)
}

#[test]
fn finds_nth_prime() {
    assert_eq!(nth_prime(6), 13);
    assert_eq!(nth_prime(10), 29);
}
