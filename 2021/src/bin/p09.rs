use anyhow::Result;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() -> Result<()> {
    let mut heightmap = vec![];
    for line in io::stdin().lock().lines() {
        let r: Vec<_> = line?.chars().map(|x| x  as u64 - '0' as u64 ).collect();
        heightmap.push(r);
    }

    let width = heightmap[0].len();
    let height = heightmap.len();

    let mut risk_level = 0;
    let mut basins = vec![];

    for row in 0..height {
        for col in 0..width {
            if is_low_point(&heightmap, row, col) {
                risk_level += heightmap[row][col] + 1;
                basins.push((row, col));
            }
        }
    }

    println!("Part 1: {}", risk_level);

    let mut basin_sizes = vec![];

    for b in basins {
        basin_sizes.push(basin(&heightmap, b.0, b.1));
    }

    basin_sizes.sort_by(|a, b| b.cmp(a)); // reverse

    let p2: usize = basin_sizes.iter().take(3).product();

    println!("Part 2: {}", p2);

    Ok(())
}

fn basin(heightmap: &Vec<Vec<u64>>, row: usize, col: usize) -> usize {
    let mut work = vec![];
    // start with low point
    work.push((row, col));

    let mut basin = HashSet::new();

    // get next point
    while let Some(p) = work.pop() {
        // add to basin
        basin.insert(p);
        // get neighbours
        let n = neighbours(&heightmap, p.0 as i64, p.1 as i64);
        // process neighbours
        for pt in n {
            // if it's a 9, continue
            if heightmap[pt.0][pt.1] == 9 {
                continue;
            }
            // if not in basin, add to work queue
            if !basin.contains(&pt) {
                work.push(pt);
            }
        }
    }

    basin.len()
}

fn neighbours(heightmap: &Vec<Vec<u64>>, row: i64, col: i64) -> Vec<(usize, usize)> {
    let width = heightmap[0].len() as i64;
    let height = heightmap.len() as i64;

    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .map(|(r,c)| (row + r, col + c))
        .filter(|&(r, c)| r >= 0 && r < height && c >= 0 && c < width )
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}

fn is_low_point(heightmap: &Vec<Vec<u64>>, row: usize, col:usize) -> bool {
    for pt in neighbours(heightmap, row as i64, col as i64) {
        if heightmap[row][col] >= heightmap[pt.0][pt.1] {
            return false;
        }
    }

    true
}
