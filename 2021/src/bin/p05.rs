use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, BufRead};

struct Line {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

fn main() -> Result<()> {
    let mut lines = vec![];

    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)")?;

    for line in io::stdin().lock().lines() {
        if let Some(caps) = re.captures(&line?) {
            let line = Line {
                x1: caps[1].parse()?,
                y1: caps[2].parse()?,
                x2: caps[3].parse()?,
                y2: caps[4].parse()?,
            };

            lines.push(line);
        }
    }

    println!("Part 1: {}", process(&lines, false));
    println!("Part 2: {}", process(&lines, true));

    Ok(())
}

fn process(lines: &Vec<Line>, include_diagonals: bool) -> usize {
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();

    for line in lines {
        let x_delta = line.x2 - line.x1;
        let y_delta = line.y2 - line.y1;

        if !include_diagonals && x_delta != 0 && y_delta != 0 {
            continue;
        }

        let x_step = x_delta / std::cmp::max(x_delta.abs(), 1);
        let y_step = y_delta / std::cmp::max(y_delta.abs(), 1);

        let mut x = line.x1;
        let mut y = line.y1;

        loop {
            *map.entry((x, y)).or_insert(0) += 1;

            if x == line.x2 && y == line.y2 {
                break;
            }

            x += x_step;
            y += y_step;
        }
    }

    map.values().filter(|&x| *x >= 2).count()
}
