use std::io::{self, BufRead};

use anyhow::Result;
use itertools::Itertools;
use regex::Regex;

type V = (i32, i32);
type Robot = (V, V);

const W: i32 = 101;
const H: i32 = 103;

fn main() -> Result<()> {

    let mut robots = parse()?;

    for seconds in 1..(W * H) {
        for (pos, vel) in robots.iter_mut() {
            let x = (pos.0 + vel.0).rem_euclid(W);
            let y = (pos.1 + vel.1).rem_euclid(H);

            *pos = (x, y);
        }

        if seconds == 100 {
            let mut quadrants = [0; 4];

            for (pos, _) in &robots {
                if pos.0 < W/2 && pos.1 < H/2 {
                    quadrants[0] += 1;
                } else if pos.0 > W/2 && pos.1 < H/2 {
                    quadrants[1] += 1;
                } else if pos.0 < W/2 && pos.1 > H/2 {
                    quadrants[2] += 1;
                } else if pos.0 > W/2 && pos.1 > H/2 {
                    quadrants[3] += 1;
                }
            }

            println!("Part 1: {}", quadrants.iter().product::<i32>());
        }

        // this seems to happen when no robots share a square. not 100%
        // sure this is always the case.
        if robots.iter().map(|&r| r.0).all_unique() {
            println!("Part 2: {}", seconds);
            break;
        }
    }

    Ok(())
}

fn parse() -> Result<Vec<Robot>> {
    let mut robots = vec![];

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    for line in io::stdin().lock().lines() {
        let line = line?;

        if let Some(c) = re.captures(&line) {
            let pos = (c[1].parse()?, c[2].parse()?);
            let vel = (c[3].parse()?, c[4].parse()?);
            robots.push((pos, vel));
        }
    }

    Ok(robots)
}
