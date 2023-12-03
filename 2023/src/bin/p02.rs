use anyhow::Result;
use std::io::{self, BufRead};

type Set = (u32, u32, u32); // RGB
type Game = Vec<Set>;

const MAX_CUBES: Set = (12, 13, 14);

fn main() -> Result<()> {
    let mut games = vec![];

    for line in io::stdin().lock().lines() {
        games.push(parse_game(&line?)?);
    }

    let mut sum = 0;

    'outer: for (idx, game) in games.iter().enumerate() {
        for set in game {
            if set.0 > MAX_CUBES.0 || set.1 > MAX_CUBES.1 || set.2 > MAX_CUBES.2 {
                continue 'outer;
            }
        }

        sum += idx + 1;
    }

    println!("Part 1: {}", sum);

    let mut power = 0;

    for game in &games {
        let mut max = (0, 0, 0);

        for set in game {
            max.0 = std::cmp::max(max.0, set.0);
            max.1 = std::cmp::max(max.1, set.1);
            max.2 = std::cmp::max(max.2, set.2);
        }

        power += max.0 * max.1 * max.2;
    }

    println!("Part 2: {}", power);

    Ok(())
}

fn parse_game(input: &str) -> Result<Game> {
    let game_tokens: Vec<_> = input.split(": ").collect(); // ["Game 1", "3 blue, ..."]

    let set_tokens: Vec<_> = game_tokens[1].split("; ").collect(); // ["1 blue, 2 red, ...", ...]

    let mut game: Game = vec![];

    for s in &set_tokens {
        game.push(parse_set(s)?);
    }

    Ok(game)
}

fn parse_set(input: &str) -> Result<Set> {
    let mut set = (0u32, 0u32, 0u32);

    // ["2 red", ...]
    for color in input.split(", ") {
        let num_color: Vec<_> = color.split(" ").collect(); // ["2", "red"]

        match num_color[1] {
            "red" => {
                set.0 = num_color[0].parse()?;
            }
            "green" => {
                set.1 = num_color[0].parse()?;
            }
            "blue" => {
                set.2 = num_color[0].parse()?;
            }
            _ => unreachable!(),
        }
    }

    Ok(set)
}
