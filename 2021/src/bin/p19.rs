use std::io::{self, BufRead};
use std::collections::HashSet;
use anyhow::Result;
use regex::Regex;
use itertools::iproduct;

type Vec3 = [i64; 3];

#[derive(Debug, Default)]
struct Scanner {
    done: bool,
    position: Vec3,
    beacons: Vec<Vec3>,
}


// hack: 48 orderings and axis negations, not 24. Some are invalid. Less efficient, still works. :)
const ORDER: [Vec3; 6] = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
const NEG: [Vec3; 8] = [
    [1, 1, 1],
    [1, 1, -1],
    [1, -1, 1],
    [1, -1, -1],
    [-1, 1, 1],
    [-1, 1, -1],
    [-1, -1, 1],
    [-1, -1, -1],
];

fn transform(order: Vec3, negate: Vec3, beacons: &Vec<Vec3>) -> Vec<Vec3> {
    let t = |i, v: Vec3| {negate[i] * v[order[i] as usize]};

    beacons.iter().map(|v| [t(0, *v), t(1, *v), t(2, *v)]).collect()
}

fn sub(a: &Vec3, b: &Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn manhatten(a: &Vec3, b: &Vec3) -> i64 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
}

fn align(a: &Vec<Vec3>, b: &Vec<Vec3>) -> Option<(Vec<Vec3>, Vec3)> {
    for order in ORDER {
        for neg in NEG {
            let b_r = transform(order, neg, &b);
            for a_pt in a {
                for b_pt in &b_r {
                    let transform_vec = sub(b_pt, a_pt);
                    let mut matches = 0;
                    let mut transformed = vec![];
                    for ob in &b_r {
                        let reverse_transform = sub(ob, &transform_vec);
                        if a.contains(&reverse_transform) {
                            matches += 1;
                        }
                        transformed.push(reverse_transform);
                    }
                    if matches >= 12 {
                        return Some((transformed, transform_vec));
                    }
                }
            }
        }
    }
    None
}

fn main() -> Result<()> {
    let mut scanners = parse()?;

    let mut all_beacons: HashSet<Vec3> = HashSet::new();

    // scanner 0 is the reference point
    for b in &scanners[0].beacons {
        all_beacons.insert(b.clone());
    }
    scanners[0].done = true;

    while !scanners.iter().all(|x| x.done) {
        // i should be not done, j should be done
        for i in 0..scanners.len() {
            if scanners[i].done {
                continue;
            }
            for j in (0..scanners.len()).filter(|x| scanners[*x].done) {
                if let Some((transform, loc)) = align(&scanners[j].beacons, &scanners[i].beacons) {
                    scanners[i].done = true;
                    scanners[i].beacons = transform.clone();
                    scanners[i].position = loc;
                    for pt in &transform {
                        all_beacons.insert(*pt);
                    }
                    break;
                }
            }
        }
    }

    println!("Part 1: {}", all_beacons.len());

    let mut distances: Vec<_> = iproduct!(&scanners, &scanners)
        .map(|(a, b)| manhatten(&a.position, &b.position))
        .collect();

    distances.sort_by(|a, b| b.cmp(a)); // reverse

    println!("Part 2: {}", distances[0]);

    Ok(())
}

fn parse()-> Result<Vec<Scanner>> {
    let mut scanners = vec![];

    let re = Regex::new(r"(-?\d+),(-?\d+),(-?\d+)")?;

    for line in io::stdin().lock().lines() {
        let line = line?;

        let cur = scanners.len() - 1;

        if line.starts_with("---") {
            scanners.push(Scanner::default());
        } else {
            if let Some(caps) = re.captures(&line) {
                scanners[cur].beacons.push([
                    caps[1].parse()?,
                    caps[2].parse()?,
                    caps[3].parse()?,
                ]);
            }
        }
    }

    Ok(scanners)
}

