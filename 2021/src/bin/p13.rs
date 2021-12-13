use anyhow::Result;
use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug)]
enum Fold {
    X(i64),
    Y(i64),
}

fn main() -> Result<()> {
    let mut points: HashSet<(i64, i64)> = HashSet::new();
    let mut folds: Vec<Fold> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.contains(',') {
            // parse pt
            let xy: Vec<i64> = line.split(',').map(|x| x.parse()).collect::<Result<Vec<i64>, _>>()?;
            points.insert((xy[0], xy[1]));
        } if let Some(i) = line.chars().position(|c| c == '=') {
            // parse fold
            let axis = &line[i-1..i];
            let pos = line[i+1..].parse()?;

            if axis == "x" {
                folds.push(Fold::X(pos));
            } else {
                folds.push(Fold::Y(pos));
            }
        }
    }

    for (i, fold) in folds.iter().enumerate() {
        match fold {
            &Fold::X(x) => fold_vertical(&mut points, x),
            &Fold::Y(y) => fold_horizontal(&mut points, y),
        }
        if i == 0 {
            println!("Part 1: {}", points.len());
        }
    }

    println!("Part 2:");

    for y in 0..6 {
        for x in 0..90 {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    Ok(())
}

fn fold_horizontal(points: &mut HashSet<(i64, i64)>, y: i64) {
    let mut new_points = HashSet::new();
    for pt in points.iter() {
        if pt.1 > y {
            // fold
            new_points.insert((pt.0, y - (pt.1 - y)));
        } else {
            // as is
            new_points.insert(*pt);
        }
    }

    *points = new_points;
}

fn fold_vertical(points: &mut HashSet<(i64, i64)>, x: i64) {
    let mut new_points = HashSet::new();
    for pt in points.iter() {
        if pt.0 > x {
            // fold
            new_points.insert((x - (pt.0 - x), pt.1));
        } else {
            // as is
            new_points.insert(*pt);
        }
    }

    *points = new_points;
}
