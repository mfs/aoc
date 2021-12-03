use anyhow::Result;
use std::io::{self, BufRead};

const BITS: usize = 12;
const MASK: u64 = (1 << BITS) - 1;

fn main() -> Result<()> {
    let mut numbers = vec![];

    for line in io::stdin().lock().lines() {
        let n = u64::from_str_radix(&line?, 2)?;
        numbers.push(n);
    }

    let mut gamma: u64 = 0;

    for i in 0..BITS {
        let count = numbers.iter().filter(|x| *x & (1 << i) != 0).count();

        if count > numbers.len() - count {
            gamma |= 1 << i;
        }
    }

    println!("Part 1: {}", gamma * (!gamma & MASK));

    let mut oxygen = numbers.clone();
    process(&mut oxygen, 1, 0);

    let mut co2 = numbers.clone();
    process(&mut co2, 0, 1);

    println!("Part 2: {}", oxygen[0] * co2[0]);

    Ok(())
}

fn process(numbers: &mut Vec<u64>, r0: u64, r1: u64) {
    for i in (0..BITS).rev() {
        let ones = numbers.iter().filter(|x| (*x >> i) & 1 == 1).count();

        if ones >= numbers.len() - ones {
            numbers.retain(|x| (*x >> i) & 1 == r0);
        } else {
            numbers.retain(|x| (*x >> i) & 1 == r1);
        }

        if numbers.len() == 1 {
            break;
        }
    }
}
