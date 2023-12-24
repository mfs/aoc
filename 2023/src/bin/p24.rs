use anyhow::Result;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
struct Hailstone {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
}

fn main() -> Result<()> {
    let mut hailstones = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        let ns: Vec<i64> = line
            .split(&[',', '@'])
            .map(|s| s.trim().parse())
            .collect::<Result<_, _>>()?;

        hailstones.push(Hailstone {
            p: (ns[0], ns[1], ns[2]),
            v: (ns[3], ns[4], ns[5]),
        });
    }

    const TARGET_MIN: f64 = 200000000000000.0;
    const TARGET_MAX: f64 = 400000000000000.0;

    use itertools::Itertools;
    let mut count = 0;
    for h in hailstones.iter().combinations(2) {
        let h0 = h[0];
        let h1 = h[1];

        let x1 = h0.p.0 as f64;
        let x2 = h0.p.0 as f64 + h0.v.0 as f64;

        let x3 = h1.p.0 as f64;
        let x4 = h1.p.0 as f64 + h1.v.0 as f64;

        let y1 = h0.p.1 as f64;
        let y2 = h0.p.1 as f64 + h0.v.1 as f64;

        let y3 = h1.p.1 as f64;
        let y4 = h1.p.1 as f64 + h1.v.1 as f64;

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4))
            / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));

        let u = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2))
            / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4));

        if t > 0.0 && u > 0.0 {
            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);

            if x < TARGET_MIN || x > TARGET_MAX || y < TARGET_MIN || y > TARGET_MAX {
                continue;
            }
            count += 1;
        }
    }

    println!("Part 1: {}", count);

    Ok(())
}
