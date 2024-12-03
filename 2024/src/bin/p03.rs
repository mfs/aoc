use std::io::{self, Read};

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {

    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don\'t\(\)").unwrap();

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;
    let mut enabled = true;

    for c in re.captures_iter(&buffer) {
        match (&c[0], enabled) {
            ("do()", _) => enabled = true,
            ("don't()", _) => enabled = false,
            (_, true) => {
                let product = c[1].parse::<u32>()? * c[2].parse::<u32>()?;
                part1 += product;
                part2 += product;
            },
            (_, false) => part1 += c[1].parse::<u32>()? * c[2].parse::<u32>()?,
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
