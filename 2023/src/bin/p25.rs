use anyhow::Result;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

type Graph = HashMap<String, HashSet<String>>;

fn main() -> Result<()> {
    let g = parse()?;

    let mut nodes: HashSet<String> = HashSet::new();

    for (k, v) in &g {
        nodes.insert(k.clone());
        for n in v {
            nodes.insert(n.to_owned());
        }
    }

    let mut graph = UnGraph::<&str, u32>::default();

    let node_map: HashMap<String, NodeIndex> = nodes
        .iter()
        .map(|n| (n.clone(), graph.add_node(&n)))
        .collect();

    for (k, v) in &g {
        for n in v {
            graph.add_edge(node_map[k], node_map[n], 1);
        }
    }

    let min: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
        stoer_wagner_min_cut(&graph, |_| Ok(1));

    let (_, partition) = min.unwrap().unwrap();

    println!(
        "Part 1: {}",
        ((nodes.len() - partition.len()) * partition.len())
    );

    Ok(())
}

fn parse() -> Result<Graph> {
    let mut connections: Graph = HashMap::new();

    for line in io::stdin().lock().lines() {
        let line = line?;

        let tokens: Vec<_> = line.split(": ").collect();

        for r in tokens[1].split(' ') {
            // connections are bi directional
            connections
                .entry(tokens[0].to_string())
                .or_insert(HashSet::new())
                .insert(r.to_string());

            connections
                .entry(r.to_string())
                .or_insert(HashSet::new())
                .insert(tokens[0].to_string());
        }
    }

    Ok(connections)
}
