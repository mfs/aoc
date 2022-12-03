use anyhow::Result;
use std::io::{self, BufRead};

const ROCK: u64 = 0;
const PAPER: u64 = 1;
const SCISSORS: u64 = 2;
const MOD: u64 = 3;

const LOSE: u64 = 0;
const DRAW: u64 = 1;
const WIN: u64 = 2;

fn winner(h: u64) -> u64 {
    (h + 1) % MOD
}

fn loser(h: u64) -> u64 {
    (h + 2) % MOD
}

fn score(mut game: (u64, u64), part2: bool) -> u64 {
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

fn map_rps(hand: char) -> u64 {
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
        games.iter().map(|&g| score(g, false)).sum::<u64>()
    );

    println!(
        "Part 2: {}",
        games.iter().map(|&g| score(g, true)).sum::<u64>()
    );

    Ok(())
}
