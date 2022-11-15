use std::fs;

const INPUT: usize = 36000000;

pub fn part1() -> usize {
    const MAX: usize = 1000000;
    let mut sieve: [usize; MAX] = [0; MAX];
    for i in 1..MAX {
        let mut j = i;
        let presents = i * 10;
        while j < MAX {
            sieve[j] += presents;
            j += i;
        }
        if sieve[i] > INPUT {
            return i;
        }
    }
    0
}

pub fn part2() -> usize {
    const MAX: usize = 1000000;
    let mut sieve: [usize; MAX] = [0; MAX];
    for i in 1..MAX {
        let presents = i * 11;
        for j in 0..50 {
            let house = i + j*i;
            if house >= MAX {
                break;
            }
            sieve[house] += presents;
        }
        if sieve[i] > INPUT {
            return i;
        }
    }
    0
}
