use std::io::{self, Read};
use anyhow::Result;
use regex::Regex;

#[derive(Debug,Default)]
struct Probe {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,

    max_y: i64,
}

impl Probe {
    fn new(vx: i64, vy: i64) -> Self {
        Probe { vx, vy, ..Default::default() }
    }

    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.max_y = std::cmp::max(self.max_y, self.y);
        if self.vx > 0 {
            self.vx -= 1;
        } else if self.vx < 0 {
            self.vx += 1;
        }
        self.vy -= 1;
    }
}

fn process(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> (i64, usize) {
    let mut probe_max_y = 0;
    let mut success = 0;

    // Bounds. There must be a way of calculating bounds however I just brute
    // forced it. Used a guess for vy and went for the target's max_x + 1 as
    // above that the first step would immediately overshoot.

    const VY: i64 = 500;

    for vx in 0..max_x+1 {
        for vy in -VY..VY {
            let mut p = Probe::new(vx, vy);
            loop {
                if p.y > min_y {
                    p.step();
                } else {
                    break;
                }
                if p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y {
                    success += 1;
                    if p.max_y > probe_max_y {
                        probe_max_y = p.max_y;
                    }
                    break;
                }
            }
        }
    }

    (probe_max_y, success)
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let re = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)")?;

    if let Some(caps) = re.captures(&buffer) {
        let min_x = caps[1].parse()?;
        let max_x = caps[2].parse()?;
        let min_y = caps[3].parse()?;
        let max_y = caps[4].parse()?;

        let (p1, p2)  = process(min_x, max_x, min_y, max_y);

        println!("Part 1: {}", p1);

        println!("Part 2: {}", p2);
    }

    Ok(())
}

