use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {

    let mut depths: Vec<u64> = vec![];

    for line in io::stdin().lock().lines() {
        depths.push(line?.parse()?);
    }

    let p1 = depths
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count();

    println!("Part 1: {}", p1);

    let p2 = depths
        .windows(4)
        .filter(|w| w[3] > w[0])
        .count();

    println!("Part 2: {}", p2);

    Ok(())
}
