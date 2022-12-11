use anyhow::Result;
use std::io::{self, Read};

struct Monkey {
    items: Vec<i64>,
    op: Box<dyn Fn(i64) -> i64>,
    test_div: i64,
    test_true: usize,
    test_false: usize,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            items: vec![],
            op: Box::new(|x| x),
            test_div: 0,
            test_true: 0,
            test_false: 0,
        }
    }
}

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer)?;

    let mut monkeys1 = vec![];
    let mut monkeys2 = vec![];

    for monkey in buffer.split("\n\n") {
        monkeys1.push(parse_monkey(monkey)?);
        monkeys2.push(parse_monkey(monkey)?);
    }

    let part1 = process(&mut monkeys1, false);

    println!("Part 1: {}", part1);

    let part2 = process(&mut monkeys2, true);

    println!("Part 2: {}", part2);

    Ok(())
}

fn process(monkeys: &mut [Monkey], part2: bool) -> i64 {
    let worry_mod = monkeys.iter().map(|m| m.test_div).product::<i64>();

    let mut inspected_counts = vec![0; monkeys.len()];

    let rounds = if part2 { 10000 } else { 20 };

    for _ in 0..rounds {
        for idx in 0..monkeys.len() {
            // inspect each item
            while !monkeys[idx].items.is_empty() {
                inspected_counts[idx] += 1;
                // get item
                let mut wl = monkeys[idx].items.remove(0);

                // inspect
                wl = (monkeys[idx].op)(wl);

                // adjust worry
                wl = if part2 { wl % worry_mod } else { wl / 3 };

                // throw
                if wl % monkeys[idx].test_div == 0 {
                    let i = monkeys[idx].test_true;
                    monkeys[i].items.push(wl);
                } else {
                    let i = monkeys[idx].test_false;
                    monkeys[i].items.push(wl);
                }
            }
        }
    }

    inspected_counts.sort_unstable_by(|a, b| b.cmp(a));

    inspected_counts.iter().take(2).product::<i64>()
}

fn parse_monkey(monkey: &str) -> Result<Monkey> {
    let mut m = Monkey::new();

    let lines: Vec<_> = monkey.lines().collect();

    if let Some(items) = lines[1].strip_prefix("  Starting items: ") {
        for n in items.split(", ") {
            if let Ok(x) = n.parse() {
                m.items.push(x);
            }
        }
    }

    if let Some(operation) = lines[2].strip_prefix("  Operation: new = ") {
        let tokens: Vec<_> = operation.split(" ").collect();

        m.op = match tokens[1..] {
            ["*", "old"] => Box::new(|old| old * old),
            ["+", "old"] => Box::new(|old| old + old),
            ["*", _] => {
                let n = tokens[2].parse::<i64>()?;
                Box::new(move |old| old * n)
            }
            ["+", _] => {
                let n = tokens[2].parse::<i64>()?;
                Box::new(move |old| old + n)
            }
            _ => unreachable!(),
        };
    }

    if let Some(test) = lines[3].strip_prefix("  Test: divisible by ") {
        m.test_div = test.parse()?;
    }

    if let Some(test_true) = lines[4].strip_prefix("    If true: throw to monkey ") {
        m.test_true = test_true.parse()?;
    }

    if let Some(test_false) = lines[5].strip_prefix("    If false: throw to monkey ") {
        m.test_false = test_false.parse()?;
    }

    Ok(m)
}
