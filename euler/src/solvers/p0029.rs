use crate::libs::integers::factorizable::Factorizable;

pub fn p0029_solver() -> String {
    distinct_powers(100).to_string()
}

fn distinct_powers(max: u64) -> usize {
    let mut powers = vec!();
    for a in 2..=max {
        let factorized = a.factorize();
        for b in 2..=max {
            let power = factorized.pow(b);
            if !powers.contains(&power) {
                powers.push(power);
            }
        }
    }
    powers.len()
}

#[test]
fn counts_number_of_distinct_powers() {
    assert_eq!(distinct_powers(5), 15);
}
