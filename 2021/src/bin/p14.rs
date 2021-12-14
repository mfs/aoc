use anyhow::{anyhow, Result};
use std::io::{self, BufRead};
use std::collections::HashMap;


fn main() -> Result<()> {

    let mut rules: HashMap<Vec<char>, char> = HashMap::new();
    let mut template = String::new();

    for line in io::stdin().lock().lines() {
        let line = line?;

        let rule: Vec<_> = line.split(" -> ").collect();

        if rule.len() == 2 {
            let pair = rule[0].chars().collect::<Vec<char>>();
            let insert = rule[1].chars().nth(0).ok_or(anyhow!("malformed rule"))?;
            rules.insert(pair, insert);
        } else if line.len() > 2 {
            template = line.to_owned();
        }
    }

    println!("Part 1: {}", process(&rules, &template, 10)?);

    println!("Part 2: {}", process(&rules, &template, 40)?);

    Ok(())
}

fn process(rules: &HashMap<Vec<char>, char>, template: &str, steps: usize) -> Result<usize> {
    let first = template.chars().nth(0).ok_or(anyhow!("empty template"))?;
    let last = template.chars().last().ok_or(anyhow!("empty template"))?;

    let mut t = HashMap::new();
    for p in template.chars().collect::<Vec<_>>().windows(2) {
        *t.entry(p.to_vec()).or_insert(0) += 1;
    }

    for _ in 0..steps {
        let mut new_t = t.clone();

        // loop over contents of t
        for (pair, count) in t {
            if let Some(x) = rules.get(&pair) {
                // remove current pair
                *new_t.entry(pair.to_vec()).or_insert(0) -= count;
                // update new pairs
                *new_t.entry([pair[0], *x].to_vec()).or_insert(0) += count;
                *new_t.entry([*x, pair[1]].to_vec()).or_insert(0) += count;
            }
        }

        t = new_t;
    }

    // we double counted all but first and last so add 1 to that pair
    *t.entry([first, last].to_vec()).or_insert(0) += 1;

    let mut totals: HashMap<char, usize> = HashMap::new();
    for (pair, count) in t {
        *totals.entry(pair[0]).or_insert(0) += count;
        *totals.entry(pair[1]).or_insert(0) += count;
    }

    let min = totals.values().min().ok_or(anyhow!("missing totals"))?;
    let max = totals.values().max().ok_or(anyhow!("missing totals"))?;

    Ok((max - min) / 2)
}
