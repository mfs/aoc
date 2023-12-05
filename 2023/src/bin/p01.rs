use anyhow::{anyhow, Result};
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in io::stdin().lock().lines() {
        let line = line?;
        part1 += value(&line, false)?;
        part2 += value(&line, true)?;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn value(s: &str, part2: bool) -> Result<u32> {
    let l = s.len();

    let left = (0..l)
        .flat_map(|idx| digit(&s[idx..], part2))
        .nth(0)
        .ok_or(anyhow!("missing digit"))?;

    let right = (0..l)
        .rev()
        .flat_map(|idx| digit(&s[idx..], part2))
        .nth(0)
        .ok_or(anyhow!("missing digit"))?;

    Ok(left * 10 + right)
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn digit(s: &str, part2: bool) -> Option<u32> {
    if let Ok(x) = s[0..1].parse() {
        return Some(x);
    }

    if !part2 {
        return None;
    }

    for (idx, num) in NUMBERS.iter().enumerate() {
        if s.starts_with(num) {
            return Some(idx as u32 + 1);
        }
    }

    None
}
