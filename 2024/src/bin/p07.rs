use std::io::{self, BufRead};

use anyhow::Result;
use rayon::prelude::*;

type Equations = Vec<Vec<i64>>;

fn main() -> Result<()> {
    let equations = parse()?;

    let part1 = equations.par_iter().map(|e| {
        if is_valid_dfs(e, false) {
            e[0]
        } else {
            0
        }
    }).sum::<i64>();

    println!("Part 1: {}", part1);

    let part2 = equations.par_iter().map(|e| {
        if is_valid_dfs(e, true) {
            e[0]
        } else {
            0
        }
    }).sum::<i64>();

    println!("Part 2: {}", part2);

    Ok(())
}

// naive dfs
// probably a faster way to do this going from right to left
// could probably rule out * based on if the solution % right == 0
// do something similar for concat and add
fn is_valid_dfs(equation: &[i64], part2: bool) -> bool {
    let left = equation[0];

    let mut stack = vec![];
    stack.push(equation[1..].to_vec());

    // process
    while let Some(e) = stack.pop() {
        // terminal
        if e.len() == 1 {
            if e[0] == left {
                return true;
            } else {
                continue;
            }
        }

        let mut ops = vec![|a: i64, b: i64| a * b, |a, b| a + b];

        if part2 {
            ops.push(|a, b| a * 10i64.pow(digits(b) as u32) + b);
        }

        for op in ops {
            let first = op(e[0], e[1]);
            if first > left {
                continue;
            }
            let mut v = vec![];
            v.push(first);
            v.extend_from_slice(&e[2..]);
            stack.push(v);
        }
    }

    false
}

fn digits(n: i64) -> u32 {
    if n < 10 {
        return 1
    } else if n < 100 {
        return 2
    } else if n < 1000 {
        return 3
    }

    unreachable!("too large n")
}

fn parse() -> Result<Equations> {
    let mut equations = vec![];

    for line in io::stdin().lock().lines() {
        let tokens: Vec<i64> = line?
            .split(' ')
            .map(|x| x.trim_matches(':').parse())
            .collect::<Result<_, _>>()?;
        equations.push(tokens);
    }

    Ok(equations)
}
