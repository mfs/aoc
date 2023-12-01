use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut lines = vec![];

    for line in io::stdin().lock().lines() {
        lines.push(line?.to_owned());
    }

    let mut sum = 0;
    for line in &lines {
        sum += value(line, numbers, false);
    }

    println!("Part 1: {}", sum);

    let mut sum = 0;
    for line in &lines {
        sum += value(line, numbers, true);
    }

    println!("Part 2: {}", sum);

    Ok(())
}

fn value(s: &str, numbers: [&str; 9], part2: bool) -> u32 {
    let l = s.len();
    let mut result = 0;

    // find first digit
    let mut idx = 0;
    loop {
        if let Some(left) = digit(&s[idx..], numbers, part2) {
            result += left * 10;
            break;
        }
        idx += 1;
    }

    // find second digit
    let mut idx = l - 1;
    loop {
        if let Some(right) = digit(&s[idx..], numbers, part2) {
            result += right;
            break;
        }
        idx -= 1;
    }

    result
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
