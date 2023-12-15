use anyhow::Result;
use std::io::{self, Read};

type Lense = (String, u32);
type Box = Vec<Lense>;

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    // part 1
    let p1: u32 = buffer.split(',').map(|s| hash(s.trim()) as u32).sum();

    println!("Part 1: {}", p1);

    // part 2
    let mut boxes: Vec<Box> = vec![vec![]; 256];

    // for each step
    for step in buffer.trim().split(',') {
        let label: String = step.chars().filter(|&c| c.is_alphabetic()).collect();
        let box_num = hash(&label) as usize;

        if let Some(s) = step.split('=').nth(1) {
            // add to box
            let lense_num: u32 = s.trim().parse()?;

            if let Some(p) = boxes[box_num].iter().position(|l| l.0 == label) {
                // update existing lense
                boxes[box_num][p] = (label, lense_num);
            } else {
                // add to box at end
                boxes[box_num].push((label, lense_num));
            }
        } else {
            // remove from box if present
            if let Some(p) = boxes[box_num].iter().position(|l| l.0 == label) {
                boxes[box_num].remove(p);
            }
        }
    }

    let mut p2 = 0;
    for (box_num, b) in boxes.iter().enumerate() {
        for (lense_num, lense) in b.iter().enumerate() {
            p2 += (box_num + 1) * (lense_num + 1) * lense.1 as usize;
        }
    }

    println!("Part 2: {}", p2);

    Ok(())
}

// working in ascii so as_bytes() ok
fn hash(s: &str) -> u8 {
    s.as_bytes()
        .iter()
        .fold(0, |acc: u8, &x| acc.wrapping_add(x).wrapping_mul(17))
}
