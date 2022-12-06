use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let signal: Vec<_> = buffer.chars().collect();

    println!("Part 1: {}", solve(4, &signal));

    println!("Part 2: {}", solve(14, &signal));

    Ok(())
}

fn solve(ws: usize, signal: &[char]) -> usize {
    for (i, w) in signal.windows(ws).enumerate() {
        let hs: HashSet<_> = w.iter().collect();
        if hs.len() == ws {
            return i + ws;
        }
    }
    unreachable!()
}
