use std::collections::VecDeque;
use std::collections::HashSet;
use std::io::{self, BufRead};

use anyhow::Result;
use itertools::iproduct;

type V = (i32, i32);
type Grid = Vec<Vec<i32>>;

trait GridExt {
    fn cell(&self, x: i32, y:i32) -> i32;
}

impl GridExt for Grid {
    fn cell(&self, x: i32, y:i32) -> i32 {
        self[y as usize][x as usize]
    }
}

fn main() -> Result<()> {
    let (grid, w, h) = parse()?;

    let trailheads: Vec<V> = iproduct!(0..w, 0..h)
        .filter(|&(x, y)| grid[y as usize][x as usize] == 0)
        .collect();

    let part1: usize = trailheads
        .iter()
        .map(|&th| bfs(th, &grid, w, h, true))
        .sum();

    println!("Part 1: {}", part1);

    let part2: usize = trailheads
        .iter()
        .map(|&th| bfs(th, &grid, w, h, false))
        .sum();

    println!("Part 2: {}", part2);

    Ok(())
}

fn bfs(start: V, grid: &Grid, w: i32, h: i32, track_seen: bool) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    seen.insert(start);
    queue.push_back(start);

    let mut score = 0;

    while let Some(v) = queue.pop_front() {
        let height = grid.cell(v.0, v.1);
        if height == 9 {
            score += 1;
            continue;
        }

        for d in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let n = (v.0 + d.0, v.1 + d.1);

            if track_seen && seen.contains(&n) {
                continue;
            }

            if n.0 >= 0 && n.0 < w && n.1 >= 0 && n.1 < h && grid.cell(n.0, n.1) == height + 1 {
                seen.insert(n);
                queue.push_back(n);
            }
        }
    }

    score
}

fn parse() -> Result<(Grid, i32, i32)> {
    let mut grid = Grid::new();

    for line in io::stdin().lock().lines() {
        let cells = line?.chars().map(|d| d.to_digit(10).unwrap() as i32).collect();
        grid.push(cells);
    }

    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    Ok((grid, w, h))
}
