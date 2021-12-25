use anyhow::Result;
use std::io::{self, BufRead};

type Map = Vec<Vec<char>>;

fn main() -> Result<()> {
    let mut map: Map = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        map.push(line.chars().collect());
    }

    let p1 = (0..).map(|_| step(&mut map)).take_while(|&x| x != 0).count() + 1;

    println!("Part 1: {}", p1);

    Ok(())
}

fn step_herd(map: &mut Map, cucumber: char, delta: (usize, usize)) -> usize {
    let width = map[0].len();
    let height = map.len();

    let mut moves = vec![];

    // calc moves
    for y in 0..height {
        for x in 0..width {
            // check dst square
            if map[y][x] == cucumber && map[(y + delta.0) % height][(x + delta.1) % width] == '.' {
                moves.push((y, x));
            }
        }
    }

    // do moves
    for m in &moves {
        map[m.0][m.1] = '.';
        map[(m.0 + delta.0) % height][(m.1 + delta.1) % width] = cucumber;
    }

    moves.len()
}

fn step(map: &mut Map) -> usize {
    step_herd(map, '>', (0, 1)) + step_herd(map, 'v', (1, 0))
}
