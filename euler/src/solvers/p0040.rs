use crate::libs::integers::digits::digitable::{Digitable, Digits};

pub fn p0040_solver() -> String {
    let mut c = champernownes_constant();
    (0..=6)
        .map(|i| c.get(10usize.pow(i)))
        .product::<u32>()
        .to_string()
}

fn champernownes_constant() -> ChampernownesConstant {
    ChampernownesConstant::new()
}

struct ChampernownesConstant {
    digits: Digits<u32>,
    n: u32,
}

impl ChampernownesConstant {
    fn new() -> Self {
        Self { digits: 1.as_decimal(), n: 1 }
    }

    fn get(&mut self, nth: usize) -> u32 {
        while self.digits.len() < nth {
            self.n += 1;
            self.digits.concatenate(self.n.as_decimal());
        }
        self.digits.get(nth-1).unwrap()
    }
}

#[test]
fn finds_nth_digit_of_champernownes_constant() {
    let mut c = champernownes_constant();
    assert_eq!(c.get(12), 1);
}
