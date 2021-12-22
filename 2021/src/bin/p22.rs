use std::io::{self, BufRead};
use std::cmp::{min, max};
use anyhow::Result;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    is_on: bool,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

const LIMIT: (i64, i64) = (-50, 50);

fn min_max(a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    (max(a.0, b.0), min(a.1, b.1))
}

impl Cuboid {
    fn volume(&self) -> i64 {
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }

    fn sign(&self) -> i64 {
        if self.is_on { 1 } else { -1 }
    }

    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        let (min_x, max_x) = min_max(self.x, other.x);
        let (min_y, max_y) = min_max(self.y, other.y);
        let (min_z, max_z) = min_max(self.z, other.z);

        if min_x <= max_x && min_y <= max_y && min_z <= max_z {
            Some(Cuboid { is_on: !other.is_on, x: (min_x, max_x), y: (min_y, max_y), z: (min_z, max_z) })
        } else {
            None
        }
    }

    fn trim(&self) -> Option<Cuboid> {
        // negate is_on as intersection flips it for use by calc
        let c = Cuboid { is_on: !self.is_on, x: LIMIT, y: LIMIT, z: LIMIT };

        self.intersection(&c)
    }
}

fn main() -> Result<()> {
    let cuboids = parse()?;

    let trimmed: Vec<Cuboid> = cuboids
        .iter()
        .filter(|x| x.trim().is_some())
        .map(|x| *x)
        .collect();

    println!("Part 1: {}", calc(&trimmed));
    println!("Part 2: {}", calc(&cuboids));

    Ok(())
}

fn calc(cuboids: &Vec<Cuboid>) -> i64 {
    let mut intersections: Vec<Cuboid> = vec![];

    for cuboid in cuboids {
        // calc intersection with existing cuboids, for each existing cuboid add
        // new cuboid with negative 'sign' as existing cuboid, this clears the
        // volume occupied by the new cuboid
        for idx in 0..intersections.len() {
            if let Some(i) = cuboid.intersection(&intersections[idx]) {
                intersections.push(i.clone());
            }
        }

        // if this is an 'on' cuboid add it to the list of intersections as is,
        // for 'off' cuboids we leave the volume cleared
        if cuboid.is_on {
            intersections.push(cuboid.clone());
        }
    }

    // final volume is the sum of the signed intersection volumes
    intersections.iter().map(|c| c.volume() * c.sign()).sum()
}

fn parse() -> Result<Vec<Cuboid>> {
    let mut cuboids = vec![];

    let re = Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)")?;

    for line in io::stdin().lock().lines() {
        let line = line?;
        if let Some(caps) = re.captures(&line) {
            let cuboid = Cuboid {
                is_on: &caps[1] == "on",
                x: (caps[2].parse()?, caps[3].parse()?),
                y: (caps[4].parse()?, caps[5].parse()?),
                z: (caps[6].parse()?, caps[7].parse()?),
            };
            cuboids.push(cuboid);
        }
    }

    Ok(cuboids)
}
