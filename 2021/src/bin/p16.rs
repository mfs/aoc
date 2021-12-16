use anyhow::{anyhow, Result};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut bits: Vec<u8> = vec![];

    for digit in buffer.chars() {
        if let Some(nibble) = digit.to_digit(16) {
            let n = nibble as u8;
            bits.append(&mut vec![(n >> 3) & 1, (n >> 2) & 1, (n >> 1) & 1, n & 1]);
        }
    }

    let mut parser = Parser::new(bits);

    let p2 = parser.packet();

    println!("Part 1: {}", parser.version_total);

    println!("Part 2: {}", p2?);

    Ok(())
}

#[derive(Default)]
struct Parser {
    bits: Vec<u8>,
    cur: usize,
    version_total: u64,
}

impl Parser {
    fn new(bits: Vec<u8>) -> Self {
        Parser {bits, ..Default::default() }
    }

    fn packet(&mut self) -> Result<u64> {
        self.version_total += self.read(3);
        let type_id = self.read(3);

        if type_id == 4 {
            // literal
            return Ok(self.literal());
        } else {
            // operator
            let length_type_id = self.read(1);

            let mut sub_packets = vec![];

            if length_type_id == 1 {
                let n = self.read(11);
                for _ in 0..n {
                    sub_packets.push(self.packet()?);
                }
            } else {
                let n = self.read(15);
                let next = self.cur + n as usize;
                while self.cur != next {
                    sub_packets.push(self.packet()?);
                }
            }

            match type_id {
                0 => Ok(sub_packets.iter().sum()),
                1 => Ok(sub_packets.iter().product()),
                2 => sub_packets.into_iter().min().ok_or(anyhow!("empty min packet")),
                3 => sub_packets.into_iter().max().ok_or(anyhow!("empty max packet")),
                5 => Ok(if sub_packets[0] > sub_packets[1] { 1 } else { 0 }),
                6 => Ok(if sub_packets[0] < sub_packets[1] { 1 } else { 0 }),
                7 => Ok(if sub_packets[0] == sub_packets[1] { 1 } else { 0 }),
                _ => Err(anyhow!("invalid type id")),
            }
        }
    }

    fn literal(&mut self) -> u64 {
        let mut value = 0;

        loop {
            let block = self.read(5);
            value = value << 4 | block & 0xf;
            if block >> 4 == 0 { break; }
        }

        value
    }

    fn read(&mut self, n: usize) -> u64 {
        let bits = &self.bits[self.cur..self.cur+n];
        self.cur += n;

        bits.iter().fold(0u64, |acc, bit| acc << 1 | *bit as u64)
    }
}
