use std::io::{self, Read};

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {

    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut muls: Vec<(u32, u32)> = vec![];

    for c in re.captures_iter(&buffer) {
        muls.push((c[1].parse()?, c[2].parse()?));
    }

    let part1: u32 = muls.iter().map(|(x, y)| x * y).sum();

    println!("Part 1: {}", part1);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don\'t\(\)").unwrap();

    let mut muls: Vec<(u32, u32)> = vec![];
    let mut enabled = true;

    for c in re.captures_iter(&buffer) {
        match (&c[0], enabled) {
            ("do()", _) => enabled = true,
            ("don't()", _) => enabled = false,
            (_, true) => {
                muls.push((c[1].parse()?, c[2].parse()?));
            },
            (_, false) => {},
        }
    }

    let part2: u32 = muls.iter().map(|(x, y)| x * y).sum();

    println!("Part 2: {}", part2);

    Ok(())
}
