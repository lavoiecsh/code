use itertools::Itertools;
use num_integer::sqrt;

pub fn proper_divisors(n: u128) -> impl Iterator<Item=u128> {
    let mut divisors = vec!(1);
    for d in 2..=sqrt(n) {
        if n % d == 0 {
            divisors.push(d);
            divisors.push(n / d);
        }
    }
    divisors.into_iter().sorted()
}
