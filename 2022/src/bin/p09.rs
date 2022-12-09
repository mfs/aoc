use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, BufRead};

type Pt = (i64, i64);

#[derive(Debug, Default)]
struct State {
    map: HashSet<Pt>,
    head: Pt,
    tail: Vec<Pt>,
}

fn main() -> Result<()> {
    let mut moves = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        let tokens: Vec<_> = line.split(" ").collect();
        let n = tokens[1].parse::<i64>()?;
        match tokens[0] {
            "L" => moves.push(('L', n)),
            "R" => moves.push(('R', n)),
            "U" => moves.push(('U', n)),
            "D" => moves.push(('D', n)),
            _ => unreachable!(),
        }
    }

    println!("Part 1: {}", process(&moves, 1));

    println!("Part 2: {}", process(&moves, 9));

    Ok(())
}

fn process(moves: &[(char, i64)], tail_n: usize) -> usize {
    let mut state = State::default();

    for _ in 0..tail_n {
        state.tail.push((0, 0));
    }

    for m in moves {
        move_head(*m, &mut state);
    }

    state.map.len()
}

fn move_head(m: (char, i64), state: &mut State) {
    for _ in 0..m.1 {
        match m.0 {
            'L' => state.head.0 -= 1,
            'R' => state.head.0 += 1,
            'U' => state.head.1 -= 1,
            'D' => state.head.1 += 1,
            _ => unreachable!(),
        }

        // update tail
        let mut h = state.head;

        for idx in 0..state.tail.len() {
            move_tail(h, &mut state.tail[idx]);
            h = state.tail[idx];
        }

        let last = state.tail.len() - 1;
        state.map.insert(state.tail[last]);
    }
}

fn move_tail(head: Pt, tail: &mut Pt) {
    let delta = (head.0 - tail.0, head.1 - tail.1);

    if delta.0.abs() <= 1 && delta.1.abs() <= 1 {
        // no change
        return;
    }

    if delta.0 == 0 && (delta.1 == 2 || delta.1 == -2) {
        tail.1 += delta.1 / 2;
    } else if delta.1 == 0 && (delta.0 == 2 || delta.0 == -2) {
        tail.0 += delta.0 / 2;
    } else {
        // diagonal move
        tail.0 += delta.0 / delta.0.abs();
        tail.1 += delta.1 / delta.1.abs();
    }
}
