use std::collections::HashSet;
use std::io::{self, BufRead};

use anyhow::{anyhow, Result};

type Grid = Vec<Vec<char>>;

type Dir = (i32, i32);

fn main() -> Result<()> {
    let (mut grid, start) = parse()?;

    let dir = (0, -1); // up

    let path = walk(&grid, start, dir).0.iter().map(|x| x.0).collect::<HashSet<_>>();
    println!("Part 1: {}", path.len());

    let mut count = 0;

    for loc in path.iter().filter(|&&x| x != start) {
        grid[loc.1 as usize][loc.0 as usize] = '#';

        if walk(&grid, start, dir).1 {
            count += 1;
        }

        grid[loc.1 as usize][loc.0 as usize] = '.';
    }

    println!("Part 2: {}", count);

    Ok(())
}

fn walk(grid: &Grid, mut loc: (i32, i32), mut dir: Dir) -> (HashSet<((i32, i32), (i32, i32))>, bool) {
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    let mut seen = HashSet::new();

    loop {
        if !seen.insert((loc, dir)) {
            return (seen, true); // looped
        }

        let nloc = (loc.0 + dir.0, loc.1 + dir.1);

        if nloc.0 < 0 || nloc.0 >= w || nloc.1 < 0 || nloc.1 >= h {
            break;
        }

        if grid[nloc.1 as usize][nloc.0 as usize] == '#' {
            let tmp = dir;
            dir = (-tmp.1, tmp.0); // turn right: (dx, dy) = (-dy, dx)
        } else {
            loc = nloc;
        }
    }

    (seen, false) // exited grid
}

fn parse() -> Result<(Grid, (i32, i32))> {
    let mut grid = Grid::new();

   for line in io::stdin().lock().lines() {
        grid.push(line?.chars().collect());
    }

    let w = grid[0].len();
    let h = grid.len();

    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == '^' {
                return Ok((grid, (x as i32, y as i32)));
            }
        }
    }

    Err(anyhow!("Missing start location."))
}
