use num_integer::binomial;

pub fn p0053_solver() -> String {
    combinatoric_selections()
        .filter(|&(_,_,c)| c > 1_000_000)
        .count()
        .to_string()
}

fn combinatoric_selections() -> impl Iterator<Item=(u128,u128,u128)> {
    (1..=100)
        .flat_map(|n| (1..=n).map(move |k| (n, k, binomial(n, k))))
}

#[test]
fn finds_binomial_selections() {
    use itertools::Itertools;
    assert!(combinatoric_selections().contains(&(23, 10, 1144066)));
}
