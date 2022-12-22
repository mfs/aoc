use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, BufRead};

enum Monkey {
    Num(i64),
    Op(String, Op, String),
}

enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

type Tree = HashMap<String, Monkey>;

const ROOT: &str = "root";
const ME: &str = "humn";

fn main() -> Result<()> {
    let mut tree = Tree::new();

    for line in io::stdin().lock().lines() {
        let line = line?;

        let tokens: Vec<_> = line.split(' ').collect();

        if tokens.len() == 2 {
            tree.insert(
                tokens[0].trim_end_matches(':').to_owned(),
                Monkey::Num(tokens[1].parse()?),
            );
        } else {
            let m = match tokens[2] {
                "+" => Monkey::Op(tokens[1].to_owned(), Op::Add, tokens[3].to_owned()),
                "-" => Monkey::Op(tokens[1].to_owned(), Op::Sub, tokens[3].to_owned()),
                "*" => Monkey::Op(tokens[1].to_owned(), Op::Mul, tokens[3].to_owned()),
                "/" => Monkey::Op(tokens[1].to_owned(), Op::Div, tokens[3].to_owned()),
                _ => unreachable!(),
            };

            tree.insert(tokens[0].trim_end_matches(":").to_owned(), m);
        }
    }

    println!("Part 1: {}", listen(ROOT, &tree));

    println!("Part 2: {}", solve(ROOT, 0, &tree));

    Ok(())
}

fn solve(monkey: &str, value: i64, tree: &Tree) -> i64 {
    if monkey == ME {
        return value;
    }

    let (m0, m1) = match tree.get(monkey) {
        Some(Monkey::Op(m0, _, m1)) => (m0, m1),
        _ => unreachable!(),
    };

    let (known, unknown) = if is_known(m0, &tree) {
        (m0, m1)
    } else {
        (m1, m0)
    };

    if monkey == ROOT {
        return solve(unknown, listen(known, &tree), &tree);
    }

    match tree.get(monkey) {
        Some(Monkey::Op(_, Op::Add, _)) => solve(unknown, value - listen(known, tree), tree),
        Some(Monkey::Op(m0, Op::Sub, m1)) => {
            if m0 == known {
                solve(m1, listen(m0, tree) - value, tree)
            } else {
                solve(m0, value + listen(m1, tree), tree)
            }
        }
        Some(Monkey::Op(_, Op::Mul, _)) => solve(unknown, value / listen(known, tree), tree),
        Some(Monkey::Op(m0, Op::Div, m1)) => {
            if m0 == known {
                solve(m1, listen(m0, tree) / value, tree)
            } else {
                solve(m0, value * listen(m1, tree), tree)
            }
        }
        Some(Monkey::Num(_)) => unreachable!(),
        None => unreachable!(),
    }
}

fn is_known(monkey: &str, tree: &Tree) -> bool {
    match tree.get(monkey) {
        Some(Monkey::Num(_)) => monkey != ME,
        Some(Monkey::Op(m0, _, m1)) => is_known(m0, tree) && is_known(m1, tree),
        None => unreachable!(),
    }
}

fn listen(monkey: &str, tree: &Tree) -> i64 {
    match tree.get(monkey) {
        Some(Monkey::Num(n)) => *n,
        Some(Monkey::Op(m0, Op::Add, m1)) => listen(m0, tree) + listen(m1, tree),
        Some(Monkey::Op(m0, Op::Sub, m1)) => listen(m0, tree) - listen(m1, tree),
        Some(Monkey::Op(m0, Op::Mul, m1)) => listen(m0, tree) * listen(m1, tree),
        Some(Monkey::Op(m0, Op::Div, m1)) => listen(m0, tree) / listen(m1, tree),
        None => unreachable!(),
    }
}
