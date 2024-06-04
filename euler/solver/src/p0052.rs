use integers::Integer;

pub fn p0052_solver() -> String {
    permuted_multiples().to_string()
}

fn permuted_multiples() -> u32 {
    (1..)
        .map(|n| (n, n.as_decimal()))
        .find(|(n,nd)| (2..=6)
            .map(|x| (n * x).as_decimal())
            .all(|d| d.same_digits(nd)))
        .unwrap()
        .0
}
