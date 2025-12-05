use std::io::{self, BufRead};

use anyhow::Result;

fn main() -> Result<()> {
    let mut  grid = parse()?;

    let mut rolls = remove_rolls(&mut grid);

    println!("Part 1: {}", rolls);

    rolls += (0..)
        .map(|_| remove_rolls(&mut grid))
        .take_while(|x| *x != 0)
        .sum::<usize>();

    println!("Part 2: {}", rolls);

    Ok(())
}

fn remove_rolls(grid: &mut Vec<Vec<char>>) -> usize {
    let w = grid[0].len();
    let h = grid.len();

    let mut remove = vec![];

    for y in 0..h {
        for x in 0..w {
            if grid[y][x] != '@' {
                continue;
            }

            let count = neighbours(x, y, w, h)
                .iter()
                .filter(|(x, y)| grid[*y][*x] == '@')
                .count();

            if count < 4 {
                remove.push((x, y));
            }
        }
    }

    for &(x, y) in &remove {
        grid[y][x] = '.';
    }

    remove.len()
}

const NEIGHBOURS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1),
];

fn neighbours(x: usize, y: usize, w: usize, h: usize) -> Vec<(usize, usize)> {
    NEIGHBOURS
        .iter()
        .map(|&(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
        .filter(|&(nx, ny)| nx < w && ny < h)
        .collect()
}

fn parse() -> Result<Vec<Vec<char>>> {
    let mut grid = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;
        grid.push(line.chars().collect());
    }

    Ok(grid)
}
