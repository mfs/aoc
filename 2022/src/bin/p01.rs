use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut elves = vec![0u64];

    for line in io::stdin().lock().lines() {
        if let Ok(n) = line?.parse::<u64>() {
            let idx = elves.len() - 1;
            elves[idx] += n;
        } else {
            elves.push(0)
        }
    }

    elves.sort_unstable_by(|a, b| b.cmp(a));

    println!("Part 1. {}", elves[0]);

    println!("Part 2: {}", elves.iter().take(3).sum::<u64>());

    Ok(())
}
