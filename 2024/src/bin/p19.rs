use std::collections::HashMap;
use std::io::{self, BufRead};

use anyhow::Result;

type Cache = HashMap<String, u64>;

fn main() -> Result<()> {
    let (towels, designs) = parse()?;

    println!("Part 1: {}", designs.iter().filter(|&d| dfs(d, &towels)).count());

    let part2 = designs
        .iter()
        .map(|d| dfs_cached(d, &towels, &mut Cache::new()))
        .sum::<u64>();

    println!("Part 2: {}", part2);

    Ok(())
}

fn dfs(design: &str, towels: &[String]) -> bool {
    if towels.iter().any(|x| x == design) {
        return true;
    }

    for t in towels {
        if let Some(s) = design.strip_prefix(t) {
            if dfs(s, towels) {
                return true;
            }
        }
    }

    false
}

fn dfs_cached(design: &str, towels: &[String], cache: &mut Cache) -> u64 {
    if cache.contains_key(design) {
        return cache[design];
    }

    let total = if design == "" {
        1
    } else {
        let mut count = 0;

        for t in towels {
            if let Some(s) = design.strip_prefix(t) {
                count += dfs_cached(s, towels, cache);
            }
        }
        count
    };

    cache.insert(design.to_owned(), total);
    total
}

fn parse() -> Result<(Vec<String>, Vec<String>)> {
    let mut towels: Vec<String> = vec![];
    let mut designs = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.contains(',') {
            towels = line.split(',').map(|s| s.trim().to_owned()).collect();
        } else if line != "" {
            designs.push(line.to_owned());
        }
    }

    Ok((towels, designs))
}
