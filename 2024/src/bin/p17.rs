use std::io::{self, BufRead};

use anyhow::Result;

const OP_ADV: u8 = 0;
const OP_BXL: u8 = 1;
const OP_BST: u8 = 2;
const OP_JNZ: u8 = 3;
const OP_BXC: u8 = 4;
const OP_OUT: u8 = 5;
const OP_BDV: u8 = 6;
const OP_CDV: u8 = 7;

#[derive(Default, Debug, Clone)]
struct CPU {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
    output: Vec<u8>,
}

impl CPU {
    fn step(&mut self, mem: &[u8]) -> bool {
        if self.pc + 1 > mem.len() {
            return false;
        }
        let opcode = mem[self.pc];
        let operand = mem[self.pc + 1];

        match opcode {
            OP_ADV => self.a = self.a >> self.combo(operand),
            OP_BXL => self.b = self.b ^ operand as u64,
            OP_BST => self.b = self.combo(operand) % 8,
            OP_JNZ => {
                if self.a != 0 {
                    self.pc = operand as usize;
                    return true;
                }
            },
            OP_BXC => self.b = self.b ^ self.c,
            OP_OUT => self.output.push((self.combo(operand) % 8) as u8),
            OP_BDV => self.b = self.a >> self.combo(operand),
            OP_CDV => self.c = self.a >> self.combo(operand),
            _ => unreachable!("invalid op code {}", opcode),
        }

        self.pc += 2;

        true
    }

    fn combo(&self, v: u8) -> u64 {
        match v {
            0..=3 => v as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<()> {
    let (cpu, mem) = parse()?;

    println!("Part 1: {}", part1(&mut cpu.clone(), &mem));

    println!("Part 2: {}", part2(&cpu, &mem));

    Ok(())
}

fn part1(cpu: &mut CPU, mem: &[u8]) -> String {
    while cpu.step(&mem) {}

    cpu.output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>().join(",")
}

// Program
// 2,4  bst   b = a % 8
// 1,1  bxl   b = b ^ 1
// 7,5  cdv   c = a >> b
// 0,3  adv   a = a >> 3
// 1,4  bxl   b = b ^ 4
// 4,4  bxc   b = b ^ c
// 5,5  out   out b
// 3,0  jnz   jump a != 0 to 0

// solve from end to front 3 bits at a time.
// not sure this works for all input
fn part2(cpu: &CPU, mem: &[u8]) -> u64 {
    let mut a = 0;

    for idx in (0..mem.len()).rev() {
        let tail = &mem[idx..];

        for next_a in (a << 3).. {
            let mut cpu = cpu.clone();
            cpu.a = next_a;
            while cpu.step(mem) {}

            if cpu.output == tail {
                a = next_a;
                break;
            }
        }
    }

    a
}

fn parse() -> Result<(CPU, Vec<u8>)> {
    let mut cpu = CPU::default();
    let mut mem = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;

        if line.contains('A') {
            cpu.a = line.split(' ').nth(2).unwrap().parse()?;
        } else if line.contains('B') {
           cpu.b = line.split(' ').nth(2).unwrap().parse()?;
        } else if line.contains('B') {
           cpu.c = line.split(' ').nth(2).unwrap().parse()?;
        } else if line.starts_with("Program") {
            let tokens: Vec<_> = line.split(&[' ', ',']).collect();
            mem = tokens[1..].iter().map(|&s| s.parse()).collect::<Result<_, _>>()?;
        }
    }

    Ok((cpu, mem))
}
