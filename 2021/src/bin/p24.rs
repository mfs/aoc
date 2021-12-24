use std::io::{self, BufRead};
use std::collections::HashMap;
use anyhow::{anyhow, Result};

const Z: usize = 3;

type Cache = HashMap<(i64, usize), Option<i64>>;

#[derive(Debug, Clone, Copy)]
enum Operand {
    Num(i64),
    Var(usize),
}

impl Operand {
    fn parse(operand: &str) -> Result<Self> {
        match operand {
            "w" => Ok(Operand::Var(0)),
            "x" => Ok(Operand::Var(1)),
            "y" => Ok(Operand::Var(2)),
            "z" => Ok(Operand::Var(3)),
            _ => Ok(Operand::Num(operand.parse()?)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

impl Instruction {
    fn parse(elements: &Vec<&str>) -> Result<Self> {
        match elements[0] {
            "inp" => Ok(Instruction::Inp(Operand::parse(elements[1])?)),
            "add" => Ok(Instruction::Add(Operand::parse(elements[1])?, Operand::parse(elements[2])?)),
            "mul" => Ok(Instruction::Mul(Operand::parse(elements[1])?, Operand::parse(elements[2])?)),
            "div" => Ok(Instruction::Div(Operand::parse(elements[1])?, Operand::parse(elements[2])?)),
            "mod" => Ok(Instruction::Mod(Operand::parse(elements[1])?, Operand::parse(elements[2])?)),
            "eql" => Ok(Instruction::Eql(Operand::parse(elements[1])?, Operand::parse(elements[2])?)),
            _ => Err(anyhow!("invalid operation")),
        }
    }
}

#[derive(Debug, Clone)]
struct ALU<'a> {
    variables: [i64; 4],
    instructions: &'a [Instruction],
    input: i64,
}

impl <'a> ALU<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        ALU { variables: [0; 4], instructions, input: 0 }
    }

    fn run(&mut self) {
        for i in self.instructions {
            match i {
                Instruction::Inp(Operand::Var(a)) => {
                    self.variables[*a] = self.input;
                },
                Instruction::Add(Operand::Var(a), Operand::Var(b)) => {
                    self.variables[*a] = self.variables[*a] + self.variables[*b];
                },
                Instruction::Add(Operand::Var(a), Operand::Num(b)) => {
                    self.variables[*a] = self.variables[*a] + *b;
                },
                Instruction::Mul(Operand::Var(a), Operand::Var(b)) => {
                    self.variables[*a] = self.variables[*a] * self.variables[*b];
                },
                Instruction::Mul(Operand::Var(a), Operand::Num(b)) => {
                    self.variables[*a] = self.variables[*a] * *b;
                },
                Instruction::Div(Operand::Var(a), Operand::Var(b)) => {
                    self.variables[*a] = self.variables[*a] / self.variables[*b];
                },
                Instruction::Div(Operand::Var(a), Operand::Num(b)) => {
                    self.variables[*a] = self.variables[*a] / *b;
                },
                Instruction::Mod(Operand::Var(a), Operand::Var(b)) => {
                    self.variables[*a] = self.variables[*a] % self.variables[*b];
                },
                Instruction::Mod(Operand::Var(a), Operand::Num(b)) => {
                    self.variables[*a] = self.variables[*a] % *b;
                },
                Instruction::Eql(Operand::Var(a), Operand::Var(b)) => {
                    if self.variables[*a] == self.variables[*b] {
                        self.variables[*a] = 1;
                    } else {
                        self.variables[*a] = 0;
                    }
                },
                Instruction::Eql(Operand::Var(a), Operand::Num(b)) => {
                    if self.variables[*a] == *b {
                        self.variables[*a] = 1;
                    } else {
                        self.variables[*a] = 0;
                    }
                },
                _ => unreachable!(),
            }
        }
    }
}

fn run_block(cache: &mut Cache, blocks: &[&[Instruction]], block: usize, z: i64, digits: &[i64]) -> Option<i64> {
    if let Some(&a) = cache.get(&(z, block)) {
       return a;
    }

    for &digit in digits {
        let mut alu = ALU::new(blocks[block]);
        alu.input = digit;
        alu.variables[Z] = z;
        alu.run();
        let z = alu.variables[Z];

        // terminal state
        if z == 0 && block == blocks.len() - 1 {
            cache.insert((z, block), Some(digit));
            return Some(digit);
        } else if block == blocks.len() - 1 {
            continue;
        }

        // recurse
        if let Some(x) = run_block(cache, blocks, block + 1, z, digits) {
            cache.insert((z, block), Some(x * 10 + digit));
            return Some(x * 10 + digit);
        }
    }

    cache.insert((z, block), None);
    None
}

fn main() -> Result<()> {
    let mut instructions = vec![];

    for line in io::stdin().lock().lines() {
        let line = line?;
        let elements: Vec<&str> = line.split(' ').collect();

        instructions.push(Instruction::parse(&elements)?);
    }

    let mut cache: HashMap<(i64, usize), Option<i64>> = HashMap::new();

    let blocks: Vec<_> = instructions.chunks(18).collect();

    let mut digits: Vec<_> = (1..=9).rev().collect();

    let part1 = run_block(&mut cache, &blocks, 0, 0, &digits).ok_or(anyhow!("solution not found"))?;

    println!("Part 1: {}", reverse(part1));

    digits.reverse();

    let part2 = run_block(&mut cache, &blocks, 0, 0, &digits).ok_or(anyhow!("solution not found"))?;

    println!("Part 2: {}", reverse(part2));

    Ok(())
}

fn reverse(mut n: i64) -> i64 {
    let mut r = 0;
    while n > 0 {
        r = r * 10 + n % 10;
        n /= 10;
    }

    r
}
