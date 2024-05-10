pub fn p0033_solver() -> String {
    let frac = digit_cancelling_fractions().into_iter()
        .reduce(|acc,cur| (acc.0 * cur.0, acc.1 * cur.1))
        .unwrap();
    simplify(frac).1.to_string()
}

fn digit_cancelling_fractions() -> Vec<(u32, u32)> {
    let mut fractions = vec!();
    for n1 in 1..=9 {
        for n2 in 1..=9 {
            let d1 = n2;
            for d2 in 1..=9 {
                let n = n1 * 10 + n2;
                let d = d1 * 10 + d2;
                if n < d && n * d2 == n1 * d {
                    fractions.push((n, d));
                }
            }
        }
    }
    fractions
}

fn simplify((num, den): (u32, u32)) -> (u32, u32) {
    let mut num = num;
    let mut den = den;
    while let Some(divisor) = (2..=num).find(|i| num % i == 0 && den % i == 0) { 
        num /= divisor;
        den /= divisor;
    }
    (num, den)
}

#[test]
fn finds_all_digit_cancelling_fractions() {
    let fractions = digit_cancelling_fractions();
    assert_eq!(fractions.len(), 4);
    assert!(fractions.iter().any(|&(n, d)| n == 49 && d == 98));
}
