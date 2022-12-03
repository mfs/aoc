use anyhow::Result;

fn main() -> Result<()> {
    println!("Advent of Code 2022!");

    /* read input into String
    use std::io::{self, Read};
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;
    */

    /* process stdin as lines
    use std::io::{self, BufRead};
    for line in io::stdin().lock().lines() {
        println!("--- {}", line?);
    }
    */

    /* process stdin as chars (ascii only)
    use std::io::{self, Read};
    for byte in io::stdin().lock().bytes() {
        println!("{}", byte? as char);
    }
    */

    /* process stdin as lines of space separated ints
    use std::io::{self, BufRead};
    for line in io::stdin().lock().lines() {
        let x: Vec<u64> = line?
            .split(' ')
            .map(|x| x.parse())
            .collect::<Result<_, _>>()?;
        println!("{:?}", x);
    }
    */

    /* process stdin using split and regex
    use std::io::{self, Read};
    use regex::Regex;
    let mut s = String::new();
    io::stdin().lock().read_to_string(&mut s)?;

    let re = Regex::new(r"([LR])(\d+)")?;

    for e in s.split(",") {
        if let Some(caps) = re.captures(e) {
            let r = &caps[1];
            let d: i64 = caps[2].parse()?;
        }
    }
    */

    Ok(())
}
