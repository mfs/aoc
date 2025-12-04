use std::collections::VecDeque;
use std::io::{self, Read};
use std::sync::atomic::{AtomicU64, Ordering};

use anyhow::Result;
use rayon::prelude::*;

fn main() -> Result<()> {
    let ranges = parse()?;

    let total_part1 = AtomicU64::new(0);
    let total_part2 = AtomicU64::new(0);

    ranges.par_iter().for_each( |range| {
        for n in range.0..=range.1 {
            let digits = n.ilog10() + 1;

            if part1_is_invalid(n, digits) {
                total_part1.fetch_add(n, Ordering::SeqCst);
            }

            if part2_is_invalid(n, digits) {
                total_part2.fetch_add(n, Ordering::SeqCst);
            }
        }
    });

    println!("Part 1: {}", total_part1.into_inner());
    println!("Part 2: {}", total_part2.into_inner());

    Ok(())
}

fn part1_is_invalid(n: u64, digits: u32) -> bool {
    let d = 10u64.pow(digits / 2);

    n / d == n % d
}

fn part2_is_invalid(n: u64, digits_num: u32) -> bool {
    let d = digits(n);

    for len in (1..=digits_num/2).filter(|x| digits_num % x == 0) {
        if d[..len as usize].iter().cycle().zip(&d).all(|(a, b)| a == b) {
            return true;
        }
    }

    false
}

fn digits(mut n: u64) -> Vec<u8> {
    let mut d = VecDeque::with_capacity(16);

    while n != 0 {
        d.push_front((n % 10) as u8);
        n /= 10;
    }

    Vec::from(d)
}

fn parse() -> Result<Vec<(u64, u64)>> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut ranges = vec![];

    for range in buffer.trim().split(',') {
        let ns: Vec<_> = range
            .split('-')
            .map(|x| x.parse::<u64>())
            .collect::<Result<_, _>>()?;

        ranges.push((ns[0], ns[1]));
    }

    Ok(ranges)
}
