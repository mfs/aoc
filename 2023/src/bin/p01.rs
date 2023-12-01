use anyhow::{anyhow, Result};
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut part1 = 0;
    let mut part2 = 0;

    for line in io::stdin().lock().lines() {
        let line = line?;
        part1 += value(&line, numbers, false)?;
        part2 += value(&line, numbers, true)?;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn value(s: &str, numbers: [&str; 9], part2: bool) -> Result<u32> {
    let l = s.len();

    let left = (0..l)
        .flat_map(|idx| digit(&s[idx..], numbers, part2))
        .nth(0)
        .ok_or(anyhow!("missing digit"))?;

    let right = (0..l)
        .rev()
        .flat_map(|idx| digit(&s[idx..], numbers, part2))
        .nth(0)
        .ok_or(anyhow!("missing digit"))?;

    Ok(left * 10 + right)
}

fn digit(s: &str, numbers: [&str; 9], part2: bool) -> Option<u32> {
    if let Ok(x) = s[0..1].parse() {
        return Some(x);
    }

    if !part2 {
        return None;
    }

    for (idx, num) in numbers.iter().enumerate() {
        if s.starts_with(num) {
            return Some(idx as u32 + 1);
        }
    }

    None
}
