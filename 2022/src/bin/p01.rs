use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut elves = vec![0u64];
    let mut last = 0;

    for line in io::stdin().lock().lines() {
        if let Ok(n) = line?.parse::<u64>() {
            elves[last] += n;
        } else {
            elves.push(0);
            last += 1;
        }
    }

    elves.sort_unstable_by(|a, b| b.cmp(a));

    println!("Part 1. {}", elves[0]);

    println!("Part 2: {}", elves.iter().take(3).sum::<u64>());

    Ok(())
}
