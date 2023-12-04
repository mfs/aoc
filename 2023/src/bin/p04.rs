use anyhow::{anyhow, Result};
use std::collections::{BTreeSet, HashMap};
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut part1 = 0;
    let mut cards = HashMap::new();

    for line in io::stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<_> = line.split(|x| x == ':' || x == '|').collect();

        let idx: usize = tokens[0]
            .split_whitespace()
            .nth(1)
            .ok_or(anyhow!("error parsing game number"))?
            .parse()?;

        let numbers: Vec<_> = tokens[1..]
            .iter()
            .map(|x| {
                x.trim()
                    .split_whitespace()
                    .map(|x| x.parse::<u32>())
                    .collect::<Result<BTreeSet<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let score = numbers[1].intersection(&numbers[0]).count();

        part1 += 2u32.pow(score as u32 - 1);

        for i in 1..=score {
            *cards.entry(idx + i).or_insert(1) += 1 * *cards.entry(idx).or_insert(1);
        }
    }

    println!("Part 1: {}", part1);

    println!("Part 2: {}", cards.values().sum::<u32>());

    Ok(())
}
