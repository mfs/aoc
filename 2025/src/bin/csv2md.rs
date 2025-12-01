use anyhow::Result;
use std::io::{self, BufRead};

fn main() -> Result<()> {
    let mut rows: Vec<Vec<String>> = vec![];

    for line in io::stdin().lock().lines() {
        rows.push(line?.split(',').map(String::from).collect());
    }

    println!("| {} |", rows[0].join(" | "));
    println!("| :--- | :--- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
    for r in &rows[1..] {
        println!("| {} |", r.join(" | "));
    }

    Ok(())
}
