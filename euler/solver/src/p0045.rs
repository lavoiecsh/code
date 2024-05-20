use number_lists::{hexagonals, pentagonals, PolygonalIterator, triangulars};

pub fn p0045_solver() -> String {
    triangular_pentagonal_hexagonal().nth(1).unwrap().3.to_string()
}

fn triangular_pentagonal_hexagonal() -> impl Iterator<Item=(u64, u64, u64, u64)> {
    TriplePolygonalIterator::new()
}

struct TriplePolygonalIterator {
    ti: u64,
    tn: PolygonalIterator<u64>,
    pi: u64,
    pn: PolygonalIterator<u64>,
    hi: u64,
    hn: PolygonalIterator<u64>,
}

impl TriplePolygonalIterator {
    fn new() -> Self {
        let mut s = Self {
            ti: 0,
            tn: triangulars(),
            pi: 0,
            pn: pentagonals(),
            hi: 0,
            hn: hexagonals(),
        };
        s.nt();
        s.np();
        s.nh();
        s
    }

    fn nt(&mut self) -> u64 {
        self.ti += 1;
        self.tn.next().unwrap()
    }

    fn np(&mut self) -> u64 {
        self.pi += 1;
        self.pn.next().unwrap()
    }

    fn nh(&mut self) -> u64 {
        self.hi += 1;
        self.hn.next().unwrap()
    }
}

impl Iterator for TriplePolygonalIterator {
    type Item = (u64, u64, u64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let mut t = self.nt();
        let mut p = self.np();
        let mut h = self.nh();
        while t != p || t != h {
            if t <= p && t <= h {
                t = self.nt();
            } else if p <= t && p <= h {
                p = self.np();
            } else if h <= t && h <= p {
                h = self.nh();
            }
        }
        Some((self.ti, self.pi, self.hi, t))
    }
}

#[test]
fn finds_triangular_pentagonal_hexagonal_numbers() {
    assert_eq!(triangular_pentagonal_hexagonal().next(), Some((285, 165, 143, 40755)));
}
