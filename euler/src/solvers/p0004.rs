pub fn p0004_solver() -> usize {
    largest_palindrome_product(3)
}

fn largest_palindrome_product(digits: u32) -> usize {
    let start = usize::pow(10, digits - 1);
    let end = usize::pow(10, digits);
    (start..end).rev()
        .flat_map(|x| (start..end).rev().map(move |y| x * y))
        .filter(|&p| is_palindrome(p))
        .max()
        .unwrap()
}

fn is_palindrome(n: usize) -> bool {
    let mut d = vec!();
    let mut n = n;
    while n > 0 {
        d.push(n % 10);
        n /= 10;
    }
    d.iter().cloned().rev().collect::<Vec<usize>>() == d
}

#[test]
fn computes_largest_palindrome_of_n_digit_product() {
    assert_eq!(largest_palindrome_product(2), 9009);
}
