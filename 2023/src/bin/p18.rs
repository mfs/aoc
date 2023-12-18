use anyhow::Result;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

#[derive(Debug)]
struct Instruction {
    dir: [Direction; 2],
    len: [i64; 2],
}

fn main() -> Result<()> {
    let instructions = parse()?;

    for idx in 0..2 {
        println!("Part {}: {}", idx + 1, process(idx, &instructions));
    }

    Ok(())
}

fn process(idx: usize, instructions: &[Instruction]) -> i64 {
    let mut points = vec![];
    let mut boundary = 0;

    let mut loc = (0i64, 0i64);
    points.push(loc);

    for i in instructions {
        boundary += i.len[idx];
        match i.dir[idx] {
            Direction::Left => {
                loc.0 -= i.len[idx];
            }
            Direction::Right => {
                loc.0 += i.len[idx];
            }
            Direction::Up => {
                loc.1 -= i.len[idx];
            }
            Direction::Down => {
                loc.1 += i.len[idx];
            }
        }
        points.push(loc);
    }

    points.push(points[0]); // dup first point at end for (i + 1) wrap around

    // https://en.wikipedia.org/wiki/Shoelace_formula
    let area = points
        .windows(2)
        .map(|w| (w[0].1 + w[1].1) * (w[0].0 - w[1].0))
        .sum::<i64>()
        / 2;

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    let internal = area - boundary / 2 + 1;

    boundary + internal
}

fn parse() -> Result<Vec<Instruction>> {
    let mut instructions = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        let tokens: Vec<_> = line.split_whitespace().collect();

        let dir = match tokens[0] {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unreachable!(),
        };

        let len = tokens[1].parse()?;

        let hex = tokens[2].trim_matches(&['(', ')', '#'][..]);

        let dir_hex = match &hex[5..] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => unreachable!(),
        };

        let len_hex = i64::from_str_radix(&hex[0..5], 16)?;

        instructions.push(Instruction {
            dir: [dir, dir_hex],
            len: [len, len_hex],
        });
    }

    Ok(instructions)
}
