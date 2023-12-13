use anyhow::Result;
use std::io::{self, Read};

const ASH: char = '.';
const ROCK: char = '#';

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    w: usize,
    h: usize,
}

fn main() -> Result<()> {
    let maps: Vec<Map> = parse()?;

    // part 1
    println!(
        "Part 1: {}",
        maps.iter().map(|m| score_map(m)).flatten().sum::<usize>()
    );

    // part 2
    let mut sum = 0;

    for map in &maps {
        //calc original score
        let original = score_map(&map);

        'outer: for y in 0..map.h {
            for x in 0..map.w {
                // clone and tweak
                let mut m = map.clone();

                m.grid[y][x] = match m.grid[y][x] {
                    ROCK => ASH,
                    ASH => ROCK,
                    _ => unreachable!(),
                };

                let new_score = score_map(&m);
                if new_score.len() > 0 && new_score != original {
                    sum += new_score
                        .iter()
                        .filter(|x| !original.contains(x))
                        .sum::<usize>();
                    break 'outer;
                }
            }
        }
    }

    println!("Part 2: {}", sum);

    Ok(())
}

// fairly inefficient but it works
fn score_map(map: &Map) -> Vec<usize> {
    let mut lines = vec![];

    let rows: Vec<String> = map.grid.iter().map(|r| r.iter().collect()).collect();

    let cols: Vec<String> = (0..map.w)
        .map(|x| (0..map.h).map(|y| map.grid[y][x]).collect())
        .collect();

    for y in 1..(map.h) {
        if is_mirror(y, &rows) {
            lines.push(y * 100);
        }
    }

    for x in 1..(map.w) {
        if is_mirror(x, &cols) {
            lines.push(x);
        };
    }

    lines
}

fn is_mirror(idx: usize, lines: &[String]) -> bool {
    let top = 0..idx;
    let bottom = idx..lines.len();

    for p in std::iter::zip(top.rev(), bottom) {
        if lines[p.0] != lines[p.1] {
            return false;
        }
    }

    true
}

fn parse() -> Result<Vec<Map>> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut maps = vec![];

    for m in buffer.split("\n\n") {
        let mut grid: Vec<Vec<char>> = vec![];
        for r in m.trim().split('\n') {
            grid.push(r.chars().collect());
        }
        let w = grid[0].len();
        let h = grid.len();
        maps.push(Map { grid, w, h });
    }

    Ok(maps)
}
