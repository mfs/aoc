use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::Result;

type Reports = Vec<Vec<i64>>;

fn main() -> Result<()> {
    let reports = parse()?;

    let part1 = reports.iter().filter(|r| is_safe(r)).count();

    println!("Part 1: {}", part1);

    let part2 = reports.iter().filter(|r| is_safe_dampener(r)).count();

    println!("Part 2: {}", part2);

    Ok(())
}

fn is_safe_dampener(report: &[i64]) -> bool {
    // if the original report is safe, it remains safe when removing the first or
    // last item so we don't need to check the original report
    for i in 0..report.len() {
        let mut r = report.to_vec();
        r.remove(i);
        if is_safe(&r) {
            return true;
        }
    }

    false
}

fn is_safe(report: &[i64]) -> bool {
    let deltas: Vec<_> = report.windows(2).map(|v| v[1] - v[0]).collect();

    if !deltas.iter().all(|&n| n < 0) && !deltas.iter().all(|&n| n > 0)  {
        return false;
    }

    deltas.iter().all(|&n| n.abs() <= 3)
}

fn parse() -> Result<Reports> {
    let mut reports = vec![];

    for line in io::stdin().lock().lines() {
        let x: Vec<i64> = line?
            .split_whitespace()
            .map(i64::from_str)
            .collect::<Result<_, _>>()?;
        reports.push(x);
    }

    Ok(reports)
}
