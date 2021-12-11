use anyhow::Result;
use std::io::{self, BufRead};
use std::collections::HashSet;
use itertools::iproduct;

fn main() -> Result<()> {
    let mut grid = vec![];
    for line in io::stdin().lock().lines() {
        let r: Vec<_> = line?.chars().map(|x| x  as u64 - '0' as u64 ).collect();
        grid.push(r);
    }

    let mut grid1 = grid.clone();
    let p1: u64 = (0..100).map(|_| step(&mut grid1)).sum();

    println!("Part 1: {}", p1);

    let p2: usize = (0..).map(|_| step(&mut grid)).take_while(|&x| x < 100).count() + 1;

    println!("Part 2: {}", p2);

    Ok(())
}


fn step(grid: &mut Vec<Vec<u64>>) -> u64 {
    let width = grid[0].len();
    let height = grid.len();

    let mut work = vec![];

    // increase (track first flashes???
    for row in 0..width {
        for col in 0..height {
            grid[row][col] += 1;
            if grid[row][col] > 9 {
                work.push((row, col));
            }
        }
    }

    // process flashes
    let mut flashed = HashSet::new();

    // get next flash
    while let Some(next) = work.pop() {
        // mark as flashed
        if flashed.insert(next) {
            // new flasher
            let nbrs = neighbours(next.0 as i64, next.1 as i64, width as i64, height as i64);
            for n in nbrs {
                grid[n.0][n.1] += 1;
                if grid[n.0][n.1] > 9 && !flashed.contains(&n) {
                    work.push(n);
                }
            }
        }
    }

    // reset to zero
    for row in 0..width {
        for col in 0..height {
            if grid[row][col] > 9 {
                grid[row][col] = 0;
            }
        }
    }

    flashed.len() as u64
}

fn neighbours(row: i64, col: i64, width: i64, height: i64) -> Vec<(usize, usize)> {
    iproduct!(-1..=1, -1..=1)
        .filter(|(r, c)| *r != 0 || *c != 0) // not (0, 0)
        .map(|(r, c)| (row + r, col + c))
        .filter(|(r, c)| (0..height).contains(r) && (0..width).contains(c)) // bounds check
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}

