use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead};

// Part 2 relies on an assumption that's likely to hold for all inputs, namely
// that you can assume that the optimal path for each quadrant if all the other
// quadrants were already solved, would also be the optimal path in total.

const ENTRANCE: char = '@';
const WALL: char = '#';

struct Map {
    grid: Vec<Vec<char>>,
    w: usize,
    h: usize,
    start: (usize, usize),
}

fn main() -> Result<()> {
    let mut map = parse()?;

    println!("Part 1: {}", bfs(map.start, &map));

    // part 2

    // patch map
    let (sx, sy) = map.start;

    for y in sy - 1..=sy + 1 {
        map.grid[y][sx] = WALL;
    }

    for x in sx - 1..=sx + 1 {
        map.grid[sy][x] = WALL;
    }

    // start locations
    let quadrants = [
        (sx + 1, sy + 1),
        (sx + 1, sy - 1),
        (sx - 1, sy + 1),
        (sx - 1, sy - 1),
    ];

    let part_2: u32 = quadrants.iter().map(|q| bfs(*q, &map)).sum();

    println!("Part 2: {}", part_2);

    Ok(())
}

type Keys = u32;

// contains
fn key_contains(keys: Keys, c: char) -> bool {
    keys & 1 << (c as u32 - 'a' as u32) != 0
}

// set
fn key_set(keys: &mut Keys, c: char) {
    *keys |= 1 << (c as u32 - 'a' as u32);
}

fn bfs(start: (usize, usize), map: &Map) -> u32 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let b_keys = bfs_keys(start, map);

    // state
    // (x, y, keys, dist)
    queue.push_back((start.0, start.1, 0, 0));

    while let Some(v) = queue.pop_front() {
        if !visited.insert((v.0, v.1, v.2)) {
            continue;
        }

        let c = map.grid[v.1][v.0];

        if c == WALL
            || (c.is_ascii_uppercase()
                && key_contains(b_keys, c.to_ascii_lowercase())
                && !key_contains(v.2, c.to_ascii_lowercase()))
        {
            continue;
        }

        // keys
        let mut keys_n = v.2;
        if c.is_ascii_lowercase() {
            key_set(&mut keys_n, c);
            // fix me
            if keys_n == b_keys {
                return v.3;
            }
        }

        for d in neighbours(v.0, v.1, &map) {
            queue.push_back((d.0, d.1, keys_n, v.3 + 1));
        }
    }

    0
}

// simple bfs for finding keys in each quadrant
fn bfs_keys(start: (usize, usize), map: &Map) -> u32 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    // state
    // (x, y)
    queue.push_back((start.0, start.1));

    let mut keys = 0;

    while let Some(v) = queue.pop_front() {
        if !visited.insert((v.0, v.1)) {
            continue;
        }

        let c = map.grid[v.1][v.0];

        if c == WALL {
            continue;
        }

        // keys
        if c.is_ascii_lowercase() {
            key_set(&mut keys, c);
        }

        for d in neighbours(v.0, v.1, &map) {
            queue.push_back((d.0, d.1));
        }
    }

    keys
}

fn neighbours(x: usize, y: usize, m: &Map) -> Vec<(usize, usize)> {
    let mut ns = vec![];

    // north
    if y > 0 && m.grid[y - 1][x] != WALL {
        ns.push((x, y - 1));
    }

    // south
    if y + 1 < m.h && m.grid[y + 1][x] != WALL {
        ns.push((x, y + 1));
    }

    // east
    if x + 1 < m.w && m.grid[y][x + 1] != WALL {
        ns.push((x + 1, y));
    }

    // west
    if x > 0 && m.grid[y][x - 1] != WALL {
        ns.push((x - 1, y));
    }

    ns
}

fn parse() -> Result<Map> {
    let mut grid: Vec<Vec<char>> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        grid.push(line.chars().collect());
    }

    let h = grid.len();
    let w = grid[0].len();

    let mut start = (0, 0);

    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == ENTRANCE {
                start.0 = x;
                start.1 = y;
            }
        }
    }

    Ok(Map { grid, w, h, start })
}
