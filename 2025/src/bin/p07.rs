use std::io::{self, BufRead};
use std::collections::BTreeSet;
use std::collections::HashMap;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let (grid, start) = parse()?;
    let w = grid[0].len();
    let h = grid.len();

    let mut beams = HashMap::from([(start.0, 1)]); // initial beam

    let mut splitters_seen = BTreeSet::new();

    for y in 0..h {
        let mut next_beams = HashMap::new();

        for (x, beam_count) in beams {
            if grid[y][x] == '^' {
                // track seen splitters
                splitters_seen.insert((x, y));

                // split beam
                if x > 0 {
                    *next_beams.entry(x - 1).or_insert(0) += beam_count;
                }
                if x < w-1 {
                    *next_beams.entry(x + 1).or_insert(0) += beam_count;
                }
            } else {
                // fall through
                *next_beams.entry(x).or_insert(0) += beam_count;
            }
        }

        beams = next_beams;
    }

    println!("Part 1: {}", splitters_seen.len());
    println!("Part 2: {}", beams.values().sum::<u64>());

    Ok(())
}

fn parse() -> Result<(Vec<Vec<char>>, (usize, usize))> {
    let mut grid = vec![];
    let mut start = None; (0, 0);

    for (y, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        if let Some(x) = line.chars().position(|c| c == 'S') {
            start = Some((x, y));
        }
        grid.push(line.chars().collect());
    }

    Ok((grid, start.ok_or(anyhow!("missing start marker"))?))
}
