use std::io::{self, BufRead};

use regex::Regex;
use anyhow::Result;

type V = (i64, i64);
type Machine = (V, V, V);

const OFFSET: i64 = 10_000_000_000_000;

fn main() -> Result<()> {
    let machines = parse()?;

    println!("Part 1: {}", part1(&machines));

    println!("Part 2: {}", part2(&machines));

    Ok(())
}

// https://en.wikipedia.org/wiki/Cramer%27s_rule#Explicit_formulas_for_small_systems
fn part2(machines: &[Machine]) -> i64 {
    let mut total = 0;

    for m in machines {
        let den = m.0.0 * m.1.1 - m.1.0 * m.0.1; // no zero denominators in input

        let prize = (m.2.0 + OFFSET, m.2.1 + OFFSET);

        let a = prize.0 * m.1.1 - m.1.0 * prize.1;
        let b = m.0.0 * prize.1 - prize.0 * m.0.1;

        if a % den == 0 && b % den == 0 {
            total += (a / den) * 3 + (b / den);
        }
    }

    total
}

fn part1(machines: &[Machine]) -> i64 {
   let mut total = 0;

    'outer: for m in machines {
        for a in 0..=100 {
            for b in 0..=100 {
                let pos = (m.0.0 * a + m.1.0 * b, m.0.1 * a + m.1.1 * b);
                if pos == m.2 {
                    total += a * 3 + b;
                    continue 'outer;
                }
            }
        }
    }

    total
}

fn parse() -> Result<Vec<Machine>> {
    let re = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)|Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut machines = vec![];
    let mut machine = Machine::default();

    for line in io::stdin().lock().lines() {
        let line = line?;

        if let Some(c) = re.captures(&line) {
            if c[0].starts_with("Button A") {
                machine.0 = (c[1].parse()?, c[2].parse()?);
            } else if c[0].starts_with("Button B") {
                machine.1 = (c[1].parse()?, c[2].parse()?);
            } else {
                machine.2 = (c[3].parse()?, c[4].parse()?);
                machines.push(machine);
                machine = Machine::default();
            }
        }
    }

    Ok(machines)
}
