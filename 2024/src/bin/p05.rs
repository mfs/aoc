use std::io::{self, BufRead};
use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;

type Rules = Vec<(u32, u32)>;
type Update = Vec<u32>;
type Updates = Vec<Update>;

fn main() -> Result<()> {
    let mut rules = Rules::new();
    let mut updates = Updates::new();

    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.contains("|") {
            let x: Vec<_> = line
                .split('|')
                .map(u32::from_str)
                .collect::<Result<_, _>>()?;

            rules.push((x[0], x[1]));
        } else if line.contains(',') {
            let x: Vec<_> = line
                .split(',')
                .map(u32::from_str)
                .collect::<Result<_, _>>()?;

            updates.push(x);
        }
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for update in &updates {
        if is_correct(update, &rules) {
            part1 += update[update.len() / 2];
        } else {
            let u = make_correct(update, &rules);
            part2 += u[u.len() / 2];
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn make_correct(update: &Update, rules: &Rules) -> Update {
    let mut correct = Update::new();

    'outer: for n in update {
        for i in 0..(correct.len() + 1) {
            let mut new_correct = correct.clone();

            new_correct.insert(i, *n);

            if is_correct(&new_correct, rules) {
                correct = new_correct;
                continue 'outer;
            }
        }
    }

    correct
}

fn is_correct(update: &Update, rules: &Rules) -> bool {
    let mut m = HashMap::new();
    for (i, n) in update.iter().enumerate() {
        m.insert(n, i);
    }

    for (a, b) in rules {
        if update.contains(a) && update.contains(b) {
            if m[a] >= m[b] {
                return false;
            }
        }
    }

    true
}
