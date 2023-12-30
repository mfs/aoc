use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone)]
struct Brick {
    p0: (u32, u32, u32),
    p1: (u32, u32, u32),
}

fn main() -> Result<()> {
    let mut bricks = parse()?;

    bricks.sort_unstable_by_key(|b| std::cmp::min(b.p0.2, b.p1.2));

    for i in 0..bricks.len() {
        let mut max_z = 1;
        for j in 0..i {
            if overlaps_in_z(&bricks[i], &bricks[j]) {
                max_z = std::cmp::max(max_z, bricks[j].p1.2 + 1);
            }
        }

        bricks[i].p1.2 -= bricks[i].p0.2 - max_z;
        bricks[i].p0.2 = max_z;
    }

    bricks.sort_unstable_by_key(|b| std::cmp::min(b.p0.2, b.p1.2));

    // bricks are now stable and sorted from min z to max z

    let mut supports: HashMap<_, _> = (0..bricks.len()).map(|i| (i, HashSet::new())).collect();
    let mut supported_by = supports.clone();

    for j in 0..bricks.len() {
        // lower bricks
        for i in 0..j {
            if overlaps_in_z(&bricks[i], &bricks[j]) && bricks[j].p0.2 == bricks[i].p1.2 + 1 {
                supports.entry(i).or_insert(HashSet::new()).insert(j);
                supported_by.entry(j).or_insert(HashSet::new()).insert(i);
            }
        }
    }

    // part 1

    let mut sum = 0;

    'outer: for i in 0..bricks.len() {
        // for each brick j that supports brick i
        for j in &supports[&i] {
            // if it is the only brick don't count
            if supported_by[j].len() <= 1 {
                continue 'outer;
            }
        }

        sum += 1;
    }

    println!("Part 1: {}", sum);

    // part 2

    let mut sum = 0;

    for i in 0..bricks.len() {
        let mut q: VecDeque<usize> = supports[&i]
            .iter()
            .filter(|j| supported_by[j].len() == 1)
            .cloned()
            .collect();

        let mut falling: HashSet<usize> = q.iter().map(|x| *x).collect();

        while let Some(j) = q.pop_front() {
            let d: HashSet<usize> = supports[&j].difference(&falling).cloned().collect();
            for k in d {
                if supported_by[&k].is_subset(&falling) {
                    q.push_back(k);
                    falling.insert(k);
                }
            }
        }

        sum += falling.len();
    }

    println!("Part 2: {}", sum);

    Ok(())
}

fn overlaps_in_z(b0: &Brick, b1: &Brick) -> bool {
    use std::cmp::{max, min};
    max(b0.p0.0, b1.p0.0) <= min(b0.p1.0, b1.p1.0) && max(b0.p0.1, b1.p0.1) <= min(b0.p1.1, b1.p1.1)
}

fn parse() -> Result<Vec<Brick>> {
    let mut bricks = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        let n: Vec<_> = line
            .split(&['~', ','])
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        bricks.push(Brick {
            p0: (n[0], n[1], n[2]),
            p1: (n[3], n[4], n[5]),
        });
    }

    Ok(bricks)
}
