pub fn p0039_solver() -> String {
    (12..=1000)
        .map(|p| (p, integer_right_triangles(p).len()))
        .max_by_key(|&(_,n)| n)
        .unwrap()
        .0
        .to_string()
}

fn integer_right_triangles(perimeter: u32) -> Vec<(u32, u32, u32)> {
    let mut triangles = vec!();
    for a in 1..perimeter {
        let a2 = a * a;
        for b in a..(perimeter-a) {
            let b2 = b * b;
            let c = perimeter - a - b;
            if a2 + b2 == c * c {
                triangles.push((a, b, c));
            }
        }
    }
    triangles
}

#[test]
fn finds_integer_right_triangles_within_perimeter() {
    let triangles = integer_right_triangles(120);
    assert_eq!(triangles.contains(&(20, 48, 52)), true);
    assert_eq!(triangles.contains(&(24, 45, 51)), true);
    assert_eq!(triangles.contains(&(30, 40, 50)), true);
}
