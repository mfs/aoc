use anyhow::Result;
use num::integer::lcm;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut lines = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;
        if line == "" {
            continue;
        }
        lines.push(line);
    }

    let instructions = &lines[0];

    let mut nodes = HashMap::new();

    for line in &lines[1..] {
        let left_right: Vec<_> = line.split('=').collect();
        let right: Vec<_> = left_right[1].trim().split(',').collect();
        nodes.insert(
            left_right[0].trim(),
            (
                right[0].trim_matches('('),
                right[1].trim().trim_matches(')'),
            ),
        );
    }

    // part 1
    let mut steps = 0;

    let mut cur_node = "AAA";

    for instruction in instructions.chars().cycle() {
        if cur_node == "ZZZ" {
            break;
        }

        cur_node = match instruction {
            'L' => nodes[cur_node].0,
            'R' => nodes[cur_node].1,
            _ => unreachable!(),
        };

        steps += 1;
    }

    println!("Part 1: {}", steps);

    // part 2.
    // Inspected the individual current_nodes and saw they each cycled at a certain step
    // count including the first cycle. Find the cycles for each of current_nodes and
    // then use lcm to find the first step where they all sync.
    let mut current_nodes: Vec<_> = nodes.keys().filter(|s| s.ends_with("A")).collect();

    let mut cycles = vec![0u64; current_nodes.len()];

    for (step, instruction) in instructions.chars().cycle().enumerate() {
        if cycles.iter().all(|&x| x != 0) {
            break;
        }

        for (i, node) in current_nodes.iter_mut().enumerate() {
            if node.ends_with("Z") && cycles[i] == 0 {
                cycles[i] = step as u64;
            }
            *node = match instruction {
                'L' => &nodes[*node].0,
                'R' => &nodes[*node].1,
                _ => unreachable!(),
            };
        }
    }

    if let Some(part2) = cycles.into_iter().reduce(|acc, x| lcm(acc, x)) {
        println!("Part 2: {}", part2);
    }

    Ok(())
}
