use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut elf_pairs = vec![];

    for line in io::stdin().lock().lines() {
        let sec: Vec<_> = line?
            .split(&['-', ','])
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        elf_pairs.push(((sec[0], sec[1]), (sec[2], sec[3])));
    }

    let part1 = elf_pairs.iter().filter(|p| contains(p.0, p.1)).count();

    println!("Part 1: {}", part1);

    let part2 = elf_pairs.iter().filter(|p| overlaps(p.0, p.1)).count();

    println!("Part 2: {}", part2);

    Ok(())
}

fn contains(a: (u64, u64), b: (u64, u64)) -> bool {
    (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1)
}

fn overlaps(a: (u64, u64), b: (u64, u64)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}
