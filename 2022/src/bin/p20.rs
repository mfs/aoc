use anyhow::{anyhow, Result};
use std::io::{self, BufRead};

const ENCRYPTION_KEY: i64 = 811589153;

#[derive(Debug, Clone, Copy)]
struct Node {
    num: i64,
    idx: usize,
}

fn main() -> Result<()> {
    let mut numbers: Vec<Node> = vec![];

    for (i, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        numbers.push(Node {
            num: line.parse()?,
            idx: i,
        });
    }

    println!("Part 1: {}", mix(&numbers, 1, 1)?);

    println!("Part 2: {}", mix(&numbers, ENCRYPTION_KEY, 10)?);

    Ok(())
}

fn mix(numbers: &Vec<Node>, key: i64, n: usize) -> Result<i64> {
    let mut numbers: Vec<_> = numbers
        .iter()
        .map(|n| Node {
            num: n.num * key,
            idx: n.idx,
        })
        .collect();

    let l = numbers.len();

    for _ in 0..n {
        for idx in 0..l {
            // find pos
            let pos = numbers
                .iter()
                .position(|n| n.idx == idx)
                .ok_or(anyhow!("missing index"))?;

            // rotate to end
            numbers.rotate_right(l - pos - 1);

            // pop number
            let n = numbers.pop().ok_or(anyhow!("no numbers"))?;

            // rotate idx to last
            if n.num < 0 {
                numbers.rotate_right(n.num.abs() as usize % (l - 1));
            } else if n.num > 0 {
                numbers.rotate_left(n.num as usize % (l - 1));
            }
            numbers.push(n);
        }
    }

    let pos0 = numbers
        .iter()
        .position(|e| e.num == 0)
        .ok_or(anyhow!("missing zero"))?;

    let sum: i64 = [1000, 2000, 3000]
        .iter()
        .map(|x| numbers[(pos0 + x) % l].num)
        .sum();

    Ok(sum)
}
