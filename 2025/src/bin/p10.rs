use std::io::{self, BufRead};
use std::collections::{HashSet, VecDeque};
use std::iter::zip;

use anyhow::Result;

fn main() -> Result<()> {
    let machines = parse()?;

    println!("Part 1: {}", part1_bfs(&machines));

    println!("Part 2: {}", part2_ilp(&machines)?);

    Ok(())
}

fn part1_bfs(machines: &Vec<Machine>) -> i32 {
    let mut count = 0;

    for m in machines {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((0, vec![false; m.lights.len()]));

        while let Some((n, l)) = queue.pop_front() {
            // check for exit
            if l == m.lights {
                count += n;
                break;
            }

            // else toggle according to all buttons and add new nodes
            for b in &m.buttons {
                let nl = toggle(&l, &b);

                if seen.insert((n + 1, nl.clone())) {
                    queue.push_back((n + 1, nl.clone()));
                }
            }
        }
    }

    count
}

// ILP solution from SuperSmurfen on Reddit
// https://www.reddit.com/r/adventofcode/comments/1pity70/comment/nt8w6qx/
fn part2_ilp(machines: &Vec<Machine>) -> Result<i64> {
    use good_lp::*;
    let mut count = 0i64;

    for m in machines {
        // variables
        let mut vars = variables!();

        let mut press_vars = vec![];
        for _ in 0..m.buttons.len() {
            press_vars.push(vars.add(variable().min(0).integer()));
        }

        let mut problem = highs(vars.minimise(press_vars.iter().sum::<Expression>()));

        // expressions
        let mut exprs = vec![0.into_expression(); m.joltage.len()];
        for btns_idx in 0..m.buttons.len() {
            for &jlt_idx in &m.buttons[btns_idx] {
                exprs[jlt_idx] += press_vars[btns_idx];
            }
        }

        // add constraints
        for (exp, jolt) in zip(exprs.into_iter(), m.joltage.iter()) {
            problem.add_constraint(exp.eq(*jolt as f64));
        }

        let sol = problem.solve()?;

        // add up all button presses
        count += press_vars.iter().map(|&v| sol.value(v)).sum::<f64>() as i64;
    }

    Ok(count)
}

fn toggle(lights: &Vec<bool>, buttons: &Vec<usize>) -> Vec<bool> {
    let mut nl = lights.clone();

    for b in buttons {
        nl[*b] = !nl[*b];
    }

    nl
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

fn parse() -> Result<Vec<Machine>> {
    let mut machines = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        if !line.contains('[') {
            continue;
        }

        let tokens: Vec<_> = line.split_whitespace().collect();

        let mut lights = vec![];
        let mut buttons = vec![];
        let mut joltage = vec![];

        for t in &tokens {
            // lights
            if t.starts_with('[') {
                lights = t
                    .chars()
                    .filter(|&c| c == '#' || c == '.')
                    .map(|c| c == '#')
                    .collect();
            }

            // buttons
            if t.starts_with('(') {
                let bl: Vec<usize> = t
                    .trim_matches(&['(', ')']).split(',')
                    .map(|s| s.parse())
                    .collect::<Result<_,_>>()?;
                buttons.push(bl);
            }

            // joltage
            if t.starts_with('{') {
                joltage = t
                    .trim_matches(&['{', '}']).split(',')
                    .map(|s| s.parse())
                    .collect::<Result<_,_>>()?;
            }
        }

        machines.push(Machine {lights, buttons, joltage});
    }

    Ok(machines)
}
