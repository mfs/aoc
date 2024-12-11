use std::collections::HashMap;
use std::io::{self, Read};

use anyhow::Result;

type Cache = HashMap<(u64, u32), u64>;

fn main() -> Result<()> {
    let stones = parse()?;

    for (i, blinks) in [25u32, 75].iter().enumerate() {
        let total = stones
            .iter()
            .map(|&n| blink(n, *blinks, &mut Cache::new()))
            .sum::<u64>();

        println!("Part {}: {}", i + 1, total);
    }
    Ok(())
}

fn blink(n: u64, level: u32, cache: &mut Cache) -> u64 {
    // terminal case
    if level == 0 {
        return 1;
    }

    if let Some (c) = cache.get(&(n, level)) {
        return *c;
    }

    let d = digits(n);

    let count = if n == 0 {
        blink(1, level - 1, cache)
    } else if d % 2 == 0 {
        let p = 10u64.pow(d / 2);

        blink(n / p, level - 1, cache) + blink(n % p, level - 1, cache)
    } else {
        blink(n * 2024, level - 1, cache)
    };

    cache.insert((n, level), count);

    count
}

fn digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut count = 0;

    while n != 0 {
        n /= 10;

        count += 1;
    }

    count
}

fn parse() -> Result<Vec<u64>> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let stones = buffer
        .split_whitespace()
        .map(|n| n.parse())
        .collect::<Result<_, _>>()?;

    Ok(stones)
}
