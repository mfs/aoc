use anyhow::Result;
use std::io::{self, BufRead};

const ROUND: char = 'O';
const _ROCK: char = '#';
const EMPTY: char = '.';

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    w: usize,
    h: usize,
}

fn main() -> Result<()> {
    let mut map: Map = parse()?;

    // part 1
    roll_north(&mut map);

    println!("Part 1: {}", load(&map));

    // part 2
    const CYCLES: usize = 1_000_000_000;
    const HISTORY: usize = 500; // increase if not working for a certain input
    const START: usize = HISTORY / 2;

    let mut loads = vec![];

    for _ in 0..HISTORY {
        // one cycle
        for _ in 0..4 {
            roll_north(&mut map);
            rotate(&mut map);
        }

        loads.push(load(&map));
    }

    for offset in 1.. {
        if loads[START..START + 3] == loads[START + offset..START + offset + 3] {
            println!("Part 2: {}", loads[START + ((CYCLES - START - 1) % offset)]);
            break;
        }
    }

    Ok(())
}

fn load(map: &Map) -> usize {
    (0..map.h)
        .rev()
        .enumerate()
        .map(|(idx, y)| (idx + 1) * map.grid[y].iter().filter(|&&c| c == ROUND).count())
        .sum::<usize>()
}

fn roll_north(map: &mut Map) {
    for y in 0..map.h {
        for x in 0..map.w {
            if map.grid[y][x] == ROUND {
                let mut ny = y;
                while ny > 0 && map.grid[ny - 1][x] == EMPTY {
                    ny -= 1;
                }

                if ny != y {
                    map.grid[y][x] = EMPTY;
                    map.grid[ny][x] = ROUND;
                }
            }
        }
    }
}

fn rotate(map: &mut Map) {
    // transpose
    for y in 0..map.h {
        for x in y + 1..map.w {
            let tmp = map.grid[y][x];
            map.grid[y][x] = map.grid[x][y];
            map.grid[x][y] = tmp;
        }
    }
    // reverse rows
    for y in 0..map.h {
        map.grid[y].reverse();
    }
}
fn parse() -> Result<Map> {
    let mut grid: Vec<_> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        grid.push(line.chars().collect::<Vec<_>>());
    }

    let w = grid[0].len();
    let h = grid.len();

    let map = Map { grid, w, h };

    Ok(map)
}
