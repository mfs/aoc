use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
use std::io::{self, BufRead};

type Pt = (i64, i64);

#[derive(Debug, Copy, Clone)]
struct Sensor {
    location: Pt,
    beacon: Pt,
}

impl Sensor {
    fn dist(&self) -> i64 {
        (self.location.0 - self.beacon.0).abs() + (self.location.1 - self.beacon.1).abs()
    }

    fn in_range(&self, pt: Pt) -> bool {
        let d = (self.location.0 - pt.0).abs() + (self.location.1 - pt.1).abs();

        d <= self.dist()
    }

    fn border_calc(&self, hs: &mut HashSet<Pt>) -> bool {
        let d = self.dist() + 1; // border

        let mut x0 = self.location.0;
        let mut x1 = self.location.0;

        for y in (0..=d).rev() {
            let bc = vec![
                (x0, self.location.1 - y),
                (x1, self.location.1 - y),
                (x0, self.location.1 + y),
                (x1, self.location.1 + y),
            ];

            for b in bc {
                if b.0 < 0 || b.0 > 4000000 || b.1 < 0 || b.1 > 4000000 {
                    continue;
                }
                hs.insert((b.0, b.1));
            }

            x0 -= 1;
            x1 += 1;
        }

        false
    }
}

fn main() -> Result<()> {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")?;

    let mut sensors = vec![];

    for line in io::stdin().lock().lines() {
        if let Some(caps) = re.captures(&line?) {
            let s = Sensor {
                location: (caps[1].parse()?, caps[2].parse()?),
                beacon: (caps[3].parse()?, caps[4].parse()?),
            };

            sensors.push(s);
        }
    }

    const ROW: i64 = 2000000;

    let mut sb = HashSet::new();
    for s in &sensors {
        sb.insert(s.location);
        sb.insert(s.beacon);
    }

    let mut min_x = *sb.iter().map(|(x, _)| x).min().unwrap();
    let mut max_x = *sb.iter().map(|(x, _)| x).max().unwrap();

    min_x = min_x - 1_000_000;
    max_x = max_x + 1_000_000;

    let mut counter = 0;

    'outer: for x in min_x..=max_x {
        for s in &sensors {
            if s.in_range((x, ROW)) {
                if !sb.contains(&(x, ROW)) {
                    counter += 1;
                    continue 'outer;
                }
            }
        }
    }

    println!("Part 1: {}", counter);

    let mut hs = HashSet::new();

    for s in &sensors {
        s.border_calc(&mut hs);
    }

    'outer: for p in &hs {
        for s in &sensors {
            if s.in_range(*p) {
                continue 'outer;
            }
        }
        println!("Part 2. {}", p.0 * 4000000 + p.1);
        break;
    }

    Ok(())
}
