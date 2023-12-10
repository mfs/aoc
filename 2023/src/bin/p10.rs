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

    // find start location and patch correct pipe
    let width = grid[0].len();
    let height = grid.len();
    let mut start = (0, 0);

    let mut map = Map {
        grid,
        w: width,
        h: height,
    };

    for (y, x) in iproduct!(0..height, 0..width) {
        if map.grid[y][x] == START {
            start = (x, y);
            map.grid[y][x] = patch(&map, x, y);
            break;
        }
    }

    Ok((map, start))
}

fn patch(map: &Map, x: usize, y: usize) -> char {
    // Some overlap with neighbours() logic
    let mut neighbours = vec![];

    // North
    if y > 0 {
        neighbours.push(Some(map.grid[y - 1][x]));
    } else {
        neighbours.push(None);
    }

    // South
    if y + 1 < map.h {
        neighbours.push(Some(map.grid[y + 1][x]));
    } else {
        neighbours.push(None);
    }

    // West
    if x > 0 {
        neighbours.push(Some(map.grid[y][x - 1]));
    } else {
        neighbours.push(None);
    }

    // East
    if x + 1 < map.w {
        neighbours.push(Some(map.grid[y][x + 1]));
    } else {
        neighbours.push(None);
    }

    // Need test cases for these, only know NE is correct as that is what
    // my input uses
    match &neighbours[..] {
        &[Some(NS | SE | SW), Some(NS | NE | NW), _, _] => NS,
        &[_, _, Some(EW | NE | SE), Some(EW | NW | SW)] => EW,
        &[Some(NS | SE | SW), _, _, Some(EW | NW | SW)] => NE,
        &[Some(NS | SE | SW), _, Some(EW | NE | SE), _] => NW,
        &[_, Some(NS | NE | NW), _, Some(EW | NW | SW)] => SE,
        &[_, Some(NS | NE | NW), Some(EW | NE | SE), _] => SW,
        _ => unreachable!(),
    }
}
