use integers::Integer;

pub fn p0056_solver() -> String {
    powerful_digit_sum().to_string()
}

fn powerful_digit_sum() -> u32 {
    let mut max = 0;
    for a in 1..100 {
        let mut power = a.as_decimal();
        let sum = power.iter().sum();
        if sum > max { max = sum; }
        for _ in 2..100 {
            power *= a;
            let sum = power.iter().sum();
            if sum > max { max = sum; }
        }
    }
    max
}