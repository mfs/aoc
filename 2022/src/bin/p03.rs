use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let rucksacks: Vec<_> = io::stdin().lock().lines().filter_map(Result::ok).collect();

    let mut score = 0;

    for rucksack in &rucksacks {
        let (first, second) = rucksack.split_at(rucksack.len() / 2);

        let mut first: HashSet<_> = first.chars().collect();
        let second: HashSet<_> = second.chars().collect();

        first.retain(|r| second.contains(r));

        score += first.iter().map(|i| priority(*i)).sum::<u64>();
    }

    println!("Part 1: {}", score);

    let mut score = 0;

    for tri in rucksacks.chunks(3) {
        let mut first: HashSet<_> = tri[0].chars().collect();
        let second: HashSet<_> = tri[1].chars().collect();
        let third: HashSet<_> = tri[2].chars().collect();

        first.retain(|r| second.contains(r));
        first.retain(|r| third.contains(r));

        score += first.iter().map(|i| priority(*i)).sum::<u64>();
    }

    println!("Part 2: {}", score);

    Ok(())
}

fn priority(ch: char) -> u64 {
    if ch.is_ascii_lowercase() {
        ch as u64 - 'a' as u64 + 1
    } else {
        ch as u64 - 'A' as u64 + 27
    }
}
