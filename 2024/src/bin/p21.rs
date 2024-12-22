use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self, BufRead};

use anyhow::Result;
use phf::phf_map;

type V = (i32, i32);
type KeypadMap = phf::Map<char, V>;
type Cache = HashMap<(String, i32), u64>; // (code, level) -> minimal_length

static DIRECTIONAL: phf::Map<char, V> = phf_map! {
    '_' => (0, 0), '^' => (1, 0), 'A' => (2, 0),
    '<' => (0, 1), 'v' => (1, 1), '>' => (2, 1),
};

static NUMERIC: phf::Map<char, V> = phf_map! {
    '7' => (0, 0), '8' => (1, 0), '9' => (2, 0),
    '4' => (0, 1), '5' => (1, 1), '6' => (2, 1),
    '1' => (0, 2), '2' => (1, 2), '3' => (2, 2),
    '_' => (0, 3), '0' => (1, 3), 'A' => (2, 3),
};

fn main() -> Result<()> {
    let codes = parse()?;

    for (part, &i) in [3, 26].iter().enumerate() {
        let ans = codes
            .iter()
            .map(|(code, n)| {
                minimal_length(&NUMERIC, &code, i, &mut Cache::new()) * n
            })
            .sum::<u64>();

        println!("Part {}: {}", part + 1, ans);
    }

    Ok(())
}

fn minimal_length(keypad: &KeypadMap, code: &str, level: i32, cache: &mut Cache) -> u64 {
    if level == 0 {
        return code.len() as u64;
    }

    if let Some(&x) = cache.get(&(code.to_owned(), level)) {
        return x;
    }

    // always start on 'A'
    let mut pos = keypad[&'A'];

    let mut min_length = 0;

    for c in code.chars() {
        let keys = generate_all_keys(keypad, pos, keypad[&c]);

        let code_min_length = keys
            .iter()
            .map(|s| minimal_length(&DIRECTIONAL, &format!("{}A", s), level - 1, cache))
            .min().unwrap();

        min_length += code_min_length;
        pos = keypad[&c];
    }

    cache.insert((code.to_owned(), level), min_length);
    return min_length;
}

// modified bfs - all key combos that take us to the target key
fn generate_all_keys(keypad: &KeypadMap, start: V, end: V) -> Vec<String> {
    let mut all_keys = vec![];

    let mut queue = VecDeque::new();
    queue.push_back((start, "".to_string()));

    while let Some((pos, keys)) = queue.pop_front() {
        if pos == end {
            all_keys.push(keys);
            continue;
        }

        let md = |a: V, b: V| { (a.0 - b.0).abs() + (a.1 - b.1).abs() };

        for (dir, c) in [((1, 0), '>'), ((-1, 0), '<'), ((0, 1), 'v'), ((0, -1), '^')] {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);

            // tests if we are still on keypad, moving towards target key & not over empty key
            if md(new_pos, end) < md(pos, end) && keypad[&'_'] != new_pos  {
                queue.push_back((new_pos, format!("{}{}", keys, c)));
            }
        }
    }

    all_keys
}

fn parse() -> Result<Vec<(String, u64)>> {
    let mut codes = vec![];

    for line in io::stdin().lock().lines() {
        let s: String = line?.chars().collect();
        let n = s[..3].parse()?; // assumes 4 char codes ending with 'A'
        codes.push((s, n));
    }

    Ok(codes)
}
