pub fn p0009_solver() -> String {
    special_pythagorean_triplet(1000).to_string()
}

fn special_pythagorean_triplet(sum: u64) -> u64 {
    (1..=sum)
        .flat_map(|a| (a+1..sum-a).map(move |b| (a, b)))
        .map(|(a,b)| (a,b,sum-a-b))
        .find(|(a,b,c)| a * a + b * b == c * c)
        .map(|(a,b,c)| a * b * c)
        .unwrap()
}

#[test]
fn finds_pythagorean_triplet() {
    assert_eq!(special_pythagorean_triplet(12), 60);
}
