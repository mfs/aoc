use anyhow::Result;
use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut fish: HashMap<u64, u64> = HashMap::new();

    for n in buffer.trim().split(',') {
        *fish.entry(n.parse()?).or_insert(0) += 1;
    }

    let p1 = process(&mut fish.clone(), 80);
    println!("Part 1: {}", p1);

    let p2 = process(&mut fish.clone(), 256);
    println!("Part 2: {}", p2);

    Ok(())
}

fn process(fish: &mut HashMap<u64, u64>, days: u64) -> u64 {
    for _ in 0..days {
        let mut next_fish: HashMap<u64, u64> = HashMap::new();

        for i in 0..9 {
            if i == 0 {
                let fish_zero = *fish.get(&0).unwrap_or(&0);
                // next_fish[6] = fish[0]
                next_fish.insert(6, fish_zero);
                //next_fish[8] = fish[0]
                next_fish.insert(8, fish_zero);
            } else {
                // next_fish[i - 1] += fish[i]
                *next_fish.entry(i - 1).or_insert(0) += fish.get(&i).unwrap_or(&0);
            }
        }

        *fish = next_fish;
    }

    fish.values().sum()
}
