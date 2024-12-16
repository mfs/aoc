use std::io::{self, BufRead};

use anyhow::Result;
use itertools::iproduct;

type Grid = Vec<Vec<char>>;
type V = (i32, i32);

trait GridExt {
    fn cell(&self, x: i32, y: i32) -> char;
    fn cell_mut(&mut self, x: i32, y: i32) -> &mut char;
}

impl GridExt for Grid {
    fn cell(&self, x: i32, y: i32) -> char {
        self[y as usize][x as usize]
    }
    fn cell_mut(&mut self, x: i32, y: i32) -> &mut char {
       &mut  self[y as usize][x as usize]
    }
}

const BOX: char = 'O';
const BOXL: char = '[';
const BOXR: char = ']';
const WALL: char = '#';
const ROBOT: char = '@';
const EMPTY: char = '.';

fn main() -> Result<()> {
    let (grid, w, h, start, moves) = parse()?;

    let mut expanded_grid = expand_grid(&grid); // part 2

    println!("Part 1: {}", part1(&mut grid.clone(), w, h, start, &moves));

    println!("Part 2: {}", part2(&mut expanded_grid, w, h, start, &moves));

    Ok(())
}

fn part1(grid: &mut Grid, w: i32, h: i32, start: V, moves: &Vec<V>) -> i32 {
   let mut robot = start;

    for &dir in moves {
        let m = trace_grid(&grid, robot, dir);

        if !m.is_empty() {
            for w in m.windows(2) {
                let new = w[0];
                let old = w[1];

                *grid.cell_mut(new.0, new.1) = grid.cell(old.0, old.1);
            }

            *grid.cell_mut(robot.0, robot.1) = EMPTY; // clear old robot square

            robot = (robot.0 + dir.0, robot.1 + dir.1); // update new robot pos
        }
    }

    iproduct!(0..w, 0..h)
        .filter(|&(x, y)| grid.cell(x, y) == BOX)
        .map(|(x, y)| 100 * y + x)
        .sum::<i32>()
}

fn part2(grid: &mut Grid, w: i32, h: i32, start: V, moves: &Vec<V>) -> i32 {
    let mut robot = (start.0 * 2, start.1);

    for &dir in moves {
        // horizontal move, use part 1 trace
        if dir == (1, 0) || dir == (-1, 0) {
            let m = trace_grid(grid, robot, dir);
            if !m.is_empty() {
                for w in m.windows(2) {
                    let new = w[0];
                    let old = w[1];

                    *grid.cell_mut(new.0, new.1) = grid.cell(old.0, old.1);
                }

                // update final position which is current robot pos before move
                *grid.cell_mut(robot.0, robot.1) = EMPTY;

                // update robot
                robot = (robot.0 + dir.0, robot.1 + dir.1);
            }
        } else {
            // vertical moves
            let mut boxes = vec![];

            let can_move = push_vertical(robot, dir, &grid, &mut boxes);

            if can_move {
                // dedup and sort by furthest away
                boxes.sort_by_key(|b| (b.1 * -dir.1, b.0));
                boxes.dedup();

                for sq in &boxes {
                    // sq is the current location of a LBOX or RBOX
                    // save type of box
                    let b = grid.cell(sq.0,sq.1);

                    // clear old square
                    *grid.cell_mut(sq.0, sq.1) = EMPTY;

                    // update new square
                    *grid.cell_mut(sq.0, sq.1 + dir.1) = b;
                }

                // update start position
                *grid.cell_mut(robot.0, robot.1) = EMPTY;
                *grid.cell_mut(robot.0, robot.1 + dir.1) = ROBOT;

                // update robot position
                robot = (robot.0 + dir.0, robot.1 + dir.1);
            }
        }
    }

    iproduct!(0..(w*2), 0..h)
        .filter(|&(x, y)| grid.cell(x, y) == BOXL)
        .map(|(x, y)| 100 * y + x)
        .sum::<i32>()
}

fn push_vertical(pos: V, dir: V, grid: &Grid, boxes: &mut Vec<V>) -> bool {
    let x = pos.0 + dir.0;
    let y = pos.1 + dir.1;
    let cell = grid.cell(x, y);

    let mut other = (x, y);

    match cell {
        EMPTY => return true,
        WALL => return false,
        BOXL => other.0 += 1,
        BOXR => other.0 -= 1,
        _ => unreachable!(),
    }

    boxes.push((x, y));
    boxes.push(other);

    return push_vertical((x, y), dir, grid, boxes) && push_vertical(other, dir, grid, boxes);
}

fn expand_grid(grid: &Grid) -> Grid {
    let mut expanded = Grid::new();

    for row in grid {
        let mut expanded_row = vec![];
        for &c in row {
            match &c {
                &WALL => expanded_row.extend(&[WALL, WALL]),
                &BOX => expanded_row.extend(&[BOXL, BOXR]),
                &EMPTY => expanded_row.extend(&[EMPTY, EMPTY]),
                &ROBOT => expanded_row.extend(&[ROBOT, EMPTY]),
                _ => unreachable!(),
            }
        }
        expanded.push(expanded_row);
    }

    expanded
}

fn trace_grid(grid: &Grid, mut pos: V, dir: V) -> Vec<V> {
    let mut squares = vec![pos];

    loop {
        pos.0 += dir.0;
        pos.1 += dir.1;

        match grid.cell(pos.0, pos.1) {
            WALL => return vec![],
            EMPTY => {
                squares.push(pos);
                break;
            },
            BOX => squares.push(pos),
            BOXL => squares.push(pos), // part 2
            BOXR => squares.push(pos), // part 2
            _ => unreachable!(),
        }
    }

    squares.reverse();
    squares
}

fn parse() -> Result<(Grid, i32, i32, V, Vec<V>)> {
    let mut grid = Grid::new();
    let mut moves = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.contains(WALL) {
            grid.push(line.chars().collect());
        } else if line != "" {
            for c in line.chars() {
                let d = match c {
                    '>' => (1, 0),
                    '<' => (-1, 0),
                    '^' => (0, -1),
                    'v' => (0, 1),
                    _ => unreachable!(),
                };
                moves.push(d);
            }
        }
    }

    let w = grid[0].len() as i32;
    let h = grid.len() as i32;

    let mut start = (0, 0);

    for (x, y) in iproduct!(0..w, 0..h) {
        if grid.cell(x, y) == ROBOT {
            start = (x, y);
        }
    }

    Ok((grid, w, h, start, moves))
}
