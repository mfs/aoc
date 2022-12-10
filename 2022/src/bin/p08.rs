use anyhow::{anyhow, Result};
use itertools::iproduct;
use std::io::{self, BufRead};

#[derive(Default)]
struct Trees {
    grid: Vec<Vec<i64>>,
    width: usize,
    height: usize,
}

impl Trees {
    fn views(&self, x: usize, y: usize) -> Vec<Vec<i64>> {
        vec![
            (0..y).rev().map(|ny| self.grid[ny][x]).collect(),
            (y + 1..self.height).map(|ny| self.grid[ny][x]).collect(),
            (0..x).rev().map(|nx| self.grid[y][nx]).collect(),
            (x + 1..self.width).map(|nx| self.grid[y][nx]).collect(),
        ]
    }
}

fn main() -> Result<()> {
    let mut trees = Trees::default();

    for line in io::stdin().lock().lines() {
        let line = line?;
        trees
            .grid
            .push(line.chars().map(|c| c as i64 - '0' as i64).collect());
    }

    trees.height = trees.grid.len();
    trees.width = trees.grid[0].len();

    let part1 = iproduct!(0..trees.height, 0..trees.width)
        .filter(|&(y, x)| is_visible(x, y, &trees))
        .count();

    println!("Part 1: {}", part1);

    let part2 = iproduct!(0..trees.height, 0..trees.width)
        .map(|(y, x)| view_distances(x, y, &trees))
        .max()
        .ok_or(anyhow!("err"))?;

    println!("Part 2: {}", part2);

    Ok(())
}

fn is_visible(x: usize, y: usize, trees: &Trees) -> bool {
    trees
        .views(x, y)
        .iter()
        .map(|v| v.iter().all(|&t| t < trees.grid[y][x]))
        .any(|b| b)
}

fn view_distances(x: usize, y: usize, trees: &Trees) -> usize {
    let h = trees.grid[y][x];

    trees.views(x, y).iter().map(|x| distance(h, &x)).product()
}

fn distance(tree: i64, trees: &[i64]) -> usize {
    if let Some(d) = trees.iter().position(|&h| h >= tree) {
        return d + 1;
    } else {
        return trees.len();
    }
}
