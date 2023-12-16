use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, BufRead};

const VERT_SPLIT: char = '|';
const HORIZ_SPLIT: char = '-';
const MIRROR_F: char = '/';
const MIRROR_B: char = '\\';
const EMPTY: char = '.';

type V = (i32, i32);
type S = HashSet<(V, V)>;

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    w: usize,
    h: usize,
}

fn main() -> Result<()> {
    let map: Map = parse()?;

    let cur: V = (0, 0);
    let dir: V = (1, 0);

    let mut set: S = HashSet::new();

    beam(cur, dir, &map, &mut set);

    let locs: HashSet<(i32, i32)> = set.iter().map(|&(l, _)| l).collect();

    println!("Part 1: {}", locs.len());

    // part 2
    let mut starts = vec![];

    // left & right
    for y in 0..map.h {
        starts.push(((0, y as i32), (1, 0)));
        starts.push(((map.w as i32 - 1, y as i32), (-1, 0)));
    }

    // top & bottom
    for x in 0..map.w {
        starts.push(((x as i32, 0), (0, 1)));
        starts.push(((x as i32, map.h as i32 - 1), (0, -1)));
    }

    let mut max = 0;
    for &(l, d) in &starts {
        set.clear();
        beam(l, d, &map, &mut set);
        let locs: HashSet<(i32, i32)> = set.iter().map(|&(l, _)| l).collect();
        if locs.len() > max {
            max = locs.len();
        }
    }

    println!("Part 2: {}", max);

    Ok(())
}

fn beam(loc: V, dir: V, map: &Map, set: &mut S) {
    // catch all out of bounds locs here
    if loc.0 < 0 || loc.0 > map.w as i32 - 1 || loc.1 < 0 || loc.1 > map.h as i32 - 1 {
        return;
    }

    // already been here with same dir
    if let Some(_) = set.get(&(loc, dir)) {
        return;
    }

    set.insert((loc, dir));

    let cur = map.grid[loc.1 as usize][loc.0 as usize];

    let mut next: Vec<(V, V)> = vec![]; // loc, start

    match cur {
        VERT_SPLIT => {
            for x in vert_split(loc, dir) {
                next.push(x);
            }
        }
        HORIZ_SPLIT => {
            for x in horiz_split(loc, dir) {
                next.push(x);
            }
        }
        MIRROR_F => {
            next.push(mirror_f(loc, dir));
        }
        MIRROR_B => {
            next.push(mirror_b(loc, dir));
        }
        EMPTY => {
            next.push(((loc.0 + dir.0, loc.1 + dir.1), dir));
        }
        _ => unreachable!(),
    }

    // now recurse on each one
    for n in next {
        beam(n.0, n.1, map, set);
    }
}

// Vert Split |
fn vert_split(loc: V, dir: V) -> Vec<(V, V)> {
    let mut v = vec![];

    match dir {
        (1, 0) => {
            v.push(((loc.0, loc.1 + 1), (0, 1)));
            v.push(((loc.0, loc.1 - 1), (0, -1)));
        }
        (-1, 0) => {
            v.push(((loc.0, loc.1 + 1), (0, 1)));
            v.push(((loc.0, loc.1 - 1), (0, -1)));
        }
        _ => {
            v.push(((loc.0 + dir.0, loc.1 + dir.1), dir));
        }
    }

    v
}

// Horiz Split -
fn horiz_split(loc: V, dir: V) -> Vec<(V, V)> {
    let mut v = vec![];

    match dir {
        (0, 1) => {
            v.push(((loc.0 + 1, loc.1), (1, 0)));
            v.push(((loc.0 - 1, loc.1), (-1, 0)));
        }
        (0, -1) => {
            v.push(((loc.0 + 1, loc.1), (1, 0)));
            v.push(((loc.0 - 1, loc.1), (-1, 0)));
        }
        _ => {
            v.push(((loc.0 + dir.0, loc.1 + dir.1), dir));
        }
    }

    v
}

// Mirror /
fn mirror_f(loc: V, dir: V) -> (V, V) {
    let new_dir = match dir {
        (1, 0) => (0, -1), // right
        (-1, 0) => (0, 1), // left
        (0, 1) => (-1, 0), // down
        (0, -1) => (1, 0), // up
        _ => unreachable!(),
    };

    ((loc.0 + new_dir.0, loc.1 + new_dir.1), new_dir)
}

// Mirror \
fn mirror_b(loc: V, dir: V) -> (V, V) {
    let new_dir = match dir {
        (1, 0) => (0, 1),   // right
        (-1, 0) => (0, -1), // left
        (0, 1) => (1, 0),   // down
        (0, -1) => (-1, 0), // up
        _ => unreachable!(),
    };

    ((loc.0 + new_dir.0, loc.1 + new_dir.1), new_dir)
}

fn parse() -> Result<Map> {
    let mut grid: Vec<_> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        grid.push(line.chars().collect::<Vec<_>>());
    }

    let w = grid[0].len();
    let h = grid.len();

    let map = Map { grid, w, h };

    Ok(map)
}
