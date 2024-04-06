pub fn p0018_solver() -> String {
    maximum_path_sum_1(&read_triangle(TRIANGLE)).to_string()
}

fn maximum_path_sum_1(triangle: &Vec<Vec<u32>>) -> u128 {
    let mut sums = vec!(0, 0);
    for row in triangle.iter() {
        let mut next_sums = vec!(0);
        for (col,&value) in row.iter().enumerate() {
            let above = u128::max(sums[col], sums[col+1]);
            next_sums.push(above + value as u128);
        }
        next_sums.push(0);
        sums = next_sums;
    }
    sums.into_iter().max().unwrap()
}

fn read_triangle(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|l| l.split(" ").map(|c| c.parse().unwrap()).collect())
        .collect()
}

#[test]
fn finds_path_with_maximum_sum() {
    let triangle = read_triangle(TEST_TRIANGLE);
    assert_eq!(maximum_path_sum_1(&triangle), 23);
}

#[cfg(test)]
static TEST_TRIANGLE: &str = "\
3
7 4
2 4 6
8 5 9 3
";

static TRIANGLE: &str = "\
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
";
