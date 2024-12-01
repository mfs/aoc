use std::iter::zip;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;

fn main() -> Result<()> {
    let mut left = vec![];
    let mut right = vec![];

    for line in io::stdin().lock().lines() {
        let x: Vec<i64> = line?
            .split_whitespace()
            .map(i64::from_str)
            .collect::<Result<_, _>>()?;
        left.push(x[0]);
        right.push(x[1]);
    }

    left.sort_unstable();
    right.sort_unstable();

    let part1: i64 = zip(&left, &right).map(|(l, r)| (r - l).abs()).sum();

    println!("Part 1: {}", part1);

    let mut counts = HashMap::new();

    for r in &right {
        *counts.entry(r).or_insert(0) += 1;
    }

    let part2: i64 = left.iter().map(|l| l * counts.get(l).unwrap_or(&0)).sum();

    println!("Part 2: {}", part2);

    Ok(())
}
