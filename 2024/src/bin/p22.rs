use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};

use anyhow::Result;

fn main() -> Result<()> {
    let numbers = parse()?;

    let part1 = numbers.iter().map(|&n| secret_number(n)).sum::<i64>();

    println!("Part 1: {}", part1);

    // part 2
    let mut map: HashMap<(i8, i8, i8, i8), i32> = HashMap::new();

    for n in &numbers {
        let mut num = *n;
        let mut prices = vec![num % 10];
        for _ in 0..2000 {
            num = next_secret(num);
            prices.push(num % 10);
        }

        let deltas: Vec<_> = prices.windows(2).map(|w| (w[1] as i32 - w[0] as i32) as i8).collect();

        let mut seen = HashSet::new();

        for (idx, w) in deltas.windows(4).enumerate() {
            let t = (w[0], w[1], w[2], w[3]);
            if !seen.contains(&t) {
                *map.entry(t).or_default() += prices[idx+1] as i32;
                seen.insert(t);
            }
        }
    }

    let part2 = map.values().max().unwrap();

    println!("Part 2: {}", part2);

    Ok(())
}

fn next_secret(mut n: i64) -> i64 {
    let mix = |a: i64, b: i64| { a ^ b };
    let prune = |a: i64| { a % 16777216 };

    n = prune(mix(n * 64, n));

    n = prune(mix(n / 32, n));

    prune(mix(n * 2048, n))
}

fn secret_number(mut n: i64) -> i64 {
    for _ in 0..2000 {
        n = next_secret(n);
    }

    n
}

fn parse() -> Result<Vec<i64>> {
    let mut numbers = vec![];

    for line in io::stdin().lock().lines() {
        numbers.push(line?.parse()?);
    }

    Ok(numbers)
}
