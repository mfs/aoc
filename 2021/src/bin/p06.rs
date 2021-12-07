use anyhow::Result;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut fish = [0u64; 9];

    for n in buffer.trim().split(',') {
        fish[n.parse::<usize>()?] += 1;
    }

    let p1 = process(&mut fish.clone(), 80);
    println!("Part 1: {}", p1);

    let p2 = process(&mut fish.clone(), 256);
    println!("Part 2: {}", p2);

    Ok(())
}

fn process(fish: &mut [u64; 9], days: u64) -> u64 {
    for _ in 0..days {
        let mut next_fish = [0u64; 9];

        for i in 0..9 {
            if i == 0 {
                next_fish[6] = fish[0];
                next_fish[8] = fish[0];
            } else {
                next_fish[i - 1] += fish[i];
            }
        }

        *fish = next_fish;
    }

    fish.iter().sum()
}
