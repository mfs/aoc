use anyhow::Result;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Element {
    n: i64,
    d: i64,
}

impl Element {
    fn new(n: i64, d: i64) -> Self {
        Element { n, d }
    }
}

fn main() -> Result<()> {
    let mut numbers = vec![];

    // store elements as a (value, depth) pair. Didn't want to try representing
    // trees for this
    for line in io::stdin().lock().lines() {
        let mut depth = 0;
        let mut number = vec![];
        for c in line?.chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                '0'..='9' => number.push(Element::new(c as i64 - '0' as i64, depth)),
                _ => {},
            }
        }
        numbers.push(number);
    }

    let mut a = numbers[0].clone();

    for num in &numbers[1..] {
        a = reduce(&add(&a, num));
    }

    println!("Part 1: {}", magnitude(&a));

    let mut max = 0;
    for a in &numbers {
        for b in &numbers {
            if a == b {
                continue;
            }
            let sum = reduce(&add(a, b));
            max = std::cmp::max(max, magnitude(&sum));
        }
    }

    println!("Part 2: {}", max);

    Ok(())
}

fn add(left: &[Element], right: &[Element]) -> Vec<Element> {
    let mut sum = vec![];
    for l in left {
        sum.push(Element::new(l.n, l.d + 1));
    }
    for r in right {
        sum.push(Element::new(r.n, r.d + 1));
    }

    sum
}

fn reduce(number: &[Element]) -> Vec<Element> {
    let mut reduced = number.to_vec();

    'outer: loop {
        // explode - find left side of pair depth 5
        if let Some(idx) = reduced.iter().position(|x| x.d == 5) {
            // idx-1, [idx, idx+1], idx+2
            if idx > 0 {
                // update left neighbour
                reduced[idx - 1].n += reduced[idx].n;
            }
            if idx < reduced.len() - 2 {
                // update right neighbour
                reduced[idx + 2].n += reduced[idx + 1].n;
            }

            // set left element to 0, dec depth
            reduced[idx].n = 0;
            reduced[idx].d -= 1;

            // delete right element
            reduced.remove(idx + 1);

            continue 'outer;
        }

        // split - find element > 10
        if let Some(idx) = reduced.iter().position(|x| x.n >= 10) {
            let n = reduced[idx].n;

            reduced[idx].n = n / 2;
            reduced[idx].d += 1;

            reduced.insert(idx + 1, Element::new((n as f64 / 2f64).ceil() as i64, reduced[idx].d));

            continue 'outer;
        }

        break;
    }

    reduced
}

fn magnitude(number: &[Element]) -> i64 {
    let mut magnitudes = number.to_vec();

    'outer: while magnitudes.len() > 1 {
        for idx in 0..magnitudes.len()-1 {
            if magnitudes[idx].d == magnitudes[idx + 1].d {
                // update left element in pair to combination, dec depth
                magnitudes[idx].n = 3 * magnitudes[idx].n + 2 * magnitudes[idx + 1].n;
                magnitudes[idx].d -= 1;
                // remove second element in pair
                magnitudes.remove(idx+1);
                continue 'outer;
            }
        }
    }

    magnitudes[0].n
}
