use anyhow::Result;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Path {
    twice: bool,
    nodes: Vec<String>,
}

impl Path {
    fn new() -> Self {
        Path {
            twice: false,
            nodes: vec!["start".to_owned()],
        }
    }
}

fn main() -> Result<()> {
    let mut rules = HashMap::new();

    for line in io::stdin().lock().lines() {
        let line = line?;
        let rule: Vec<String> = line.split('-').map(|x| x.to_owned()).collect();

        (*rules.entry(rule[0].clone()).or_insert(vec![])).push(rule[1].clone());

        (*rules.entry(rule[1].clone()).or_insert(vec![])).push(rule[0].clone());
    }

    let paths: Vec<Path> = vec![Path::new()];

    println!("Part 1: {}", generate_paths(paths.clone(), &rules, false));

    println!("Part 2: {}", generate_paths(paths, &rules, true));

    Ok(())
}

fn generate_paths(mut paths: Vec<Path>, rules: &HashMap<String, Vec<String>>, twice: bool) -> usize {
    loop {
        let mut new_paths: Vec<Path> = vec![];

        let mut exit = true;

        for path in &paths {
            if let Some(last) = path.nodes.iter().last() {
                if last == "end" {
                    new_paths.push(Path{ twice: path.twice, nodes: path.nodes.clone()});
                    continue;
                } else if !rules.contains_key(last) {
                    continue;
                }

                for next in &rules[last] {
                    let mut path_copy = path.clone();

                    if add_node(&mut path_copy, &next, twice) {
                        new_paths.push(path_copy);
                        exit = false;
                    }
                }
            }
        }

        paths = new_paths;

        if exit {
            break;
        }
    }

    paths.len()
}

// probably a more concise way of stating this logic but I find this
// easy to understand
fn add_node(path: &mut Path, node: &str, twice: bool) -> bool {
    if node == "start" {
        // don't revisit start
        return false;
    } else if !is_lower(node) {
        // can always insert uppercase
        path.nodes.push(node.to_owned());
        return true;
    }

    // could speed up by maintaining a set per path
    // with already present nodes, this isn't too bad though
    let present = path.nodes.contains(&node.to_owned());

    // part 1: insert if lower case and not present
    if !twice && is_lower(node) && !present {
        path.nodes.push(node.to_owned());
        return true;
    }

    // part 2: if lowercase and path.twice is false insert, update path.twice if required
    if is_lower(node) && !path.twice && twice  {
        path.twice = present;
        path.nodes.push(node.to_owned());
        return true;
    }

    // part 2: if lowercase and path.twice is true insert if not already present
    if is_lower(node) && path.twice && !present && twice {
        path.nodes.push(node.to_owned());
        return true;
    }

    false
}

fn is_lower(s: &str) -> bool {
    s.chars().all(|x| x.is_ascii_lowercase())
}
