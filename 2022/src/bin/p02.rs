use anyhow::Result;
use std::io::{self, BufRead};

const ROCK: i64 = 0;
const PAPER: i64 = 1;
const SCISSORS: i64 = 2;
const MOD: i64 = 3;

const LOSE: i64 = 0;
const DRAW: i64 = 1;
const WIN: i64 = 2;

fn winner(h: i64) -> i64 {
    (h + 1) % MOD
}

fn loser(h: i64) -> i64 {
    (h + 2) % MOD
}

fn score(mut game: (i64, i64), part2: bool) -> i64 {
    if part2 {
        match game.1 {
            LOSE => game.1 = loser(game.0),
            DRAW => game.1 = game.0,
            WIN => game.1 = winner(game.0),
            _ => unreachable!(),
        }
    }

    if winner(game.0) == game.1 {
        6 + (game.1 + 1) // I win
    } else if game.0 == game.1 {
        3 + (game.1 + 1) // Draw
    } else {
        game.1 + 1 // I lose
    }
}

fn map_rps(hand: char) -> i64 {
    match hand {
        'A' | 'X' => ROCK,
        'B' | 'Y' => PAPER,
        'C' | 'Z' => SCISSORS,
        _ => unreachable!(),
    }
}

fn main() -> Result<()> {
    let mut games = vec![];

    for line in io::stdin().lock().lines() {
        let hands: Vec<_> = line?.chars().collect();

        games.push((map_rps(hands[0]), map_rps(hands[2])));
    }

    println!(
        "Part 1: {}",
        games.iter().map(|&g| score(g, false)).sum::<i64>()
    );

    println!(
        "Part 2: {}",
        games.iter().map(|&g| score(g, true)).sum::<i64>()
    );

    Ok(())
}
