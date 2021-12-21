use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, BufRead};
use regex::Regex;
use itertools::iproduct;

// * track d100 and pos as 0-9 and 0-99 so we can use % arithmatic

type Cache = HashMap<(u64, u64, u64, u64), (u64, u64)>;

const WIN: u64 = 21;

#[derive(Debug, Default)]
struct Die {
    count: u64,
    state: u64,
}

impl Die {
    fn roll(&mut self) -> u64 {
        self.count += 1;

        let r = self.state + 1; // *

        self.state = (self.state + 1) % 100;

        r
    }
}

fn main() -> Result<()> {
    let re = Regex::new(r"Player (\d) starting position: (\d+)")?;

    let mut start: Vec<u64> = vec![];

    for line in io::stdin().lock().lines() {
        if let Some(caps) = re.captures(&line?) {
            start.push(caps[2].parse()?);
        }
    }

    println!("Part 1: {}", game(0, start[0] - 1, 0, start[1] - 1)); // *

    let mut cache: Cache = HashMap::new();

    let (a, b) = quantum(&mut cache, 0, start[0] - 1, 0, start[1] - 1); // *

    println!("Part 2: {}", std::cmp::max(a, b));

    Ok(())
}

fn game(mut p1_score: u64, mut p1_pos: u64, mut p2_score: u64, mut p2_pos: u64) -> u64 {
    let mut die = Die::default();

    loop {
        // player 1
        let roll = die.roll() + die.roll() + die.roll();
        p1_pos = (p1_pos + roll) % 10;
        p1_score += p1_pos + 1; // *

        if p1_score >= 1000 {
            return p2_score * die.count;
        }

        // player 2
        let roll = die.roll() + die.roll() + die.roll();
        p2_pos = (p2_pos + roll) % 10;
        p2_score += p2_pos + 1; // *

        if p2_score >= 1000 {
            return p1_score * die.count;
        }
    }
}

fn quantum(cache: &mut Cache, p1_score: u64, p1_pos: u64, p2_score: u64, p2_pos: u64) -> (u64, u64) {
    if p2_score >= WIN {
        return (0, 1);
    }

    if let Some(&s) = cache.get(&(p1_score, p1_pos, p2_score, p2_pos)) {
        return s;
    }

    let mut score = (0, 0);
    for die in iproduct!(1..=3, 1..=3, 1..=3).map(|(d0, d1, d2)| d0 + d1 + d2) {
        let p1_pos_next = (p1_pos + die) % 10;
        let (s1, s2) = quantum(cache, p2_score, p2_pos, p1_score + p1_pos_next + 1, p1_pos_next); // *
        score.0 += s2;
        score.1 += s1;
    }

    cache.insert((p1_score, p1_pos, p2_score, p2_pos), score);

    score
}
