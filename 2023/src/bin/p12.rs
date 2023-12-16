use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, BufRead};

const OPERATIONAL: char = '.';
const DAMAGED: char = '#';
const UNKNOWN: char = '?';

type Cache = HashMap<(String, Vec<u64>), u64>;
fn main() -> Result<()> {
    let rows = parse()?;

    let mut cache: Cache = HashMap::new();

    let p1: u64 = rows.iter().map(|r| arrange(&r.0, &r.1, &mut cache)).sum();

    println!("Part 1: {}", p1);

    let mut unfolded = vec![];
    for (pattern, counts) in &rows {
        let pattern_unfolded: String = std::iter::repeat(pattern.clone()).take(5).join("?");

        let counts_unfolded: Vec<u64> = std::iter::repeat(counts.clone())
            .take(5)
            .flatten()
            .collect::<Vec<_>>();

        unfolded.push((pattern_unfolded, counts_unfolded));
    }

    let p2: u64 = unfolded
        .iter()
        .map(|r| arrange(&r.0, &r.1, &mut cache))
        .sum();

    println!("Part 2: {}", p2);

    Ok(())
}

fn arrange(pattern: &str, groups: &[u64], cache: &mut Cache) -> u64 {
    if let Some(x) = cache.get(&(pattern.to_owned(), groups.to_vec())) {
        return *x;
    }

    // ==================== Terminal Cases ====================

    // if we are out of groups this might be still a vaild pattern if none of
    // the remaining springs are DAMAGED otherwise it's not valid
    // [..?] () is valid, [..#.] () is invalid
    if groups.len() == 0 {
        if pattern.chars().all(|s| s != DAMAGED) {
            return 1;
        } else {
            return 0;
        }
    }

    // if we have one group left which is the same length as our pattern
    // and the pattern contains no OPERATIONAL springs this is a valid
    // pattern otherwise it's not valid
    // [##?#] (4) is valid, [##.#] (4) is invalid
    if groups.len() == 1 && pattern.len() == groups[0] as usize {
        if pattern.chars().all(|s| s != OPERATIONAL) {
            return 1;
        } else {
            return 0;
        }
    }

    // if the sum of the remaining groups is greater than the pattern length
    // the groups can not fit and this pattern is invalid
    if groups.iter().sum::<u64>() > pattern.len() as u64 {
        return 0;
    }

    // ==================== Recursive Cases ====================

    // skip operation springs until we find a potential starting location for
    // the next group
    if let Some(OPERATIONAL) = pattern.chars().nth(0) {
        return arrange(&pattern[1..], groups, cache);
    }

    // if current spring is unknown we have two potential cases

    // 1: treat it as operational and skip as above
    let skip_cases = if let Some(UNKNOWN) = pattern.chars().nth(0) {
        arrange(&pattern[1..], groups, cache)
    } else {
        0
    };

    // 2: treat it as damaged and potentially part of the next group

    // if any springs in the next potential group are operational, cache and
    // return the skip_cases only
    if pattern[..groups[0] as usize]
        .chars()
        .any(|c| c == OPERATIONAL)
    {
        cache.insert((pattern.to_owned(), groups.to_vec()), skip_cases);
        return skip_cases;
    }

    // if the next spring after the current group is not damaged, this is a valid
    // pattern, move forward to the next group
    let match_cases = if let Some(OPERATIONAL | UNKNOWN) = pattern.chars().nth(groups[0] as usize) {
        arrange(&pattern[(groups[0] as usize + 1)..], &groups[1..], cache)
    } else {
        0
    };

    cache.insert(
        (pattern.to_owned(), groups.to_vec()),
        skip_cases + match_cases,
    );

    skip_cases + match_cases
}

fn parse() -> Result<Vec<(String, Vec<u64>)>> {
    let mut rows: Vec<(String, Vec<u64>)> = vec![];
    for line in io::stdin().lock().lines() {
        let line = line?;

        let left_right: Vec<_> = line.split_whitespace().collect();

        rows.push((
            left_right[0].chars().collect(),
            left_right[1]
                .split(',')
                .map(|s| s.parse())
                .collect::<Result<Vec<u64>, _>>()?,
        ));
    }

    Ok(rows)
}
