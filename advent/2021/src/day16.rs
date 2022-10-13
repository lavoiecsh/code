#[allow(unused_macros)] #[cfg(debug_assertions)] macro_rules! debug { ($x: expr) => { dbg!($x) }}
#[allow(unused_macros)] #[cfg(not(debug_assertions))] macro_rules! debug { ($x:expr) => { std::convert::identity($x) }}

use std::fs;

const FILENAME: &str = "inputs/day16.txt";

#[allow(dead_code)]
fn read_input() -> BITS {
    BITS::new(fs::read_to_string(FILENAME).expect("error reading")
            .trim()
            .chars()
            .flat_map(|c|to_bits(c))
            .collect())
}

#[allow(dead_code)]
pub fn part1() -> usize {
    let mut bits = read_input();
    let packet = Packet::new(&mut bits);
    packet.version_sum()
}

#[allow(dead_code)]
pub fn part2() -> usize {
    let mut bits = read_input();
    let packet = Packet::new(&mut bits);
    packet.expression_value()
}

#[allow(dead_code)]
struct Packet {
    version: usize,
    type_id: usize,
    length_type_id: usize,
    value: usize,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn new(bits: &mut BITS) -> Packet {
        let v = bits.read_n(3);
        let t = bits.read_n(3);
        debug!((v, t));
        if t == 4 {
            Packet {
                version: v,
                type_id: t,
                length_type_id: usize::MAX,
                value: debug!(bits.read_number()),
                sub_packets: Vec::new(),
            }
        } else {
            let mut sp = Vec::new();
            let l = debug!(bits.read_n(1));
            match l {
                0 => {
                    let length = bits.read_n(15);
                    let before = bits.current;
                    while bits.current != before + length {
                        debug!((bits.current, before, length));
                        sp.push(Packet::new(bits));
                    }
                },
                1 => {
                    let mut count = bits.read_n(11);
                    while count > 0 {
                        debug!(count);
                        sp.push(Packet::new(bits));
                        count -= 1;
                    }
                },
                _ => panic!("unknown length type id"),
            }
            Packet {
                version: v,
                type_id: t,
                length_type_id: l,
                value: usize::MAX,
                sub_packets: sp,
            }
        }
    }

    fn version_sum(&self) -> usize {
        if self.type_id == 4 {
            return self.version;
        }

        return self.version + self.sub_packets.iter().fold(0, |acc, sp| acc + sp.version_sum());
    }

    fn expression_value(&self) -> usize {
        match self.type_id {
            0 => self.sub_packets.iter().fold(0, |acc, sp| acc + sp.expression_value()),
            1 => self.sub_packets.iter().fold(1, |acc, sp| acc * sp.expression_value()),
            2 => self.sub_packets.iter().map(|sp| sp.expression_value()).min().unwrap(),
            3 => self.sub_packets.iter().map(|sp| sp.expression_value()).max().unwrap(),
            4 => self.value,
            5 => if self.sub_packets[0].expression_value()  > self.sub_packets[1].expression_value() { 1 } else { 0 },
            6 => if self.sub_packets[0].expression_value()  < self.sub_packets[1].expression_value() { 1 } else { 0 },
            7 => if self.sub_packets[0].expression_value() == self.sub_packets[1].expression_value() { 1 } else { 0 },
            _ => panic!("unknown type_id"),
        }
    }
}

struct BITS {
    bits: Vec<u8>,
    current: usize,
}

impl BITS {
    fn new(bits: Vec<u8>) -> BITS {
        BITS {
            bits: bits.to_vec(),
            current: 0
        }
    }
    
    fn read_n(&mut self, mut size: usize) -> usize {
        let mut number: usize = 0;
        while size != 0 {
            number = number * 2 + self.bits[self.current] as usize;
            self.current += 1;
            size -= 1;
        }
        number
    }

    fn read_number(&mut self) -> usize {
        let mut number: usize = 0;
        while self.bits[self.current] == 1 {
            self.current += 1;
            number *= 16;
            number += self.read_n(4);
        }
        self.current += 1;
        number *= 16;
        number += self.read_n(4);
        number
    }
}

fn to_bits(c: char) -> [u8; 4] {
    match c {
        '0' => [0, 0, 0, 0],
        '1' => [0, 0, 0, 1],
        '2' => [0, 0, 1, 0],
        '3' => [0, 0, 1, 1],
        '4' => [0, 1, 0, 0],
        '5' => [0, 1, 0, 1],
        '6' => [0, 1, 1, 0],
        '7' => [0, 1, 1, 1],
        '8' => [1, 0, 0, 0],
        '9' => [1, 0, 0, 1],
        'A' => [1, 0, 1, 0],
        'B' => [1, 0, 1, 1],
        'C' => [1, 1, 0, 0],
        'D' => [1, 1, 0, 1],
        'E' => [1, 1, 1, 0],
        'F' => [1, 1, 1, 1],
        _ => panic!("unexpected char")
    }
}
