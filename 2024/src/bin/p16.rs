use std::collections::{BinaryHeap, HashSet, HashMap};
use std::cmp::Reverse;
use std::io::{self, BufRead};

use anyhow::Result;
use itertools::Itertools;

type V = (i32, i32);
type Grid = Vec<Vec<char>>;
type Prev = HashMap<(V, V), HashSet<(V, V)>>;

const WALL: char = '#';

fn main() -> Result<()> {
    let (grid, start, end) = parse()?;

    let dir = (1, 0);

    let part1 = dijkstra(&grid, start, dir, end);

    println!("Part 1: {}", part1);

    println!("Part 2: {}", dijkstra_all_paths(&grid, start, dir, end, part1));

    Ok(())
}

fn dijkstra(grid: &Grid, start: V, dir: V, end: V) -> i32 {
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    let mut seen = HashSet::new();

    let mut queue = BinaryHeap::from([Reverse((0, start, dir))]);

    while let Some(Reverse((cost, pos, dir))) = queue.pop() {
        // check for exit
        if pos == end {
            return cost;
        }

        // skip if seen
        if !seen.insert((pos, dir)) {
            continue;
        }

        // neighbours
        let mut neighbours = vec![
            (cost + 1000, pos, (-dir.1, dir.0)),
            (cost + 1000, pos, (dir.1, -dir.0 )),
        ];

        let npos = (pos.0 + dir.0, pos.1 + dir.1);
        if npos.0 >= 0 && npos.0 < w && npos.1 >= 0 && npos.1 < h && grid[npos.1 as usize][npos.0 as usize] != WALL {
            neighbours.push((cost + 1, npos, dir));
        }

        // enqueue
        for n in &neighbours {
            queue.push(Reverse(*n));
        }
    }

    -1
}

fn dijkstra_all_paths(grid: &Grid, start: V, dir: V, end: V, cost: i32) -> i32 {
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    let mut seen = HashSet::new();

    let mut queue = BinaryHeap::from([Reverse((0, start, dir))]);

    let mut dist = HashMap::new();
    let mut prev = Prev::new();

    while let Some(Reverse((cost, pos, dir))) = queue.pop() {
        // skip if seen
        if !seen.insert((pos, dir)) {
            continue;
        }

        // neighbours
        let mut neighbours = vec![
            (cost + 1000, pos, (-dir.1, dir.0)),
            (cost + 1000, pos, (dir.1, -dir.0 )),
        ];

        let npos = (pos.0 + dir.0, pos.1 + dir.1);
        if npos.0 >= 0 && npos.0 < w && npos.1 >= 0 && npos.1 < h && grid[npos.1 as usize][npos.0 as usize] != WALL {
            neighbours.push((cost + 1, npos, dir));
        }

        // enqueue
        for n in &neighbours {
            let d = *dist.get(&(n.1, n.2)).unwrap_or(&i32::MAX);
            if n.0 < d {
                prev.insert((n.1, n.2), HashSet::from([(pos, dir)]));
                dist.insert((n.1, n.2), n.0);
            } else if n.0 == d {
                prev.entry((n.1, n.2)).or_default().insert((pos, dir));
            }
            queue.push(Reverse(*n));
        }
    }

    // prev now contains all paths from end back to start
    // dfs from all nodes with end and min cost
    // only the one in my input but could be more so loop
    let mut seen = HashSet::new();

    for (k, v) in &dist {
        if *v == cost && k.0 == end {
            dfs(*k, &prev, &mut seen);
        }
    }

    seen.iter().map(|v| v.0).unique().count() as i32
}

fn dfs(v: (V, V), prev: &Prev, seen: &mut HashSet<(V, V)>) {
    seen.insert(v);
    if let Some(s) = prev.get(&v) {
        for p in s {
            if !seen.contains(p) {
                dfs(*p, prev, seen);
            }
        }
    }
}

fn parse() -> Result<(Grid, V, V)> {
    let mut grid = Grid::new();

    for line in io::stdin().lock().lines() {
        grid.push(line?.chars().collect());
    }

    let w = grid[0].len();
    let h = grid.len();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == 'S' {
                start = (x as i32, y as i32);
            } else if grid[y][x] == 'E' {
                end = (x as i32, y as i32);
            }
        }
    }

    Ok((grid, start, end))
}
