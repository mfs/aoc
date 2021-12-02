use anyhow::Result;
use std::io::{self, BufRead};

#[derive(Default)]
struct Sub {
    position: i64,
    depth: i64,
    aim: i64,
}

fn main() -> Result<()> {
    let mut sub1 = Sub::default();
    let mut sub2 = Sub::default();

    for line in io::stdin().lock().lines() {
        let cmd_x: Vec<_> = line?.split(' ').map(|x| x.to_owned()).collect();
        let x: i64 = cmd_x[1].parse()?;

        match &*cmd_x[0] {
            "forward" => {
                sub1.position += x;
                sub2.position += x;
                sub2.depth += sub2.aim * x
            }
            "down" => {
                sub1.depth += x;
                sub2.aim += x
            }
            "up" => {
                sub1.depth -= x;
                sub2.aim -= x
            }
            _ => panic!("error: unknown command"),
        }
    }

    println!("Part 1: {}", sub1.depth * sub1.position);
    println!("Part 2: {}", sub2.depth * sub2.position);

    Ok(())
}
