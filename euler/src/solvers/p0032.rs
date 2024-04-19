use itertools::Itertools;

pub fn p0032_solver() -> String {
    pandigital_products()
        .map(|(_, _, p)| p)
        .unique()
        .sum::<u64>()
        .to_string()
}

fn pandigital_products() -> impl Iterator<Item=(u64, u64, u64)> {
    let mut products = vec!();
    for multiplicand in 1..1000 {
        for multiplier in multiplicand..2000 {
            let product = multiplicand * multiplier;
            if is_1_9_pandigital(&format!("{multiplicand}{multiplier}{product}")) {
                products.push((multiplicand, multiplier, product));
            }
        }
    }
    products.into_iter()
}

fn is_1_9_pandigital(input: &str) -> bool {
    input.len() == 9 &&
        !input.contains('0') &&
        input.chars().unique().count() == 9
}

#[test]
fn finds_pandigital_products() {
    assert_eq!(pandigital_products().any(|(mand, mier, prod)| mand == 39 && mier == 186 && prod == 7254), true);
}
