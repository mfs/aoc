use anyhow::Result;
use itertools::{iproduct, Itertools};
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;
type Pt = (usize, usize);

#[derive(Debug)]
struct Image {
    grid: Grid,
    w: usize,
    h: usize,
}

const EMPTY: char = '.';
const GALAXY: char = '#';

fn main() -> Result<()> {
    let image = parse()?;

    // find rows and colums that have no GALAXIES.
    let (empty_rows, empty_cols) = empties(&image);

    // find galaxies
    let galaxies: Vec<_> = iproduct!(0..image.w, 0..image.h)
        .filter(|&(x, y)| image.grid[y][x] == GALAXY)
        .collect();

    for (idx, s) in [2, 1_000_000].into_iter().enumerate() {
        let sum: u64 = galaxies
            .iter()
            .combinations(2)
            .map(|v| distance(*v[0], *v[1], &empty_rows, &empty_cols, s))
            .sum();

        println!("Part {}: {}", idx + 1, sum);
    }

    Ok(())
}

fn distance(g0: Pt, g1: Pt, empty_rows: &[usize], empty_cols: &[usize], size: u64) -> u64 {
    use std::cmp::{max, min};

    let mut dist = 0;

    let rows = (min(g0.1, g1.1), max(g0.1, g1.1));
    let cols = (min(g0.0, g1.0), max(g0.0, g1.0));

    for (range, empty) in &[(rows, empty_rows), (cols, empty_cols)] {
        dist += range.1 as u64 - range.0 as u64;

        dist += empty
            .iter()
            .filter(|&r| r > &range.0 && r < &range.1)
            .count() as u64
            * (size - 1);
    }

    dist
}

fn empties(image: &Image) -> (Vec<usize>, Vec<usize>) {
    let mut rows = vec![];
    let mut cols = vec![];

    for y in 0..image.h {
        if image.grid[y].iter().all(|&c| c == EMPTY) {
            rows.push(y);
        }
    }

    for x in 0..image.w {
        if (0..image.h).map(|y| image.grid[y][x]).all(|c| c == EMPTY) {
            cols.push(x);
        }
    }

    (rows, cols)
}

fn parse() -> Result<Image> {
    let mut grid: Grid = vec![];

    for line in io::stdin().lock().lines() {
        grid.push(line?.chars().collect());
    }

    let width = grid[0].len();
    let height = grid.len();

    let image = Image {
        grid,
        w: width,
        h: height,
    };

    Ok(image)
}
