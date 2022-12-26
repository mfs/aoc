use anyhow::{anyhow, Result};
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut total = 0;

    for line in io::stdin().lock().lines() {
        total += snafu_to_decimal(&line?);
    }

    println!("Part 1: {}", decimal_to_snafu(total)?);

    Ok(())
}

fn decimal_to_snafu(mut n: i64) -> Result<String> {
    let mut snafu = String::new();

    while n > 0 {
        let d = n % 5;
        n /= 5;
        match d {
            0..=2 => {
                let c = char::from_digit(d as u32, 10).ok_or(anyhow!("invalid digit"))?;
                snafu.push(c);
            }
            3 => {
                snafu.push('=');
                n += 1; // carry
            }
            4 => {
                snafu.push('-');
                n += 1; // carry
            }
            _ => unreachable!(),
        }
    }

    Ok(snafu.chars().rev().collect())
}

fn snafu_to_decimal(s: &str) -> i64 {
    let mut n = 0;

    for (i, c) in s.chars().rev().enumerate() {
        n += match c {
            '2' => 2 * 5i64.pow(i as u32),
            '1' => 1 * 5i64.pow(i as u32),
            '0' => 0 * 5i64.pow(i as u32),
            '-' => -1 * 5i64.pow(i as u32),
            '=' => -2 * 5i64.pow(i as u32),
            _ => 0,
        }
    }

    n
}
