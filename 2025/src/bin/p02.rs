use std::io::{self, Read};

use anyhow::Result;

fn main() -> Result<()> {
    let ranges = parse()?;

    let mut total_part1 = 0;
    let mut total_part2 = 0;

    for range in &ranges {
        for n in range.0..=range.1 {
            let digits = n.ilog10() + 1;

            if part1_is_invalid(n, digits) {
                total_part1 += n;
            }

            if part2_is_invalid(n, digits) {
                total_part2 += n;
            }
        }
    }

    println!("Part 1: {}", total_part1);
    println!("Part 2: {}", total_part2);

    Ok(())
}

fn part1_is_invalid(n: u64, digits: u32) -> bool {
    let d = 10u64.pow(digits / 2);

    n / d == n % d
}

fn part2_is_invalid(n: u64, digits: u32) -> bool {
    for len in (1..=digits/2).filter(|x| digits % x == 0) {
        if seq(n, len).windows(2).all(|x| x[0] == x[1]) {
            return true;
        }
    }

    false
}

fn seq(mut n: u64, size: u32) -> Vec<u64> {
    let mut v = vec![];
    let pow = 10u64.pow(size);

    while n != 0 {
        v.push(n % pow);
        n /= pow;
    }

    v
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
