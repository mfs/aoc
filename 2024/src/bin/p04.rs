use std::io::{self, BufRead};

use anyhow::Result;

type Grid = Vec<Vec<char>>;

fn main() -> Result<()> {
    let mut grid = Grid::new();

    for line in io::stdin().lock().lines() {
        grid.push(line?.chars().collect());
    }

    let part1 = xmas(&grid);

    println!("Part 1: {}", part1);

    let part2 = cross(&grid);

    println!("Part 2: {}", part2);

    Ok(())
}

fn cross(g: &Grid) -> u32 {
    let w = g[0].len();
    let h = g.len();

    let mut count = 0;

    for y in 1..h-1 {
        for x in 1..w-1 {
            // if not a continue
            if g[y][x] != 'A' {
                continue;
            }

            // a b
            //  x
            // c d

            let (a, b, c, d) = (g[y-1][x-1], g[y-1][x+1], g[y+1][x-1], g[y+1][x+1]);

            match (a, b, c, d) {
                ('M', 'M', 'S', 'S') => count += 1,
                ('S', 'M', 'S', 'M') => count += 1,
                ('M', 'S', 'M', 'S') => count += 1,
                ('S', 'S', 'M', 'M') => count += 1,
                _ => {},
            }
        }
    }

    count
}

fn xmas(g: &Grid) -> u32 {
    let w = g[0].len() as i32;
    let h = g.len() as i32;

    let mut count = 0;

    const DIRECTIONS: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)];

    let grid = |y: i32, x: i32| g[y as usize][x as usize];

    for y in 0..h {
        for x in 0..w {
            for dir in DIRECTIONS {
                // check end point is in bounds
                let end = (x + 3 * dir.0, y + 3 * dir.1);
                if end.0 < 0 || end.0 >= w || end.1 < 0 || end.1 >= h {
                    continue;
                }

                // check for XMAS
                let (a, b, c, d) = (
                    grid(y, x),
                    grid(y + dir.1, x + dir.0),
                    grid(y + 2 * dir.1, x + 2 * dir.0),
                    grid(y + 3 * dir.1, x + 3 * dir.0),
                );

                if a == 'X' && b == 'M' && c == 'A' && d == 'S' {
                    count += 1;
                }
            }
        }
    }

    count
}
