use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
struct Rule {
    idx: usize,
    op: fn(u32, u32) -> bool,
    val: u32,
    dest: String,
}

type Workflow = Vec<Rule>;

type Part = [u32; 4];

fn main() -> Result<()> {
    let (workflows, parts) = parse()?;

    let mut sum = 0;

    'outer: for part in &parts {
        let mut name = "in";

        'inner: loop {
            for rule in &workflows[name] {
                if (rule.op)(part[rule.idx], rule.val) {
                    if rule.dest == "A" {
                        sum += part.iter().sum::<u32>();
                        continue 'outer;
                    } else if rule.dest == "R" {
                        continue 'outer;
                    }
                    name = &rule.dest;
                    continue 'inner;
                }
            }
        }
    }

    println!("Part 1: {}", sum);

    let sum = count(
        &workflows,
        [(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
        "in",
    );

    println!("Part 2: {}", sum);

    Ok(())
}

fn count(workflows: &HashMap<String, Workflow>, mut ranges: [(u32, u32); 4], name: &str) -> u64 {
    // terminal cases
    if name == "R" {
        return 0;
    } else if name == "A" {
        return ranges
            .iter()
            .map(|r| r.1 as u64 - r.0 as u64 + 1)
            .product::<u64>();
    }

    // recursive cases

    let mut counts = 0u64;

    for rule in &workflows[name] {
        // early out if on last rule
        // for last rule op is a func that always return true
        if (rule.op)(0, 1) && (rule.op)(1, 0) {
            counts += count(workflows, ranges, &rule.dest);
            return counts;
        }

        let (lo, hi) = ranges[rule.idx];

        let (true_range, false_range) = if (rule.op)(0, 1) {
            // less than op
            ((lo, rule.val - 1), (rule.val, hi))
        } else {
            // greater than op
            ((rule.val + 1, hi), (lo, rule.val))
        };

        if true_range.0 <= true_range.1 {
            ranges[rule.idx] = true_range;
            counts += count(workflows, ranges, &rule.dest);
        }

        if false_range.0 <= false_range.1 {
            ranges[rule.idx] = false_range;
        }
    }

    counts
}

fn parse() -> Result<(HashMap<String, Workflow>, Vec<Part>)> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let sections: Vec<_> = buffer.split("\n\n").collect();

    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    for line in sections[0].lines() {
        let tokens: Vec<_> = line.trim_matches('}').split(&['{', ','][..]).collect();

        let mut rules = vec![];

        for rule in &tokens[1..] {
            let rule_tokens: Vec<_> = rule.split(':').collect();
            if rule_tokens.len() == 1 {
                // last rule
                rules.push(Rule {
                    idx: 0, // dummy
                    op: |_, _| true,
                    val: 0, // dummy
                    dest: rule_tokens[0].to_owned(),
                });
            } else {
                // normal rule
                let idx = match &rule_tokens[0][0..1] {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => unreachable!(),
                };

                let val: u32 = rule_tokens[0][2..].parse()?;

                let op = match &rule_tokens[0][1..2] {
                    ">" => move |a: u32, b: u32| a > b,
                    "<" => move |a: u32, b: u32| a < b,
                    _ => unreachable!(),
                };

                rules.push(Rule {
                    idx,
                    op,
                    val,
                    dest: rule_tokens[1].to_owned(),
                });
            }
        }

        workflows.insert(tokens[0].to_owned(), rules);
    }

    // parse parts
    // assumes x,m,a,s are in order
    let mut parts: Vec<Part> = vec![];

    for line in sections[1].lines() {
        let r: Vec<_> = line
            .split(',')
            .map(|s| s.trim_matches(&['{', '}', '=', 'x', 'm', 'a', 's'][..]))
            .map(|s| s.parse())
            .collect::<Result<_, _>>()?;

        parts.push([r[0], r[1], r[2], r[3]]);
    }

    Ok((workflows, parts))
}
