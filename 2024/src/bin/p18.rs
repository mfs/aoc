use std::io::{self, BufRead};
use std::collections::{VecDeque,HashSet};

use anyhow::Result;

const SIZE: i32 = 70;

type V = (i32, i32);
type Bytes = Vec<V>;

fn main() -> Result<()> {
    let bytes = parse()?;

    let corrupted: HashSet<V> = bytes.iter().take(1024).cloned().collect();

    println!("Part 1: {}", bfs(&corrupted, (0, 0), (SIZE, SIZE)).1);

    let mut left =  1024;
    let mut right = bytes.len();

    // binary search for number of bytes to first path block
    // https://en.wikipedia.org/wiki/Binary_search#Procedure_for_finding_the_leftmost_element
    while left < right {
        let m = (left + right) / 2;
        let corrupted: HashSet<V> = bytes.iter().take(m).cloned().collect();
        if bfs(&corrupted, (0, 0), (SIZE, SIZE)).0 {
            left = m + 1;
        } else {
            right = m;
        }
    }

    // last byte index
    let idx = left - 1;

    println!("Part 2: {},{}", bytes[idx].0, bytes[idx].1);

    Ok(())
}

fn bfs(corrupted: &HashSet<V>, start: V, end: V) -> (bool, i32) {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    seen.insert(start);
    queue.push_back((0, start));

    while let Some((cost, pos)) = queue.pop_front() {
        if pos == end {
            return (true, cost);
        }

        for dir in [(1, 0), (-1, 0), (0, -1), (0, 1)] {
            let np = (pos.0 + dir.0, pos.1 + dir.1);

            if np.0 < 0 || np.0 > SIZE || np.1 < 0 || np.1 > SIZE {
                continue;
            }

            if  !seen.contains(&np) && !corrupted.contains(&np) {
                seen.insert(np);
                queue.push_back((cost + 1, np));
            }
        }
    }

    (false, 0)
}

fn parse() -> Result<Bytes> {
    let mut bytes = vec![];

    for line in io::stdin().lock().lines() {
        let tokens: Vec<i32> = line?
            .split(',')
            .map(|x| x.parse())
            .collect::<Result<_, _>>()?;

        bytes.push((tokens[0], tokens[1]));
    }

    Ok(bytes)
}
