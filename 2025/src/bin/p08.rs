use std::io::{self, BufRead};
use std::cmp::Reverse;

use anyhow::Result;

type JuncBox = (i64, i64, i64);
type DSP = Vec<usize>; // Disjoint Set Parents

fn main() -> Result<()> {
    let boxes = parse()?;

    // vec of (dist squared, idx0, idx1) where idxX is boxes[idxX]
    let mut distances = vec![];

    for (idx0, jb0) in boxes.iter().enumerate() {
        for (idx1, jb1) in boxes.iter().enumerate() {
            // filter out duplicates due to ordering and where jb0 == jb1
            if idx0 < idx1 {
                distances.push((dist2(*jb0, *jb1), idx0, idx1));
            }
        }
    }

    distances.sort_unstable();

    // https://en.wikipedia.org/wiki/Disjoint-set_data_structure
    let mut parent: DSP = (0..boxes.len()).collect();

    let mut connections = 0;

    for (idx, (_, idx0, idx1)) in distances.iter().enumerate() {
        if idx == 1000 {
            let s = sizes(&mut parent);
            println!("Part 1: {}", s[0] * s[1] * s[2]);
        }

        if find(*idx0, &mut parent) != find(*idx1, &mut parent) {
            // different roots so different sets
            union(*idx0, *idx1, &mut parent);
            connections += 1;
            if connections == boxes.len() - 1 {
                println!("Part 2: {}", boxes[*idx0].0 * boxes[*idx1].0);
                break;
            }
        }
    }

    Ok(())
}

// returns the root of the set using path compression
fn find(x: usize, parent: &mut DSP) -> usize {
    if parent[x] != x {
        let tmp = find(parent[x], parent);
        parent[x] = tmp;
        return parent[x];
    } else {
        return x;
    }
}

// joins two sets by finding the roots and making one the root of the other
fn union(x: usize, y: usize, parent: &mut DSP) {
    let y = find(y, parent);

    parent[y] = find(x, parent);
}

// returns sizes of sets sorted in reverse order
fn sizes(parent: &mut DSP) -> Vec<usize> {
    let mut sizes = vec![0; parent.len()];

    for i in 0..sizes.len() {
        sizes[find(i, parent)] += 1;
    }

    sizes.sort_by_key(|x| Reverse(*x));

    sizes
}

fn dist2(a: JuncBox, b: JuncBox) -> i64 {
    ((b.0 - a.0).pow(2)) + ((b.1 - a.1).pow(2)) + ((b.2 - a.2).pow(2))
}

fn parse() -> Result<Vec<(i64, i64, i64)>> {
    let mut boxes = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        let ns: Vec<_> = line
            .split(',')
            .map(|s| s.parse::<i64>())
            .collect::<Result<_, _>>()?;

        boxes.push((ns[0], ns[1], ns[2]));
    }

    Ok(boxes)
}
