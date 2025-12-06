use std::io::{self, BufRead};

use anyhow::Result;

fn main() -> Result<()> {
    let homework = parse()?;

    let w = homework[0].len();
    let h = homework.len();

    let mut part1 = 0;
    let mut part2 = 0;

    let mut cur_op = ' ';
    let mut numbers_vert: Vec<u64> = vec![];
    let mut numbers_horiz: Vec<u64> = vec![0; h-1];

    for x in 0..w {
        if homework[h-1][x] != ' ' {
            cur_op = homework[h-1][x];
        }

        // extract column x
        let col: Vec<_> = (0..h-1)
            .map(|y| homework[y][x])
            .collect();

        // if current column is all spaces, process numbers and reset
        if col.iter().all(|&c| c == ' ') {
            if cur_op == '*' {
                part1 += numbers_horiz.iter().product::<u64>();
                part2 += numbers_vert.iter().product::<u64>();
            } else {
                part1 += numbers_horiz.iter().sum::<u64>();
                part2 += numbers_vert.iter().sum::<u64>();
            }

            numbers_horiz.fill(0);
            numbers_vert.clear();
            continue;
        }

        // read vertical number in column x
        let n = col
            .iter()
            .filter_map(|c| c.to_digit(10))
            .fold(0, |acc, x| acc * 10 + x);

        numbers_vert.push(n as u64);

        // update horizontal numbers with digits in column x
        for y in 0..h-1 {
            if let Some(horiz_n) = homework[y][x].to_digit(10) {
                numbers_horiz[y] = numbers_horiz[y] * 10 + horiz_n as u64;
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse() -> Result<Vec<Vec<char>>> {
    let mut homework: Vec<Vec<char>> = vec![];

    for line in io::stdin().lock().lines() {
        let mut row: Vec<_> = line?.chars().collect();
        // add a column of spaces at end to process last set of numbers
        row.push(' ');

        homework.push(row);
    }

    Ok(homework)
}
