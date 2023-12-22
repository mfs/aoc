use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, BufRead};

const PLOT: char = '.';
const _ROCK: char = '#';
const START: char = 'S';

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    w: usize,
    h: usize,
    start: (usize, usize),
}

const STEPS: usize = 26501365;

fn main() -> Result<()> {
    let map: Map = parse()?;

    let mut plots = HashSet::new();
    plots.insert(map.start);

    for _ in 0..64 {
        // do steps
        let mut new_plots = HashSet::new();
        for &p in &plots {
            let neigh = neighbours(p, &map);
            for n in neigh {
                new_plots.insert(n);
            }
        }

        // set to new set
        plots = new_plots;
    }

    println!("Part 1: {}", plots.len());

    let start = (map.start.0 as i64, map.start.1 as i64);

    let mut plots = HashSet::new();
    plots.insert(start);

    let mut counts = vec![];

    for i in 0.. {
        // do steps
        let mut new_plots = HashSet::new();
        for &p in &plots {
            let neigh = neighbours_wrap(p, &map);
            for n in neigh {
                new_plots.insert(n);
            }
        }

        if i % map.w == STEPS % map.w {
            // elements of a quadratic sequence
            counts.push(plots.len());
        }

        if counts.len() == 3 {
            break;
        }

        // set to new set
        plots = new_plots;
    }

    // first difference
    let d1 = [counts[1] - counts[0], counts[2] - counts[1]];

    // second difference
    let d2 = d1[1] - d1[0];

    let n = STEPS / map.w;

    let part_2 = counts[0] + d1[0] * n + (n * (n - 1) / 2) * d2;

    println!("Part 2: {}", part_2);

    Ok(())
}

fn neighbours_wrap(pt: (i64, i64), map: &Map) -> HashSet<(i64, i64)> {
    let (x, y) = pt;
    let mut n = HashSet::new();

    // north
    let yy = (y - 1).rem_euclid(map.h as i64);
    let xx = (x).rem_euclid(map.w as i64);
    if map.grid[yy as usize][xx as usize] == PLOT {
        n.insert((x, y - 1));
    }

    // south
    let yy = (y + 1).rem_euclid(map.h as i64);
    let xx = (x).rem_euclid(map.w as i64);
    if map.grid[yy as usize][xx as usize] == PLOT {
        n.insert((x, y + 1));
    }

    // east
    let xx = (x + 1).rem_euclid(map.w as i64);
    let yy = y.rem_euclid(map.h as i64);
    if map.grid[yy as usize][xx as usize] == PLOT {
        n.insert((x + 1, y));
    }

    // west
    let xx = (x - 1).rem_euclid(map.w as i64);
    let yy = y.rem_euclid(map.h as i64);
    if map.grid[yy as usize][xx as usize] == PLOT {
        n.insert((x - 1, y));
    }

    n
}

fn neighbours(pt: (usize, usize), map: &Map) -> HashSet<(usize, usize)> {
    let (x, y) = pt;
    let mut n = HashSet::new();

    // north
    if y > 0 && map.grid[y - 1][x] == PLOT {
        n.insert((x, y - 1));
    }

    // south
    if y + 1 < map.h && map.grid[y + 1][x] == PLOT {
        n.insert((x, y + 1));
    }

    // east
    if x + 1 < map.w && map.grid[y][x + 1] == PLOT {
        n.insert((x + 1, y));
    }

    // west
    if x > 0 && map.grid[y][x - 1] == PLOT {
        n.insert((x - 1, y));
    }

    n
}

fn parse() -> Result<Map> {
    let mut grid: Vec<_> = vec![];

    let mut start = (0, 0);

    for (y, line) in io::stdin().lock().lines().enumerate() {
        let line = line?;
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == START {
                start = (x, y);
                row.push(PLOT);
            } else {
                row.push(c);
            }
        }
        grid.push(row);
    }

    let w = grid[0].len();
    let h = grid.len();

    let map = Map { grid, w, h, start };

    Ok(map)
}
