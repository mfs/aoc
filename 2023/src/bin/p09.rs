use anyhow::Result;
use std::io::{self, BufRead};

enum Dir {
    Forward,
    Backward,
}

fn main() -> Result<()> {
    let mut reports = vec![];

    for line in io::stdin().lock().lines() {
        let history: Vec<i64> = line?
            .split_whitespace()
            .map(|x| x.parse())
            .collect::<Result<_, _>>()?;

        reports.push(history);
    }

    let part1: i64 = reports.iter().map(|h| complete(h, Dir::Forward)).sum();

    println!("Part 1: {}", part1);

    let part2: i64 = reports.iter().map(|h| complete(h, Dir::Backward)).sum();

    println!("Part 2: {}", part2);

    Ok(())
}

fn complete(history: &[i64], dir: Dir) -> i64 {
    let (mut last_numbers, f): (_, fn(i64, i64) -> i64) = match dir {
        Dir::Forward => (vec![history[history.len() - 1]], |acc, x| acc + x),
        Dir::Backward => (vec![history[0]], |acc, x| x - acc),
    };

    let mut row = history.to_vec();

    loop {
        let next = row.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        if next.iter().all(|x| *x == 0) {
            break;
        }

        match dir {
            Dir::Forward => last_numbers.push(next[next.len() - 1]),
            Dir::Backward => last_numbers.push(next[0]),
        }

        row = next;
    }

    last_numbers.into_iter().rev().fold(0, f)
}
