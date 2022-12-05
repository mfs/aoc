use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; 9];
    let mut moves: Vec<(usize, usize, usize)> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.contains('[') {
            // process stack
            for (i, c) in line.chars().enumerate().filter(|&(_, c)| c.is_uppercase()) {
                stacks[(i - 1) / 4].insert(0, c); // insert is slow but stacks are small
            }
        } else if line.starts_with("move") {
            // process move
            let el: Vec<_> = line.split(' ').collect();
            moves.push((el[1].parse()?, el[3].parse()?, el[5].parse()?));
        }
    }

    println!("Part 1: {}", process(&mut stacks.clone(), &moves, false));

    println!("Part 2: {}", process(&mut stacks, &moves, true));

    Ok(())
}

fn process(stacks: &mut Vec<Vec<char>>, moves: &[(usize, usize, usize)], part2: bool) -> String {
    for &(m, f, t) in moves {
        let mut temp: Vec<_> = (0..m).filter_map(|_| stacks[f - 1].pop()).collect();

        if part2 {
            temp.reverse();
        }

        stacks[t - 1].append(&mut temp);
    }

    stacks.iter().filter_map(|s| s.last()).collect()
}
