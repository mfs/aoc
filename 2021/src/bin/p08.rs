use anyhow::{anyhow, Result};
use std::io::{self, BufRead};
use std::collections::BTreeSet;
use std::collections::HashMap;

// Initially did this via brute force using Itertools::permutations() as 7!
// is only 5040. This worked but was a bit unwieldy. Came across this
// shortcut on /r/adventofcode but now can't find the author. It works
// by using the fact that each digit is uniquely identifiable by looking
// at the length and the intersection of the segments with two of the
// known digits.

fn main() -> Result<()> {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in io::stdin().lock().lines() {
        let a: Vec<_> = line?.split(" | ").map(|x| x.to_owned()).collect();

        let patterns: Vec<BTreeSet<char>> = a[0].trim().split(' ').map(|x| x.chars().collect()).collect();
        let values: Vec<BTreeSet<char>> = a[1].trim().split(' ').map(|x| x.chars().collect()).collect();

        part1 += values.iter().filter(|x| [2, 4, 3, 7].contains(&x.len())).count();

        let one = patterns.iter().find(|x| x.len() == 2).ok_or(anyhow!("missing 1 pattern"))?;
        let four = patterns.iter().find(|x| x.len() == 4).ok_or(anyhow!("missing 4 pattern"))?;

        let mut map = HashMap::new();

        for pattern in &patterns {
            let one_int = one.intersection(&pattern).count();
            let four_int = four.intersection(&pattern).count();

            let digit = match (pattern.len(), one_int, four_int) {
                (2, _, _) => 1,
                (3, _, _) => 7,
                (4, _, _) => 4,
                (5, 2, _) => 3,
                (5, _, 2) => 2,
                (5, _, _) => 5,
                (6, 1, _) => 6,
                (6, _, 4) => 9,
                (6, _, _) => 0,
                (7, _, _) => 8,
                _ => unreachable!(),
            };

            map.insert(pattern, digit);
        }

        part2 += values.iter().fold(0, |acc, n| acc * 10 + map[n]);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

