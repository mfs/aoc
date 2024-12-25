use std::collections::HashMap;
use std::io::{self, BufRead};

use anyhow::Result;

#[derive(Debug, Clone)]
enum Gate {
    And(String, String),
    Or(String, String),
    Xor(String, String),
    In(u8),
}
use std::collections::HashSet;
type Gates = HashMap<String, Gate>;
type HS = HashSet<String>;

fn main() -> Result<()> {
    let gates = parse()?;

    println!("Part 1: {}", bus(&gates, 'z', &mut HS::new()));
    println!("Part 2: {}", part2(&gates));

    println!();
    gprint(&gates, "z00", 0);
    println!();
    gprint(&gates, "z01", 0);
    println!();
    gprint(&gates, "z02", 0);
    println!();

    Ok(())
}

fn part2(gates: &Gates) -> String {
    let mut incorrect = HashSet::new();

    let is_bus = |s: &str| { s.starts_with("x") || s.starts_with("y") || s.starts_with("z") };

    let mut and_inputs: HashSet<String> = HashSet::new();
    let mut xor_inputs: HashSet<String> = HashSet::new();
    let mut or_inputs: HashSet<String> = HashSet::new();

    for  v in gates.values() {
        match v {
            Gate::Or(a, b) => or_inputs.extend([a.to_owned(), b.to_owned()]),
            Gate::And(a, b) => and_inputs.extend([a.to_owned(), b.to_owned()]),
            Gate::Xor(a, b) => xor_inputs.extend([a.to_owned(), b.to_owned()]),
            Gate::In(_) => {},
        }
    }

    for (k, v) in gates {
        match v {
            Gate::Or(_, _) => {
                // output gates are always xor except the last
                // any or gate connected to z bus except z45 is incorrect
                if k.starts_with("z") && k != "z45" {
                    incorrect.insert(k.to_owned());
                }
            },
            Gate::And(a, _) => {
                // output gates are always xor or or for z45
                // any and gate connected to z bus is incorrect
                if k.starts_with("z") {
                    incorrect.insert(k.to_owned());
                }
                // and gates only connect to or gates except the x00/y00 and gate
                if a != "x00" && a!= "y00" && (xor_inputs.contains(k) || and_inputs.contains(k)) {
                    incorrect.insert(k.to_owned());
                }
            },
            Gate::Xor(a, b) => {
                // an xor gate not connected to any i/o bus is incorrect
                if (!is_bus(a) && !is_bus(b) && !is_bus(k)) || or_inputs.contains(k) {
                    incorrect.insert(k.to_owned());
                }
            }
            Gate::In(_) => {},
        }
    }

    let mut v: Vec<_> = incorrect.iter().cloned().collect();
    v.sort();

    v.join(",")
}

fn gprint(gates: &Gates, g: &str, level: usize) {
    let spc = "                      ";
    match &gates[g] {
        Gate::In(_) => println!("{}{}", &spc[..level],  g),
        Gate::And(a, b) => {
            println!("{}AND {}", &spc[..level], g);
            gprint(gates, a, level + 1);
            gprint(gates, b, level + 1);
        },
        Gate::Or(a, b) => {
            println!("{}OR {}", &spc[..level], g);
            gprint(gates, a, level + 1);
            gprint(gates, b, level + 1);
        },
        Gate::Xor(a, b) => {
            println!("{}XOR {}", &spc[..level], g);
            gprint(gates, a, level + 1);
            gprint(gates, b, level + 1);
        },
    }
}

fn bus(gates: &Gates, c: char, store: &mut HS) -> u64 {
    let mut n = 0;
    for (g, _) in gates {
        if g.starts_with(c) {
            let bit = g[1..].parse::<u32>().unwrap();
            n |= (eval(&gates, &g, store) as u64) << bit;
        }
    }

    n
}

fn eval(gates: &Gates, output: &str, store: &mut HS) -> u8 {
    let z = match &gates[output] {
        Gate::In(x) => *x,
        Gate::And(a, b) => eval(gates, &a, store) & eval(gates, &b, store),
        Gate::Or(a, b) => eval(gates, &a, store) | eval(gates, &b, store),
        Gate::Xor(a, b) => eval(gates, &a, store) ^ eval(gates, &b, store),
    };

    if z == 1 && !output.starts_with("x") && !output.starts_with("y") {
        store.insert(output.to_owned());
    }

    return z;
}

fn parse() -> Result<HashMap<String, Gate>> {
    let mut gates = HashMap::new();

    for line in io::stdin().lock().lines() {
        let line = line?;

        let tokens: Vec<_> = line.split_whitespace().collect();

        if tokens.len() == 2 {
            gates.insert(tokens[0].trim_matches(':').to_owned(), Gate::In(tokens[1].parse()?));
        } else if tokens.len() == 5 {
            let g = match tokens[1] {
                "AND" => Gate::And(tokens[0].to_owned(), tokens[2].to_owned()),
                "OR" => Gate::Or(tokens[0].to_owned(), tokens[2].to_owned()),
                "XOR" => Gate::Xor(tokens[0].to_owned(), tokens[2].to_owned()),
                _ => unreachable!(),
            };

            gates.insert(tokens[4].to_owned(), g);
        }
    }

    Ok(gates)
}
