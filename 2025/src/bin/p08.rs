use std::io::{self, BufRead};
use std::collections::{HashMap};
use std::cmp::Reverse;

use anyhow::Result;

type JuncBox = (i64, i64, i64);

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
    // could probably just use a Vec here for performance
    let mut disjoint_set = HashMap::new();
    for i in 0..boxes.len() {
        disjoint_set.insert(i, i);
    }

    let mut connections = 0;

    for (idx, (_, idx0, idx1)) in distances.iter().enumerate() {
        if idx == 1000 {
            let s = sizes(&mut disjoint_set);
            println!("Part 1: {}", s[0] * s[1] * s[2]);
        }

        if find(*idx0, &mut disjoint_set) != find(*idx1, &mut disjoint_set) {
            // different roots so different sets
            union(*idx0, *idx1, &mut disjoint_set);
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
fn find(x: usize, disjoint_set: &mut HashMap<usize, usize>) -> usize {
    if disjoint_set[&x] != x {
        let tmp = find(disjoint_set[&x], disjoint_set);
        disjoint_set.insert(x, tmp);
        return disjoint_set[&x];
    } else {
        return x;
    }
}

// joins two sets by finding the roots and making one the root of the other
fn union(x: usize, y: usize, disjoint_set: &mut HashMap<usize, usize>) {
    let x = find(x, disjoint_set);
    let y = find(y, disjoint_set);

    disjoint_set.insert(y, x);
}

// returns sizes of sets sorted in reverse order
fn sizes(disjoint_set: &mut HashMap<usize, usize>) -> Vec<usize> {
    let mut sizes = HashMap::new();

    for i in 0..disjoint_set.len() {
        *sizes.entry(find(i, disjoint_set)).or_insert(0) += 1;
    }

    let mut sv: Vec<_> = sizes.into_values().collect();
    sv.sort_by_key(|x| Reverse(*x));

    sv
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
