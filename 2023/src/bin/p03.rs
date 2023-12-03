use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::io::{self, BufRead};

type Schematic = Vec<Vec<char>>;

type Number = (usize, usize, u32); // (x, y, n)

type Symbol = (usize, usize, char); // (x, y, char)

#[derive(PartialEq)]
enum State {
    Symbol,
    Number,
}

fn main() -> Result<()> {
    let mut schematic: Schematic = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        schematic.push(line.chars().collect());
    }

    let (numbers, symbols) = parse(&schematic)?;

    // part 1
    let mut sum = 0;

    'outer: for &(x, y, n) in &numbers {
        let (min_x, max_x, min_y, max_y) =
            bounding_box(schematic[0].len(), schematic.len(), x, y, n);

        for s in &symbols {
            if s.0 <= max_x && s.0 >= min_x && s.1 <= max_y && s.1 >= min_y {
                sum += n;
                continue 'outer;
            }
        }
    }

    println!("Part 1: {}", sum);

    // part 2
    let mut ratio_sum = 0;

    // brute force works though not very efficient
    for pair in numbers.iter().combinations(2) {
        let &(x0, y0, n0) = pair[0];
        let &(x1, y1, n1) = pair[1];

        for &(gx, gy, _) in symbols.iter().filter(|s| s.2 == '*') {
            let (n0_min_x, n0_max_x, n0_min_y, n0_max_y) =
                bounding_box(schematic[0].len(), schematic.len(), x0, y0, n0);

            let (n1_min_x, n1_max_x, n1_min_y, n1_max_y) =
                bounding_box(schematic[0].len(), schematic.len(), x1, y1, n1);

            if !(gx <= n0_max_x && gx >= n0_min_x && gy <= n0_max_y && gy >= n0_min_y) {
                continue;
            }

            if !(gx <= n1_max_x && gx >= n1_min_x && gy <= n1_max_y && gy >= n1_min_y) {
                continue;
            }

            ratio_sum += n0 * n1;
        }
    }

    println!("Part 2: {}", ratio_sum);

    Ok(())
}

// make do symbols, gears too
// State table parser, probably overkill
fn parse(schematic: &Schematic) -> Result<(Vec<Number>, Vec<Symbol>)> {
    let mut numbers = vec![];
    let mut symbols = vec![];
    let mut state = State::Symbol;
    let mut n = 0;
    let mut start_x = 0;

    for y in 0..schematic.len() {
        for x in 0..schematic[0].len() {
            let c = schematic[y][x];
            if c != '.' && !c.is_digit(10) {
                symbols.push((x, y, c));
            }
            match state {
                State::Symbol => {
                    if c.is_digit(10) {
                        // if number, move to number state, save digit in n
                        state = State::Number;
                        start_x = x;
                        n = c.to_digit(10).ok_or(anyhow!("invalid digit"))?;
                    }
                }
                State::Number => {
                    if c.is_digit(10) {
                        // if number, save digit in n
                        n = n * 10 + c.to_digit(10).ok_or(anyhow!("invalid digit"))?;
                    } else {
                        // if symbol, save (x, y, n) and reset
                        numbers.push((start_x, y, n)); // this is end x
                        n = 0;
                        start_x = 0;
                        state = State::Symbol;
                    }
                }
            }
        }
        // end of line, process last number if present
        if state == State::Number {
            numbers.push((start_x, y, n));
            n = 0;
            start_x = 0;
            state = State::Symbol;
        }
    }

    Ok((numbers, symbols))
}

fn bounding_box(
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    n: u32,
) -> (usize, usize, usize, usize) {
    let mut min_x = x;
    let mut min_y = y;
    let mut max_x = x + len_n(n) - 1;
    let mut max_y = y;

    if min_x > 0 {
        // left pad
        min_x -= 1;
    }

    if max_x + 1 < width {
        // right pad
        max_x += 1;
    }

    if min_y > 0 {
        // top pad
        min_y -= 1;
    }

    if max_y + 1 < height {
        // bottom pad
        max_y += 1;
    }

    (min_x, max_x, min_y, max_y)
}

fn len_n(n: u32) -> usize {
    if n < 10 {
        return 1;
    } else if n < 100 {
        return 2;
    } else if n < 1000 {
        return 3;
    }

    unreachable!();
}
