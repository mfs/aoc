use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(PartialEq, Eq, Hash, Debug)]
struct Chemical {
    name: String,
    amount: u32,
}

// reverse map

type Reactions = HashMap<Chemical, Vec<Chemical>>;

fn main() -> Result<()> {
    let reactions = parse()?;

    for (k, v) in &reactions {
        println!("{:?} - {:?}", k, v);
    }

    println!("Part 1: {}", process("FUEL", 1, &reactions));

    Ok(())
}

fn process(chem: &str, amount: u32, r: &Reactions) -> u32 {
    // terminal cases
    if chem == "ORE" {
        return amount;
    }

    // recursive cases
    let mut total = 0;

    // for each chem in r[chem]
    //

    let ch = Chemical {
        name: chem.to_owned(),
        amount,
    };

    for c in &r[&ch] {
        total += amount * process(&c.name, c.amount, r);
    }

    total
}

fn parse() -> Result<Reactions> {
    let mut reactions = Reactions::new();

    for line in io::stdin().lock().lines() {
        let line = line?;

        let tokens: Vec<_> = line.split(" => ").collect();

        let r: Vec<_> = tokens[1].split_whitespace().collect();

        let k = Chemical {
            name: r[1].to_owned(),
            amount: r[0].parse()?,
        };

        let mut chemicals = vec![];

        for chem in tokens[0].split(", ") {
            let aa: Vec<_> = chem.split_whitespace().collect();
            // add to vec
            let ck = Chemical {
                name: aa[1].to_owned(),
                amount: aa[0].parse()?,
            };

            chemicals.push(ck);
        }

        //reactions
        reactions.insert(k, chemicals);
    }

    Ok(reactions)
}
