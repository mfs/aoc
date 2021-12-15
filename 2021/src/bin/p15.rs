use anyhow::{anyhow, Result};
use std::io::{self, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;

const MAX: u64 = u64::MAX - 10;

fn main() -> Result<()> {
    let mut riskmap = vec![];
    for line in io::stdin().lock().lines() {
        let r: Vec<_> = line?.chars().map(|x| x  as u64 - '0' as u64 ).collect();
        riskmap.push(r);
    }

    let width = riskmap[0].len();
    let height = riskmap.len();

    let start = (0, 0);
    let end = (height - 1, width - 1);

    println!("Part 1: {}", a_star(start, end, &riskmap)?);

    let mut riskmap2 = vec![];
    for r in 0..height*5 {
        let mut row = vec![];
        for c in 0..width*5 {
            let x = c / width;
            let y = r / height;
            // risk is 1-9, the dec, and inc are so we can work in 0-8
            // and use modular arithmetic
            let risk = riskmap[r % height][c % width] - 1;
            let risk = (risk + (x + y) as u64) % 9;
            row.push(risk + 1);
        }
        riskmap2.push(row);
    }

    let width = riskmap2[0].len();
    let height = riskmap2.len();

    let start = (0, 0);
    let end = (height - 1, width - 1);

    println!("Part 2: {}", a_star(start, end, &riskmap2)?);

    Ok(())
}

// https://en.wikipedia.org/wiki/A*_search_algorithm
fn a_star(start: (usize, usize), end: (usize, usize), riskmap: &Vec<Vec<u64>>) -> Result<u64> {

    // a priority queue would be better here
    let mut open: HashSet<(usize, usize)> = HashSet::new();
    open.insert(start);

    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    // cheapest cost from start to n currently known
    let mut g_score: HashMap<(usize, usize), u64> = HashMap::new();
    g_score.insert(start, 0);

    // f_score = g_score + h
    let mut f_score: HashMap<(usize, usize), u64> = HashMap::new();
    f_score.insert(start, h(start, end));

    while !open.is_empty() {
        let mut current: (usize, usize) = *open.iter().min_by_key(|x| f_score.get(x).unwrap_or(&MAX)).unwrap();

        if current == end {
            // make path
            let mut cost = 0;
            while current != start {
                cost += riskmap[current.0][current.1];
                current = came_from[&current];
            }

            return Ok(cost);
        }

        open.remove(&current);

        for n in neighbours(&riskmap, current.0 as i64, current.1 as i64) {
            let tent_g_score = *g_score.entry(current).or_insert(MAX) + riskmap[n.0][n.1];
            if tent_g_score < *g_score.entry(n).or_insert(MAX) {
                came_from.insert(n, current);
                g_score.insert(n, tent_g_score);
                f_score.insert(n, tent_g_score + h(n, end));
                if !open.contains(&n) {
                    open.insert(n);
                }
            }
        }
    }

    Err(anyhow!("no path found"))
}

fn h(a: (usize, usize), b: (usize, usize)) -> u64 {
    ((b.0 as i64 - a.0 as i64).abs() + (b.1 as i64 - a.1 as i64).abs()) as u64
}

fn neighbours(heightmap: &Vec<Vec<u64>>, row: i64, col: i64) -> Vec<(usize, usize)> {
    let width = heightmap[0].len() as i64;
    let height = heightmap.len() as i64;

    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .map(|(r,c)| (row + r, col + c))
        .filter(|&(r, c)| r >= 0 && r < height && c >= 0 && c < width )
        .map(|(r, c)| (r as usize, c as usize))
        .collect()
}

