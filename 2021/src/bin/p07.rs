use anyhow::{anyhow, Result};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut crabs: Vec<i64> = vec![];

    for n in buffer.trim().split(',') {
        crabs.push(n.parse()?);
    }

    let min = *crabs.iter().min().ok_or(anyhow!("no crabs :("))?;
    let max = *crabs.iter().max().ok_or(anyhow!("no crabs :("))?;

    let p1 = process(&crabs, min, max, true);
    println!("Part 2: {}", p1);

    let p2 = process(&crabs, min, max, false);
    println!("Part 2: {}", p2);

    Ok(())
}

fn process(crabs: &Vec<i64>, min: i64, max: i64, part1: bool) -> i64 {
    let mut best_fuel = i64::MAX;

    for pos in min..max {
        let mut fuel = 0;
        for c in crabs {
            let d = (c - pos).abs();
            if part1 {
                fuel += d;
            } else {
                fuel += d * (d + 1) / 2;
            }
        }

        if fuel < best_fuel {
            best_fuel = fuel;
        }
    }

    best_fuel
}

