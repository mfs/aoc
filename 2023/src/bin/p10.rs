use anyhow::Result;
use itertools::iproduct;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;

struct Map {
    grid: Grid,
    w: usize,
    h: usize,
}

const NS: char = '|';
const EW: char = '-';
const NE: char = 'L';
const NW: char = 'J';
const SW: char = '7';
const SE: char = 'F';
const GROUND: char = '.';
const START: char = 'S';

fn main() -> Result<()> {
    let (mut map, start) = parse()?;

    // patch pipe at start loc by brute forcing each possibility
    // until one returns the correct number of neighbours
    for pipe in [NS, EW, NE, NW, SW, SE] {
        map.grid[start.1][start.0] = pipe;
        if neighbours(&map, start.0, start.1).len() == 2 {
            break;
        }
    }

    let pipes = trace_loop(&map, start);

    // might need to handle if path is odd  length
    println!("Part 1: {}", pipes.len() / 2);

    // remove any random pipe segments
    for (y, x) in iproduct!(0..map.h, 0..map.w) {
        if !pipes.contains(&(x, y)) {
            map.grid[y][x] = GROUND;
        }
    }

    let inside = iproduct!(0..map.h, 0..map.w)
        .filter(|&(y, x)| map.grid[y][x] == GROUND && is_inside(&map, x, y))
        .count();

    println!("Part 2: {}", inside);
    Ok(())
}

// Use point in polygon algorithm by casting to outside the loop
// and counting crossings. Keep track of first corner seen so
// can tell by next corner if it's a crossing. e.g. 'L7' is a crossing
// but 'LJ' is not. Cast ray along x as this is easiest with map
// in row major format.
fn is_inside(map: &Map, x: usize, y: usize) -> bool {
    let mut crossings = 0;
    let mut first = GROUND;

    for &c in &map.grid[y][x..] {
        match (c, first) {
            (NS, _) => crossings += 1,
            (NE | SE, _) => first = c,
            (SW, NE) => crossings += 1,
            (NW, SE) => crossings += 1,
            _ => {}
        }
    }

    crossings % 2 == 1
}

fn trace_loop(map: &Map, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut loc = start;
    let mut path = vec![];

    while path.is_empty() || loc != start {
        let neighbours = neighbours(map, loc.0, loc.1);

        path.push(loc);
        // slow test, could use set
        if path.contains(&neighbours[0]) {
            loc = neighbours[1];
        } else {
            loc = neighbours[0];
        }
    }

    path
}

// return neighbours valid by current pipe segment
fn neighbours(map: &Map, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    let cur = map.grid[y][x];

    // North
    if y > 0 && [NS, NE, NW].contains(&cur) && [NS, SE, SW].contains(&map.grid[y - 1][x]) {
        neighbours.push((x, y - 1));
    }

    // South
    if y + 1 < map.h && [NS, SE, SW].contains(&cur) && [NS, NE, NW].contains(&map.grid[y + 1][x]) {
        neighbours.push((x, y + 1));
    }

    // West
    if x > 0 && [EW, SW, NW].contains(&cur) && [EW, SE, NE].contains(&map.grid[y][x - 1]) {
        neighbours.push((x - 1, y));
    }

    // East
    if x + 1 < map.w && [EW, SE, NE].contains(&cur) && [EW, SW, NW].contains(&map.grid[y][x + 1]) {
        neighbours.push((x + 1, y));
    }

    neighbours
}

fn parse() -> Result<(Map, (usize, usize))> {
    let mut grid: Grid = vec![];

    for line in io::stdin().lock().lines() {
        grid.push(line?.chars().collect());
    }

    let width = grid[0].len();
    let height = grid.len();
    let mut start = (0, 0);

    let map = Map {
        grid,
        w: width,
        h: height,
    };

    for (y, x) in iproduct!(0..height, 0..width) {
        if map.grid[y][x] == START {
            start = (x, y);
            break;
        }
    }

    Ok((map, start))
}
