use number_lists::Pentagonal;

pub fn p0044_solver() -> String {
    pentagon_numbers()
        .map(|(pj, pk)| pk - pj)
        .min()
        .unwrap()
        .to_string()
}

fn pentagon_numbers() -> impl Iterator<Item=(u64,u64)> {
    const MAX: u64 = 3000;
    (1..MAX)
        .flat_map(|j| (j+1..=MAX).map(move |k| (j,k)))
        .map(|(j,k)| (j.as_pentagonal(), k.as_pentagonal()))
        .filter(|&(pj,pk)| (pk + pj).is_pentagonal() && (pk - pj).is_pentagonal())
}
