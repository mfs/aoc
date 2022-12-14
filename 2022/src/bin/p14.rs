use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, BufRead};

type Pt = (i64, i64);
type Grid = HashMap<(i64, i64), char>;

const START: Pt = (500, 0);

fn main() -> Result<()> {
    let mut grid: HashMap<Pt, char> = HashMap::new();

    for line in io::stdin().lock().lines() {
        let line = line?;

        let points: Vec<_> = line
            .split(" -> ")
            .map(|s| {
                s.split(",")
                    .filter_map(|s| s.parse::<i64>().ok())
                    .collect::<Vec<_>>()
            })
            .collect();

        for w in points.windows(2) {
            let pt0 = (w[0][0], w[0][1]);
            let pt1 = (w[1][0], w[1][1]);
            plot([pt0, pt1], &mut grid);
        }
    }

    let floor = *grid.keys().map(|(_, y)| y).max().unwrap() + 2;

    // part 1
    let mut part1 = 0;

    while drop_sand(&mut grid, floor, false) {
        part1 += 1;
    }

    println!("Part 1: {}", part1);

    // clear sand
    grid.retain(|_, c| *c != 'o');

    let mut part2 = 0;
    while !grid.contains_key(&START) && drop_sand(&mut grid, floor, true) {
        part2 += 1;
    }

    println!("Part 2: {}", part2);

    Ok(())
}

fn drop_sand(grid: &mut Grid, floor: i64, floor_active: bool) -> bool {
    let mut s = START;

    loop {
        if s.1 > floor - 2 && !floor_active {
            return false;
        }

        if !grid.contains_key(&(s.0, s.1 + 1)) && s.1 + 1 < floor {
            // straight drop
            s.1 += 1;
        } else if !grid.contains_key(&(s.0 - 1, s.1 + 1)) && s.1 + 1 < floor {
            // left drop
            s.0 -= 1;
            s.1 += 1;
        } else if !grid.contains_key(&(s.0 + 1, s.1 + 1)) && s.1 + 1 < floor {
            // right drop
            s.0 += 1;
            s.1 += 1;
        } else {
            // rest
            grid.insert(s, 'o');
            return true;
        }
    }
}

fn plot(mut points: [Pt; 2], grid: &mut Grid) {
    points.sort();

    let (a, b) = (points[0], points[1]);

    if a.0 == b.0 {
        // vertical
        for y in a.1..=b.1 {
            grid.insert((a.0, y), '#');
        }
    } else if a.1 == b.1 {
        // horizontal
        for x in a.0..=b.0 {
            grid.insert((x, a.1), '#');
        }
    }
}
