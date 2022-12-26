use anyhow::{anyhow, Result};
use std::io::{self, BufRead};

const INF: u64 = u64::MAX / 2;

#[derive(Debug)]
struct Loc {
    height: char,
    dist: u64,
}

impl Loc {
    fn new(height: char) -> Self {
        Loc { height, dist: INF }
    }
}

fn main() -> Result<()> {
    let mut map: Vec<Vec<Loc>> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;
        let mut row = vec![];
        for c in line.chars() {
            row.push(Loc::new(c));
        }
        map.push(row);
    }

    let height = map.len();
    let width = map[0].len();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..height {
        for x in 0..width {
            if map[y][x].height == 'S' {
                start = (x, y);
                map[y][x].height = 'a';
            } else if map[y][x].height == 'E' {
                end = (x, y);
                map[y][x].height = 'z';
            }
        }
    }

    // search from end
    dijkstra(end, &mut map);

    let mut distances = vec![];
    for y in 0..height {
        for x in 0..width {
            if map[y][x].height == 'a' {
                distances.push(map[y][x].dist);
            }
        }
    }
    println!("Part 1: {}", map[start.1][start.0].dist);

    println!(
        "Part 2: {}",
        distances.iter().min().ok_or(anyhow!("no distances"))?
    );

    Ok(())
}

fn dijkstra(start: (usize, usize), map: &mut Vec<Vec<Loc>>) {
    let height = map.len();
    let width = map[0].len();

    let mut queue = vec![];
    for y in 0..height {
        for x in 0..width {
            queue.push((x, y));
        }
    }

    map[start.1][start.0].dist = 0;

    while let Some(u) = queue.pop() {
        for v in neighbours(u.0, u.1, map) {
            if !queue.contains(&v) {
                continue;
            }

            let alt = map[u.1][u.0].dist + 1;
            if alt < map[v.1][v.0].dist {
                map[v.1][v.0].dist = alt;
            }
        }

        queue.sort_unstable_by_key(|&(x, y)| std::cmp::Reverse(map[y][x].dist));
    }
}

fn neighbours(x: usize, y: usize, map: &Vec<Vec<Loc>>) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];

    let h = map.len();
    let w = map[0].len();

    let reachable = |start: char, end: char| start as i64 <= (end as i64 + 1);
    let inbound = |p: (i64, i64)| (0..w as i64).contains(&p.0) && (0..h as i64).contains(&p.1);

    for pt in [(1i64, 0i64), (-1, 0), (0, 1), (0, -1)] {
        let n = (x as i64 + pt.0, y as i64 + pt.1);

        if inbound(n) && reachable(map[y][x].height, map[n.1 as usize][n.0 as usize].height) {
            neighbours.push((n.0 as usize, n.1 as usize));
        }
    }

    neighbours
}
