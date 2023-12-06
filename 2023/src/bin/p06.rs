use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut lines = vec![];

    for line in io::stdin().lock().lines() {
        lines.push(line?);
    }

    let races: Vec<Vec<u64>> = lines
        .iter()
        .map(|l| l.split_whitespace().skip(1).map(|n| n.parse()).collect())
        .collect::<Result<_, _>>()?;

    let part_1: usize = std::iter::zip(&races[0], &races[1])
        .map(|(&t, &d)| race(t, d))
        .product();

    println!("Part 1: {}", part_1);

    let numbers: Vec<u64> = lines
        .iter()
        .map(|x| {
            x.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
        })
        .collect::<Result<_, _>>()?;

    println!("Part 2: {}", race(numbers[0], numbers[1]));

    Ok(())
}

// there are a few ways to speed this up, start in the middle and exit as soon
// as each direction starts failing to make distance. Probably possible to just
// calc the first and last times as well
fn race(time: u64, distance: u64) -> usize {
    (0..=time)
        .filter(|delay| (time - delay) * delay > distance)
        .count()
}
