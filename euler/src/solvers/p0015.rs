pub fn p0015_solver() -> String {
    lattice_paths(20).to_string()
}

fn lattice_paths(size: usize) -> u128 {
    let mut row = vec!();
    row.resize(size+1, 1);
    for _ in 1..=size {
        for j in 1..=size {
            row[j] += row[j-1];
        }
    }
    row[size]
}

#[test]
fn counts_paths_in_lattice_of_size() {
    assert_eq!(lattice_paths(1), 2);
    assert_eq!(lattice_paths(2), 6);
    assert_eq!(lattice_paths(3), 20);
    assert_eq!(lattice_paths(4), 70);
}
