use std::io::{self, Read};

use anyhow::Result;

fn main() -> Result<()> {
    let map = parse()?;

    println!("Part 1: {}", part1(&map));

    println!("Part 2: {}", part2(&map));

    Ok(())
}

fn part2(map: &[u8]) -> u64 {
    let mut files = vec![];
    let mut spaces = vec![];

    let mut pos: usize = 0;

    for (i, &len) in map.iter().enumerate() {
        if i % 2 == 0 {
            files.push((pos, len));
        } else {
            spaces.push((pos, len));
        }
        pos += len as usize;
    }

    for file_index in (0..files.len()).rev() {
        let (file_pos, file_size) = files[file_index];

        for i in 0..spaces.len() {
            let (space_pos, space_size) = spaces[i];

            if space_pos >= file_pos {
                break; // space is to the right of current file, bail
            }

            if file_size <= space_size {
                files[file_index] = (space_pos, file_size);

                if file_size == space_size {
                    spaces.remove(i);
                } else {
                    spaces[i] = (space_pos + file_size as usize, space_size - file_size);
                }
                break;
            }
        }
    }

    files.iter().enumerate().map(|(i, &(pos, len))| {
        let a1  = pos as u64; //file.0 as u64;
        let n = len as u64;

        i as u64 * ((n * (2 * a1 + (n - 1))) / 2)
    }).sum()
}

fn part1(map: &[u8]) -> i64 {
    const FREE: i32 = -1;

    let mut expanded_map: Vec<i32> = vec![];

    // expand map
    for (i, &len) in map.iter().enumerate() {
        let file_id = if i % 2 == 0 {
            (i / 2) as i32
        } else {
            FREE
        };

        for _ in 0..len {
            expanded_map.push(file_id as i32);
        }
    }

    // get indices of empty space
    let empties: Vec<_> = expanded_map
        .iter()
        .enumerate()
        .filter(|a| a.1 == &FREE)
        .map(|a| a.0)
        .collect();

    for empty in &empties {
        // remove trailing free space
        while let Some(&FREE) = expanded_map.last() {
            expanded_map.pop();
        }

        // is empty still valid
        if *empty >= expanded_map.len() {
            break;
        }

        // put last element in free space
        if let Some(end) = expanded_map.pop() {
            expanded_map[*empty] = end;
        }
    }

    expanded_map.iter().enumerate().map(|a| a.0 as i64 * *a.1 as i64).sum()
}

fn parse() -> Result<Vec<u8>> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut map = vec![];

    for c in buffer.chars().filter(|x| x.is_digit(10)) {
        map.push(c.to_digit(10).unwrap() as u8);
    }

    Ok(map)
}
