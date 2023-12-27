use anyhow::Result;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<u32>>,
    w: usize,
    h: usize,
}

// (hl, x, y, dx, dy, n)
type Node = BTreeSet<(u32, i32, i32, i32, i32, usize)>;

fn main() -> Result<()> {
    let map: Map = parse()?;

    println!("Part 1: {}", dijkstra_1(&map));

    println!("Part 2: {}", dijkstra_2(&map));

    Ok(())
}

fn dijkstra_1(map: &Map) -> u32 {
    let mut seen = HashSet::new();

    let mut queue: Node = Node::from([(0, 0, 0, 0, 0, 0)]);

    while let Some((heat, x, y, dx, dy, n)) = queue.pop_first() {
        // check for exit
        if y == map.h as i32 - 1 && x == map.w as i32 - 1 {
            return heat;
        }

        // skip if we have visited this tile before
        if !seen.insert((x, y, dx, dy, n)) {
            continue;
        }

        // go straight
        if n < 3 && (dx, dy) != (0, 0) {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < map.w as i32 && ny >= 0 && ny < map.h as i32 {
                let h = heat + map.grid[ny as usize][nx as usize];
                queue.insert((h, nx, ny, dx, dy, n + 1));
            }
        }

        // try turning
        for (ndx, ndy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if (ndx, ndy) != (dx, dy) && (ndx, ndy) != (-dx, -dy) {
                let nx = x + ndx;
                let ny = y + ndy;

                if nx >= 0 && nx < map.w as i32 && ny >= 0 && ny < map.h as i32 {
                    let h = heat + map.grid[ny as usize][nx as usize];
                    queue.insert((h, nx, ny, ndx, ndy, 1));
                }
            }
        }
    }

    0
}

fn dijkstra_2(map: &Map) -> u32 {
    let mut seen = HashSet::new();

    let mut queue: Node = Node::from([(0, 0, 0, 0, 0, 0)]);

    while let Some((heat, x, y, dx, dy, n)) = queue.pop_first() {
        // check for exit
        if y == map.h as i32 - 1 && x == map.w as i32 - 1 && n >= 4 {
            return heat;
        }

        // skip if we have visited this tile before
        if !seen.insert((x, y, dx, dy, n)) {
            continue;
        }

        // go straight
        if n < 10 && (dx, dy) != (0, 0) {
            let ny = y + dy;
            let nx = x + dx;
            if ny >= 0 && ny < map.h as i32 && nx >= 0 && nx < map.w as i32 {
                let h = heat + map.grid[ny as usize][nx as usize];
                queue.insert((h, nx, ny, dx, dy, n + 1));
            }
        }

        // try turning
        if n >= 4 || (dx, dy) == (0, 0) {
            for (ndx, ndy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if (ndx, ndy) != (dx, dy) && (ndx, ndy) != (-dx, -dy) {
                    let ny = y + ndy;
                    let nx = x + ndx;

                    if ny >= 0 && ny < map.h as i32 && nx >= 0 && nx < map.w as i32 {
                        let h = heat + map.grid[ny as usize][nx as usize];
                        queue.insert((h, nx, ny, ndx, ndy, 1));
                    }
                }
            }
        }
    }

    0
}

fn parse() -> Result<Map> {
    let mut grid: Vec<_> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;
        let mut row = vec![];
        for c in line.chars() {
            let hl = c.to_digit(10).unwrap();
            row.push(hl);
        }
        grid.push(row);
    }

    let w = grid[0].len();
    let h = grid.len();

    let map = Map { grid, w, h };

    Ok(map)
}
