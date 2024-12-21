use std::io::{self, BufRead};
use std::collections::{HashMap,HashSet};

use anyhow::Result;
use rayon::prelude::*;

type V = (i32, i32);
type Grid = Vec<Vec<char>>;
type Path = HashMap<V, i32>;

fn main() -> Result<()> {
    let (grid, start, end) = parse()?;

    let path = bfs(&grid, start, end);

    println!("Part 1: {}", cheat(&path, 2));

    println!("Part 2: {}", cheat(&path, 20));

    Ok(())
}

fn cheat(path: &Path, radius: i32) -> i32 {
    let dia = diamond(radius);

    path.par_iter().map(|(pos, cost)| {
        let mut count = 0;
        for dst in &dia {
            let dst = (pos.0 + dst.0, pos.1 + dst.1);
            let md = (pos.0 - dst.0).abs() + (pos.1 - dst.1).abs();
            if path.contains_key(&dst) && cost - path[&dst] >= 100 + md {
                count += 1;
            }
        }

        count
    }).sum::<i32>()
}

fn diamond(radius: i32) -> HashSet<V> {
    let mut d = HashSet::new();

    let mut xr = radius;
    let mut yr = 0;

    while xr >= 0 {
        for x in -xr..=xr {
            d.insert((x, yr));
            d.insert((x, -yr));
        }
        yr += 1;
        xr -= 1;
    }

    d
}

fn bfs(grid: &Grid, start: V, end: V) -> HashMap<V, i32> {
    let mut path = HashMap::new();

    let (mut pos, mut cost) = (start, 0);

    loop  {
        path.insert(pos, cost);

        if pos == end {
            break;
        }

        for dir in [(1, 0), (-1, 0), (0, -1), (0, 1)] {
            let n = (pos.0 + dir.0, pos.1 + dir.1);

            if path.contains_key(&n) {
                continue;
            }

            if grid[n.1 as usize][n.0 as usize] != '#' {
                path.insert(n, cost + 1);
                (pos, cost) = (n, cost + 1);
                break;
            }
        }
    }

    path
}

fn parse() -> Result<(Grid, V, V)> {
    let mut grid = Grid::new();

    for line in io::stdin().lock().lines() {
        grid.push(line?.chars().collect());
    }

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                start = (x as i32, y as i32);
            } else if c == 'E' {
                end = (x as i32, y as i32);
            }
        }
    }

    Ok((grid, start, end))
}
