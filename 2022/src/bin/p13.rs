use anyhow::Result;
use std::cmp::{min, Ordering};
use std::io::{self, Read};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Int(u64),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        match (self, other) {
            (Packet::Int(n), Packet::Int(o)) => n.cmp(o),

            (Packet::Int(n), Packet::List(_)) => Packet::List(vec![Packet::Int(*n)]).cmp(other),

            (Packet::List(_), Packet::Int(n)) => self.cmp(&Packet::List(vec![Packet::Int(*n)])),

            (Packet::List(left), Packet::List(right)) => {
                let ll = left.len();
                let lr = right.len();
                for i in 0..min(ll, lr) {
                    match left[i].cmp(&right[i]) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        _ => {}
                    }
                }

                ll.cmp(&lr)
            }
        }
    }
}

struct Parser {
    chars: Vec<char>,
    cur: usize,
}

impl Parser {
    fn new(chars: Vec<char>) -> Self {
        Self { chars, cur: 0 }
    }

    fn next(&mut self) -> char {
        let c = self.chars[self.cur];
        self.cur += 1;

        c
    }

    fn matches(&mut self, p: fn(char) -> bool) -> Option<char> {
        if p(self.chars[self.cur]) {
            return Some(self.next());
        }

        None
    }

    fn parse(&mut self) -> Packet {
        if self.matches(|c| c == '[').is_some() {
            // parse list
            let mut list = vec![];
            if self.matches(|c| c == ']').is_some() {
                return Packet::List(list);
            }
            loop {
                // parse elements
                let p = self.parse();
                list.push(p);

                if self.next() == ']' {
                    return Packet::List(list);
                }
            }
        } else {
            // parse int
            let mut num = self.next() as u64 - '0' as u64;

            while let Some(d) = self.matches(|c| c.is_digit(10)) {
                num = num * 10 + d as u64 - '0' as u64;
            }

            return Packet::Int(num);
        }
    }
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    // add part 2 packets in wrong order so ignored in part 1
    buffer += "\n[[6]]\n[[2]]\n";

    let mut packets = vec![];

    for line in buffer.lines().filter(|&s| s != "") {
        let mut parser = Parser::new(line.chars().collect());
        let r = parser.parse();
        packets.push(r);
    }

    let part1: usize = packets
        .chunks(2)
        .enumerate()
        .filter(|(_, p)| p[0] < p[1])
        .map(|(i, _)| i + 1)
        .sum();

    println!("Part 1: {}", part1);

    // last two are dividers
    let n = packets.len();
    let dividers: Vec<_> = packets.iter().skip(n - 2).cloned().collect();

    packets.sort_unstable();

    let part2: usize = packets
        .iter()
        .enumerate()
        .filter(|(_, p)| dividers.contains(p))
        .map(|(i, _)| i + 1)
        .product();

    println!("Part 2: {}", part2);

    Ok(())
}
