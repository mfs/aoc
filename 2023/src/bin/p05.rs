use anyhow::{anyhow, Result};
use std::io::{self, BufRead};

type Range = (u64, u64, u64);
type Ranges = Vec<Range>;
type Almanac = Vec<Ranges>;

fn main() -> Result<()> {
    let mut lines = vec![];

    for line in io::stdin().lock().lines() {
        lines.push(line?);
    }

    let seeds = lines[0]
        .split(':')
        .nth(1)
        .ok_or(anyhow!("error parsing seed"))?
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut almanac: Almanac = vec![];
    let mut ranges: Ranges = vec![];

    // line by line parser
    for line in &lines[1..] {
        if line == "" {
            continue;
        }

        if line.contains(':') {
            // add prev if not empty
            if !ranges.is_empty() {
                almanac.push(ranges);
            }
            // make new map
            ranges = vec![];
        } else {
            // add to current map
            let numbers = line
                .split_whitespace()
                .map(|x| x.parse::<u64>())
                .collect::<Result<Vec<_>, _>>()?;

            ranges.push((numbers[0], numbers[1], numbers[2]));
        }
    }

    almanac.push(ranges.clone());

    let mut locations = vec![];
    for &seed in &seeds {
        locations.push(al(seed, &almanac));
    }

    println!(
        "Part 1: {}",
        locations.iter().min().ok_or(anyhow!("empty locations"))?
    );

    let mut part2 = u64::MAX;

    // part 2 brute force solution, runs in 1m35s on my desktop
    // Proper solution would be to rewrite al() to take a range
    // and then split the range into sub rnages that overlap the
    // next map's ranges, etc. Then return the minimum of the ranges
    // Need to take into account ranges that don't overlap
    for v in seeds.chunks(2) {
        for seed in v[0]..(v[0] + v[1]) {
            part2 = std::cmp::min(al(seed, &almanac), part2);
        }
    }

    println!("Part 2: {}", part2);

    Ok(())
}

fn al(mut seed: u64, almanac: &Almanac) -> u64 {
    // dest, source, len
    'outer: for ranges in almanac {
        for range in ranges {
            if seed >= range.1 && seed < (range.1 + range.2) {
                seed = range.0 + (seed - range.1);
                continue 'outer;
            }
        }
    }

    seed
}
