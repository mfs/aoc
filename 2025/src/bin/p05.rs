use std::io::{self, BufRead};

use anyhow::Result;

fn main() -> Result<()> {
    let (fresh, ingredients) = parse()?;

    let mut count = 0;

    'outer: for id in &ingredients {
        for (min, max) in &fresh {
            if id >= min && id <= max {
                count += 1;
                continue 'outer;
            }
        }
    }

    println!("Part 1: {}", count);

    let mut intervals = fresh.clone();
    intervals.sort_unstable();

    let mut count = 0;
    let mut cur_max = 0;

    // process one interval at a time sorted by min/start.
    for (mut intvl_min, intvl_max) in intervals {
        // trim interval if it overlaps
        if cur_max >= intvl_min {
            intvl_min = cur_max + 1;
        }
        // do we still have a valid interval?
        if intvl_min <= intvl_max {
            count += intvl_max - intvl_min + 1;
        }
        // move cur_max forward if needed
        cur_max = std::cmp::max(cur_max, intvl_max);
    }

    println!("Part 2: {}", count);

    Ok(())
}

fn parse() -> Result<(Vec<(u64, u64)>, Vec<u64>)> {
    let mut fresh = vec![];
    let mut ingredients = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.contains('-') {
            let tokens: Vec<_> = line.split('-').collect();
            fresh.push((tokens[0].parse()?, tokens[1].parse()?));
        } else if let Ok(n) = line.parse::<u64>() {
            ingredients.push(n);
        }
    }

    Ok((fresh, ingredients))
}
