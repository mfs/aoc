use std::io::{self, BufRead};

use anyhow::{anyhow, Result};

const MOD: i32 = 100;
const START: i32 = 50;

fn main() -> Result<()> {
    let rotations = parse()?;

    let mut dial = START;
    let mut part_1_zeros = 0;
    let mut part_2_zeros = 0;

    for rot in &rotations {
        let clicks = rot.abs();
        let dir = clicks / rot;

        for _ in 0..clicks {
            dial = (dial + dir).rem_euclid(MOD);
            if dial == 0 {
                part_2_zeros += 1;
            }
        }

        if dial == 0 {
            part_1_zeros += 1;
        }
    }

    println!("Part 1: {}", part_1_zeros);
    println!("Part 2: {}", part_2_zeros);

    Ok(())
}

fn parse() -> Result<Vec<i32>> {
    let mut rotations = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;
        match line.chars().nth(0) {
            Some('L') => rotations.push(-line[1..].parse::<i32>()?),
            Some('R') => rotations.push(line[1..].parse::<i32>()?),
            _ => return Err(anyhow!("Unexpected start char: {}", line)),
        }
    }

    Ok(rotations)
}
