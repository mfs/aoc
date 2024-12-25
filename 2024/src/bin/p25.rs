use std::io::{self, Read};
use std::iter::zip;

use anyhow::Result;
use itertools::iproduct;

type Schematic = [u8; 5];

type Schematics = Vec<Schematic>;

fn main() -> Result<()> {
    let (locks, keys) = parse()?;

    let mut count: u32= 0;

    for (lock, key) in iproduct!(&locks, &keys) {
        if !zip(lock, key).any(|(l, k)| l + k > 5) {
            count += 1;
        }
    }

    println!("Part 1: {}", count);

    Ok(())
}

fn parse() -> Result<(Schematics, Schematics)> {
    let mut locks = vec![];
    let mut keys = vec![];

    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    for schematic in buffer.split("\n\n") {
        let mut counts = [0u8; 5];

        for row in schematic.lines().skip(1).take(5) {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    counts[x] += 1;
                }
            }
        }

        if &schematic[0..1] == "#" {
            locks.push(counts);
        } else {
            keys.push(counts);
        }
    }

    Ok((locks, keys))
}
