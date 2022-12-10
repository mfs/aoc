use anyhow::Result;
use std::io::{self, BufRead};

const CRT_WIDTH: usize = 40;
const CRT_HEIGHT: usize = 6;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

struct CPU<'a> {
    reg_x: i64,
    cycle: usize,
    instructions: &'a [Instruction],
    signals: Vec<i64>,
    crt: Vec<Vec<char>>,
}

impl<'a> CPU<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            reg_x: 1,
            cycle: 0,
            instructions,
            signals: vec![],
            crt: vec![vec![' '; CRT_WIDTH]; CRT_HEIGHT],
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;

        if self.cycle == 20 || (self.cycle + 20) % 40 == 0 {
            self.signals.push(self.cycle as i64 * self.reg_x);
        }

        let x = (self.cycle - 1) % CRT_WIDTH;
        let y = (self.cycle - 1) / CRT_WIDTH;

        if x as i64 >= self.reg_x - 1 && x as i64 <= self.reg_x + 1 {
            self.crt[y][x] = '#';
        }
    }

    fn run(&mut self) {
        for ins in self.instructions {
            match ins {
                Instruction::Noop => self.tick(),
                Instruction::Addx(n) => {
                    self.tick();
                    self.tick();
                    self.reg_x += n;
                }
            }
        }
    }
}

fn parse() -> Result<Vec<Instruction>> {
    let mut instructions = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;
        let tokens: Vec<_> = line.split(" ").collect();

        let ins = match tokens[0] {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(tokens[1].parse()?),
            _ => unreachable!(),
        };

        instructions.push(ins);
    }

    Ok(instructions)
}

fn main() -> Result<()> {
    let instructions = parse()?;

    let mut cpu = CPU::new(&instructions);
    cpu.run();

    println!("Part 1: {}", cpu.signals.iter().sum::<i64>());

    println!("Part 2:");
    for row in cpu.crt {
        println!("{}", row.iter().cloned().collect::<String>());
    }

    Ok(())
}
