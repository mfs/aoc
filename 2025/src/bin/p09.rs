use std::io::{self, BufRead};
use std::cmp::Reverse;

use anyhow::Result;

type Tiles = Vec<(i64, i64)>;

fn main() -> Result<()> {
    let tiles = parse()?;

    let mut max_area = 0;

    let mut areas = vec![]; // (area, v0, v1)

    for t0 in &tiles {
        for t1 in &tiles {
            let area = ((t0.0 - t1.0).abs() + 1) * ((t0.1 - t1.1).abs() + 1);
            areas.push((area, t0, t1));
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Part 1: {}", max_area);

    areas.sort_unstable_by_key(|x| Reverse(*x));

    let mut tiles = tiles.clone();
    tiles.push(tiles[0]); // so windows(2) is cyclic

    'outer: for (area, t0, t1) in areas {
        let min_x = std::cmp::min(t0.0, t1.0);
        let max_x = std::cmp::max(t0.0, t1.0);
        let min_y = std::cmp::min(t0.1, t1.1);
        let max_y = std::cmp::max(t0.1, t1.1);

        for tile in &tiles {
            if tile.0 > min_x && tile.0 < max_x && tile.1 > min_y && tile.1 < max_y {
                // contains another red tile
                continue 'outer;
            }
        }

        // check for lines intersecting, not on edges
        // probably an easier way to do this
        for e in tiles.windows(2) {
            let v0 = e[0];
            let v1 = e[1];

            if v0.0 == v1.0 {
                // same x co-ordinate vertical line
                let e_min_y = std::cmp::min(v0.1, v1.1);
                let e_max_y = std::cmp::max(v0.1, v1.1);
                let e_x = v0.0;

                if e_min_y <= min_y && e_max_y >= max_y && e_x > min_x && e_x < max_x {
                    continue 'outer;
                }
            } else {
                // horiz line
                let e_min_x = std::cmp::min(v0.0, v1.0);
                let e_max_x = std::cmp::max(v0.0, v1.0);
                let e_y = v0.1;

                if e_min_x <= min_x && e_max_x >= max_x && e_y > min_y && e_y < max_y {
                    continue 'outer;
                }
            }
        }

        println!("Part 2: {}", area);
        break;
    }

    Ok(())
}

fn parse() -> Result<Tiles> {
    let mut tiles = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;
        let tokens = line.split(',').map(|s| s.parse::<i64>()).collect::<Result<Vec<_>,_>>()?;
        tiles.push((tokens[0], tokens[1]));
    }

    Ok(tiles)
}
