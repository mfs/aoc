use std::io::{self, BufRead};

use anyhow::{Result,anyhow};

fn main() -> Result<()> {
    let (tiles, regions) = parse()?;

    let mut count = 0;

    for r in &regions {
        let used = std::iter::zip(tiles.iter(), r.qs.iter())
            .map(|(a, b)| a * b)
            .sum::<u32>();

        if (r.w * r.h) > used {
            count += 1;
        }
    }

    println!("Part 1: {}", count);

    Ok(())
}

#[derive(Debug)]
struct Region {
    w: u32,
    h: u32,
    qs: Vec<u32>,
}

fn parse() -> Result<(Vec<u32>, Vec<Region>)> {
    let mut tiles = vec![];
    let mut regions = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.contains('x') {
            let ns: Vec<u32> = line
                .split(&['x', ' '])
                .map(|s| s.trim_matches(':').parse())
                .collect::<Result<_,_>>()?;

            regions.push(
                Region {
                    w: ns[0],
                    h: ns[1],
                    qs: ns[2..].to_vec(),
                }
            );
        } else if line.contains(':') {
            tiles.push(0);
        } else if line.contains(&['#', '.']) {
            let c = line.chars().filter(|&c| c == '#').count();
            *tiles.last_mut().ok_or(anyhow!("missing tile index"))? += c as u32;
        }
    }

    Ok((tiles, regions))
}
