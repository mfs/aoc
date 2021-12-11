use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut p1_score = 0;
    let mut p2_scores = vec![];

    'outer: for line in io::stdin().lock().lines() {
        let mut open = vec![];

        for c in line?.chars() {
            // save openers
            if ['(', '[', '{', '<'].contains(&c) {
                open.push(c);
                continue;
            }

            if [')', ']', '}', '>'].contains(&c) {
                if let Some(top) = open.pop() {
                    if map(top) != c {
                        p1_score += score(c).0;
                        continue 'outer;
                    }
                } else {
                    continue 'outer;
                }
            }
        }

        let p2_score = open.iter().rev().fold(0, |a, x| 5 * a + score(map(*x)).1);

        p2_scores.push(p2_score);
    }

    println!("Part 1: {}", p1_score);

    p2_scores.sort();
    println!("Part 2: {}", p2_scores[p2_scores.len() / 2]);

    Ok(())
}

fn score(b: char) -> (u64, u64) {
    match b {
        ')' => (3, 1),
        ']' => (57, 2),
        '}' => (1197, 3),
        '>' => (25137, 4),
        _ => panic!(),
    }
}

fn map(b: char) -> char {
    match b {
        '[' => ']',
        '(' => ')',
        '{' => '}',
        '<' => '>',
        _ => panic!(),
    }
}
