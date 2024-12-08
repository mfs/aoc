use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

use anyhow::Result;
use itertools::Itertools;

type V = (i32, i32);
type Antenna = Vec<V>;
type Antennas = HashMap<char, Antenna>;

fn main() -> Result<()> {
    let (antennas, w, h) = parse()?;

    let antinodes = solve(&antennas, w, h);

    println!("Part 1: {}", antinodes.0);
    println!("Part 2: {}", antinodes.1);

    Ok(())
}

fn solve(antennas: &Antennas, w: i32, h: i32) -> (usize, usize) {
    let mut antinodes: [HashSet<V>; 2] = Default::default();

    for (_, v) in antennas {
        for pair in v.iter().permutations(2) {
            let delta = (pair[0].0 - pair[1].0, pair[0].1 - pair[1].1);
            antinodes_part1(&pair, delta, w, h, &mut antinodes[0]);
            antinodes_part2(*pair[0], delta, w, h, &mut antinodes[1]);
        }
    }

    (antinodes[0].len(), antinodes[1].len())
}

fn antinodes_part1(pair: &[&V], delta: V, w: i32, h: i32, antinodes: &mut HashSet<V>) {
   for antenna in pair {
       for delta in [(delta.0, delta.1), (-delta.0, -delta.1)] {
           let p = (antenna.0 + delta.0, antenna.1 + delta.1);
           if p.0 >= 0 && p.0 < w && p.1 >= 0 && p.1 < h && !pair.contains(&&p) {
               antinodes.insert(p);
           }
       }
   }
}

fn antinodes_part2(start: V, delta: V, w: i32, h: i32, antinodes: &mut HashSet<V>) {
   for dir in [(delta.0, delta.1), (-delta.0, -delta.1)] {
       let mut p = start;

       while p.0 >= 0 && p.0 < w && p.1 >= 0 && p.1 < h {
           antinodes.insert(p);
           p = (p.0 + dir.0, p.1 + dir.1);
       }
    }
}

fn parse() -> Result<(Antennas, i32, i32)> {
    let mut grid = vec![];

    for line in io::stdin().lock().lines() {
        grid.push(line?.chars().collect::<Vec<_>>());
    }

    let w = grid[0].len();
    let h = grid.len();

   let mut antennas = Antennas::new();

    for  y in 0..h {
        for x in 0..w {
            let c = grid[y][x];
            if c != '.' {
                antennas.entry(c).or_insert(vec![]).push((x as i32, y as i32));
            }
        }
    }

    Ok((antennas, w as i32, h as i32))
}
