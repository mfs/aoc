use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

use anyhow::Result;

type V = (i32, i32);
type Grid = Vec<Vec<char>>;
type Regions = HashMap<(char, i32), HashSet<V>>;

fn main() -> Result<()> {
    let (grid, w, h) = parse()?;

    let regions = regions(&grid, w, h);

    println!("Part 1: {}", price_part1(&regions, w, h));
    println!("Part 2: {}", price_part2(&regions));

    Ok(())
}

fn regions(grid: &Grid, w: i32, h: i32) -> HashMap<(char, i32), HashSet<V>> {

    let mut region_id = 0;
    let mut regions = HashMap::new();

    let mut seen: HashSet<V> = HashSet::new();

    for y in 0..h {
        for x in 0..w {
            if seen.contains(&(x, y)) {
                continue;
            }

            let r = bfs((x, y), &grid, w, h);

            seen.extend(&r);

            regions.insert((grid[y as usize][x as usize], region_id), r);

            region_id += 1;
        }
    }

    regions
}

fn price_part1(regions: &Regions, w: i32, h: i32) -> i32 {
    let mut price = 0;

    for region in regions {
        let mut perimeter = 0;

        for sq in region.1 {
            for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let n = (sq.0 + dir.0, sq.1 + dir.1);
                if n.0 < 0 || n.0 >= w || n.1 < 0 || n.1 >= h {
                    perimeter += 1;
                } else if !region.1.contains(&n) {
                    perimeter += 1;
                }
            }
        }

        price += region.1.len() * perimeter;
    }

    price as i32
}

fn price_part2(regions: &Regions) -> i32 {
    let mut price = 0;

    for region in regions {
        let mut perimeter = 0;

        for sq in region.1 {
            for diag in [(-1, -1), (1, -1), (-1, 1), (1, 1)] {
                let nd = (sq.0 + diag.0, sq.1 + diag.1);
                let nx = (nd.0, sq.1);
                let ny = (sq.0, nd.1);

                match (region.1.contains(&nd), region.1.contains(&nx), region.1.contains(&ny)) {
                    (_, false, false) => perimeter += 1, // outer corner
                    (false, true, true) => perimeter += 1, // inner corner
                    _ => {},
                }
            }
        }

        price += region.1.len() * perimeter;
    }

    price as i32
}

fn bfs(start: V, grid: &Grid, w: i32, h: i32) -> HashSet<V> {
    let mut region = HashSet::new();

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    seen.insert(start);
    queue.push_back(start);
    let cr = grid[start.1 as usize][start.0 as usize];

    while let Some(v) = queue.pop_front() {
        let c = grid[v.1 as usize][v.0 as usize];
        if c == cr {
            region.insert(v);
        }

        for d in [(1,0), (-1, 0), (0, 1), (0, -1)] {
            let n = (v.0 + d.0, v.1 + d.1);

            if n.0 < 0 || n.0 >= w || n.1 < 0 || n.1 >= h || grid[n.1 as usize][n.0 as usize] != cr {
                continue;
            }

            if !seen.contains(&n) {
                seen.insert(n);
                queue.push_back(n);
            }
        }
    }

    region
}

fn parse() -> Result<(Grid, i32, i32)> {
    let mut grid = Grid::new();

    for line in io::stdin().lock().lines() {
        grid.push(line?.chars().collect());
    }

    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    Ok((grid, w, h))
}
