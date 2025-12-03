use std::io::{self, BufRead};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let banks = parse()?;

    let mut joltage_part_1 = 0;
    let mut joltage_part_2 = 0;

    for bank in &banks {
        joltage_part_1 += joltage(&bank, 2)?.iter().fold(0, |acc, elem| acc * 10 + *elem as u64);
        joltage_part_2 += joltage(&bank, 12)?.iter().fold(0, |acc, elem| acc * 10 + *elem as u64);
    }

    println!("Part 1: {}", joltage_part_1);
    println!("Part 2: {}", joltage_part_2);

    Ok(())
}

// assumes bank is at least 12 digits long
fn joltage(bank: &[u32], size: usize) -> Result<Vec<u32>> {
    if size == 1 {
        let next = bank
            .iter()
            .max()
            .context("empty bank")?;

        return Ok(vec![*next]);
    }

    let off = bank.len() - size + 1;
    let max = bank[..off]
        .iter()
        .max()
        .context("empty bank")?;

    let max_idx = bank
        .iter()
        .position(|x| x == max)
        .context("empty bank")?;

    let mut v = vec![*max];
    v.extend_from_slice(&joltage(&bank[max_idx+1..], size - 1)?);

    Ok(v)
}

fn parse() -> Result<Vec<Vec<u32>>> {
    let mut banks = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;
        banks.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    Ok(banks)
}
