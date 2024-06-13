use number_theory::Lychrel;

pub fn p0055_solver() -> String {
    lychrel_numbers()
        .take_while(|&n| n < 10_000)
        .count()
        .to_string()
}

fn lychrel_numbers() -> impl Iterator<Item=u128> {
    (1..)
        .filter(|&n| n.is_lychrel(50))
}

#[test]
fn finds_lychrel_numbers() {
    use itertools::Itertools;
    
    assert!(lychrel_numbers().contains(&196));
    assert!(lychrel_numbers().contains(&4994));
}