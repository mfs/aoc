use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::cmp::{min, max};

type Pixels = HashSet<(i64, i64)>;

#[derive(Debug, Clone)]
struct Image {
    pixels: Pixels,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    void: bool,
}

impl Image {
    fn new(pixels: Pixels, void: bool) -> Self {
        fn limits(acc: (i64, i64, i64, i64), n: &(i64, i64)) -> (i64, i64, i64, i64) {
            (min(acc.0, n.0), max(acc.1, n.0), min(acc.2, n.1), max(acc.3, n.1))
        }

        let (min_x, max_x, min_y, max_y) =
            pixels.iter().fold((i64::MAX, i64::MIN, i64::MAX, i64::MIN), limits);

        Image { pixels, min_x, max_x, min_y, max_y, void }
    }

    fn get(&self, x: i64, y:i64) -> bool {
        if x < self.min_x || x > self.max_x || y < self.min_y || y > self.max_y {
            self.void
        } else {
            self.pixels.contains(&(x, y))
        }
    }
}

fn main() -> Result<()> {
    let (algorithm, image) = parse()?;

    let part1 = enhance(image.clone(), &algorithm, 2);

    println!("Part 1: {}", part1.pixels.iter().count());

    let part2 = enhance(image, &algorithm, 50);

    println!("Part 2: {}", part2.pixels.iter().count());

    Ok(())
}

fn enhance(mut image: Image, algorithm: &Vec<char>, steps: usize) -> Image {
    for _ in 0..steps {
        let mut next_pixels: HashSet<(i64, i64)> = HashSet::new();

        for y in image.min_y-1..=image.max_y+1 {
            for x in image.min_x-1..=image.max_x+1 {
                if algorithm[index(&image, x, y)] == '#' {
                    next_pixels.insert((x, y));
                }
            }
        }

        let next_void = match image.void {
            true => algorithm[0] == '.',  // background static
            false => algorithm[0] == '#', // background alternates
        };

        image = Image::new(next_pixels, next_void);
    }

    image
}

fn index(image: &Image, x: i64, y: i64) -> usize {
    let mut n = 0;

    for by in y-1..=y+1 {
        for bx in x-1..=x+1 {
            n <<= 1;
            n |= if image.get(bx, by) { 1 } else { 0 };
        }
    }
    n
}

fn parse() -> Result<(Vec<char>, Image)> {
    let mut algorithm: Vec<char> = vec![];
    let mut lines: Vec<String> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.trim().len() == 512 {
            algorithm = line.trim().chars().collect();
        } else if line.len() > 0 {
            lines.push(line);
        }
    }

    let mut pixels = Pixels::new();

    for (y, row) in lines.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                pixels.insert((x as i64, y as i64));
            }
        }
    }

    let image = Image::new(pixels, false);

    Ok((algorithm, image))
}
