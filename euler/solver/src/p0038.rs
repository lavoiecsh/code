use integers::Integer;

pub fn p0038_solver() -> String {
    pandigital_multiples().into_iter().map(|(_,_,pm)| pm).max().unwrap().to_string()
}

fn pandigital_multiples() -> Vec<(u64, u64, u64)> {
    let mut pm = vec!();
    for p in 1..100000 {
        let mut d = 0u64.as_decimal();
        for n in 1..10 {
            d.concatenate((p * n).as_decimal());
            if d.len() > 9 {
                break;
            }
            if d.is_pandigital() {
                pm.push((p, n, d.number()));
                break;
            }
        }
    }
    pm
}

#[test]
fn finds_pandigital_multiples() {
    let pm = pandigital_multiples();
    assert_eq!(pm.contains(&(192, 3, 192384576)), true);
    assert_eq!(pm.contains(&(9, 5, 918273645)), true);
}
