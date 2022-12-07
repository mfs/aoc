use anyhow::{anyhow, Result};
use std::io::{self, BufRead};

const DIR_LIMIT: u64 = 100_000;
const DISK_SIZE: u64 = 70_000_000;
const MIN_FREE: u64 = 30_000_000;

#[derive(Debug, Default)]
struct Node {
    name: String,
    size: u64, // if size > 0 this is a file, dir otherwise
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    fn mk_dir(name: &str, parent: Option<usize>) -> Self {
        Node {
            name: name.to_owned(),
            parent,
            ..Default::default()
        }
    }

    fn mk_file(name: &str, size: u64, parent: Option<usize>) -> Self {
        Node {
            name: name.to_owned(),
            size,
            parent,
            ..Default::default()
        }
    }
}

fn main() -> Result<()> {
    let nodes = parse()?;

    let mut dir_sizes: Vec<_> = (0..nodes.len())
        .filter(|&idx| nodes[idx].size == 0)
        .map(|idx| size(idx, &nodes))
        .collect();

    let part1: u64 = dir_sizes.iter().filter(|&&n| n < DIR_LIMIT).sum();

    println!("Part 1: {}", part1);

    dir_sizes.sort_unstable();

    let unused_space = DISK_SIZE - size(0, &nodes);
    let min_dir_size = MIN_FREE - unused_space;

    if let Some(dir_size) = dir_sizes.iter().skip_while(|&&n| n < min_dir_size).nth(0) {
        println!("Part 2: {:?}", dir_size);
    }

    Ok(())
}

fn size(idx: usize, nodes: &[Node]) -> u64 {
    let mut dir_size = 0;

    for n in &nodes[idx].children {
        if nodes[*n].size > 0 {
            dir_size += nodes[*n].size;
        } else {
            dir_size += size(*n, nodes);
        }
    }

    dir_size
}

fn parse() -> Result<Vec<Node>> {
    let mut nodes: Vec<Node> = vec![];

    nodes.push(Node::mk_dir("/", None));

    let mut cur_dir = 0;

    for line in io::stdin().lock().lines() {
        let el: Vec<String> = line?.split(" ").map(|s| s.to_owned()).collect();

        if &el == &["$", "cd", "/"] || &el == &["$", "ls"] {
            continue;
        } else if el.len() == 2 {
            // file or directory
            if el[0] == "dir" {
                nodes.push(Node::mk_dir(&el[1], Some(cur_dir)));
            } else {
                nodes.push(Node::mk_file(&el[1], el[0].parse()?, Some(cur_dir)));
            }

            let last_index = nodes.len() - 1;
            nodes[cur_dir].children.push(last_index);
        } else if el[1] == "cd" {
            if el[2] == ".." {
                cur_dir = nodes[cur_dir].parent.ok_or(anyhow!("underflow"))?;
            } else {
                for child in &nodes[cur_dir].children {
                    if nodes[*child].name == el[2] {
                        cur_dir = *child;
                        break;
                    }
                }
            }
        }
    }

    Ok(nodes)
}
