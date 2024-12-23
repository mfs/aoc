use std::collections::{HashMap, HashSet, BTreeSet};
use std::io::{self, BufRead};

use anyhow::Result;

type Network = HashMap<String, HashSet<String>>;
type Nodes = HashSet<String>;

fn main() -> Result<()> {
    let network = parse()?;

    println!("Part 1: {}", part1(&network));

    println!("Part 2: {}", part2(&network));

    Ok(())
}

fn part1(network: &Network) -> usize {
   let mut triples: HashSet<BTreeSet<String>> = HashSet::new();

    // c0 -> c1 -> c2
    // if c0 != c2 and c2 points back to c0 this is a triple

    for c0 in network.keys() {
        for c1 in &network[c0] {
            for c2 in &network[c1] {
                if c0 != c2 && network[c2].contains(c0) {
                    triples.insert(BTreeSet::from([c0.clone(), c1.clone(), c2.clone()]));
                }
            }
        }
    }

    triples
        .iter()
        .filter(|&x| x.iter().any(|s| s.starts_with("t")))
        .count()
}

fn part2(network: &Network) -> String {
    let r = HashSet::new();
    let mut p: HashSet<String> = network.keys().cloned().collect();
    let mut x = HashSet::new();

    let mut max_clique = Nodes::new();

    bron_kerbosch(network, r, &mut p, &mut x, &mut max_clique);

    let mut computers: Vec<String> = max_clique.iter().cloned().collect();
    computers.sort_unstable();

    computers.join(",")
}

fn bron_kerbosch(network: &Network, r: Nodes, p: &mut Nodes, x: &mut Nodes, max_clique: &mut Nodes) {
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            *max_clique = r.clone();
        }
        return;
    }

    for v in p.clone() {
        let r_next = &r | &HashSet::from([v.clone()]);

        let n = &network[&v];

        let mut p_next = p.intersection(&n).cloned().collect();

        let mut x_next = x.intersection(&n).cloned().collect();

        bron_kerbosch(network, r_next, &mut p_next, &mut x_next, max_clique);

        p.remove(&v);
        x.insert(v);
    }
}

fn parse() -> Result<Network> {
    let mut network = Network::new();

    for line in io::stdin().lock().lines() {
        let line = line?;

        let tokens: Vec<_> = line.split('-').collect();

        network.entry(tokens[0].to_owned()).or_default().insert(tokens[1].to_owned());
        network.entry(tokens[1].to_owned()).or_default().insert(tokens[0].to_owned());
    }

    Ok(network)
}
