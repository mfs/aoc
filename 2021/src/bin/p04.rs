use anyhow::Result;
use std::io::{self, BufRead};

const SZ: usize = 5;

#[derive(Default, Debug)]
struct Board {
    solved: bool,
    cells: [[u64; SZ]; SZ],
    marks: [[u64; SZ]; SZ],
}

impl Board {
    fn mark(&mut self, n: u64) {
        for row in 0..SZ {
            for col in 0..SZ {
                if self.cells[row][col] == n {
                    self.marks[row][col] = 1;
                }
            }
        }
    }

    fn bingo(&self) -> bool {
        let mut rows = [0; SZ];
        let mut cols = [0; SZ];

        for row in 0..SZ {
            for col in 0..SZ {
               rows[row] += self.marks[row][col];
               cols[col] += self.marks[row][col];
            }
        }

        rows.iter().any(|x| *x == SZ as u64) || cols.iter().any(|x| *x == SZ as u64)
    }

    fn score(&self) -> u64 {
        let mut sum = 0;
        for row in 0..SZ {
            for col in 0..SZ {
                // (1 - mark) as we want unmarked
                sum += self.cells[row][col] * (1 - self.marks[row][col]);
            }
        }

        sum
    }
}

fn main() -> Result<()> {

    let (moves, mut boards) = parse()?;

    let mut scores = vec![];

    for m in moves {
        for b in &mut boards {
            b.mark(m);
            if !b.solved && b.bingo() {
                b.solved = true;
                scores.push(b.score() * m);
            }
        }
    }

    println!("Part 1: {}", scores[0]);
    println!("Part 2: {}", scores[scores.len() - 1]);

    Ok(())
}

fn parse() -> Result<(Vec<u64>, Vec<Board>)> {
    let mut moves = vec![];
    let mut rows = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.contains(',') {
            moves = line.split(',').map(|x| x.parse()).collect::<Result<_, _>>()?;
        } else if line.contains(' ') {
            let row: Vec<u64> = line
                .split_whitespace()
                .map(|x| x.parse())
                .collect::<Result<_, _>>()?;
            rows.push(row);
        }
    }

    let mut boards = vec![];
    for br in rows.chunks(SZ) {
        let mut b = Board::default();

        for row in 0..SZ {
            for col in 0..SZ {
                b.cells[row][col] = br[row][col];
            }
        }

        boards.push(b);
    }

    Ok((moves, boards))
}
