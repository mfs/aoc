use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque};

use anyhow::Result;

type Devices = HashMap<String, Vec<String>>;

fn main() -> Result<()> {
    let devices = parse()?;

    println!("Part 1: {}", bfs(&devices));

    println!("Part 2: {}", dfs("svr", false, false, &devices, &mut HashMap::new()));

    Ok(())
}

fn bfs(devices: &Devices) -> u64 {
    let mut queue = VecDeque::new();

    queue.push_back("you");

    let mut count = 0;

    while let Some(v) = queue.pop_front() {
        if v == "out" {
            count += 1;
            continue;
        }

        for n in &devices[v] {
            queue.push_back(n);
        }
    }

    count
}

fn dfs<'a>(start: &'a str, dac_seen: bool, fft_seen: bool, devices: &'a Devices, cache: &mut HashMap<(&'a str, bool, bool), u64>) -> u64 {
    if let Some(n) = cache.get(&(start, dac_seen, fft_seen)) {
        return *n;
    }

    if start == "out" && dac_seen && fft_seen {
        return 1;
    } else if start == "out" {
        return 0;
    }

    let mut count = 0;

    for n in &devices[start] {
        let dac_seen = dac_seen || start == "dac";
        let fft_seen = fft_seen || start == "fft";
        count += dfs(&n, dac_seen, fft_seen, devices, cache);
    }

    cache.insert((start, dac_seen, fft_seen), count);
    count
}

fn parse() -> Result<Devices> {
    let mut devices = HashMap::new();

    for line in io::stdin().lock().lines() {
        let line = line?;

        let tokens: Vec<_> = line.split_whitespace().collect();

        devices.insert(
            tokens[0].trim_matches(':').to_owned(),
            tokens[1..].iter().map(|s| s.to_string()).collect()
        );
    }

    Ok(devices)
}
