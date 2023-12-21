use anyhow::Result;
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
enum State {
    Off,
    On,
}

impl State {
    fn invert(&self) -> Self {
        match self {
            State::Off => State::On,
            State::On => State::Off,
        }
    }
}

#[derive(Debug, Clone)]
enum Module {
    Broadcaster,                         // outputs
    FlipFlop(State),                     // state, outputs
    Conjunction(HashMap<String, Pulse>), // last pulses, outputs
}

type IOMap = HashMap<String, Vec<String>>;

fn main() -> Result<()> {
    let (mut modules, inputs, outputs) = parse()?;

    let (mut a, mut b) = (0, 0);

    let mut cycles = HashMap::new();

    for i in 1..5000 {
        let p = push_button(&mut modules, &inputs, &outputs);
        a += p.0;
        b += p.1;
        if i == 1000 {
            println!("Part 1: {}", a * b);
        }

        // cycles
        if p.2 != "" {
            cycles.insert(p.2, i as u64);
        }

        // hardcoded for 4 inputs to gate before rx
        if cycles.len() == 4 {
            println!(
                "Part 2: {}",
                cycles.into_values().reduce(|acc, x| lcm(acc, x)).unwrap()
            );
            break;
        }
    }

    Ok(())
}

fn push_button(
    modules: &mut HashMap<String, Module>,
    inputs: &IOMap,
    outputs: &IOMap,
) -> (u32, u32, String) {
    let mut pulses = VecDeque::new();
    let mut low_gate = "".to_string();

    pulses.push_back(("button".to_owned(), Pulse::Low, "broadcaster".to_string()));

    let mut low_count = 0;
    let mut high_count = 0;

    while let Some((src, pulse, dest)) = pulses.pop_front() {
        match pulse {
            Pulse::Low => low_count += 1,
            Pulse::High => high_count += 1,
        }

        // look for cycles, rm is gate before rx
        if dest == "rm" && pulse == Pulse::High {
            low_gate = src.clone();
        }

        if let Some(module) = modules.get_mut(&dest) {
            match module {
                Module::Broadcaster => {
                    for p in &outputs["broadcaster"] {
                        pulses.push_back(("broadcaster".to_owned(), Pulse::Low, p.to_owned()));
                    }
                }
                Module::FlipFlop(state) => {
                    if pulse == Pulse::Low {
                        *state = state.invert();
                        for nd in &outputs[&dest] {
                            match state {
                                State::Off => {
                                    pulses.push_back((dest.clone(), Pulse::Low, nd.to_owned()))
                                }
                                State::On => {
                                    pulses.push_back((dest.clone(), Pulse::High, nd.to_owned()))
                                }
                            }
                        }
                    }
                }
                Module::Conjunction(last) => {
                    last.insert(src, pulse);

                    // count input history
                    let inputs_x = &inputs[&dest];
                    let mut input_pulses: Vec<_> = vec![];
                    for i in inputs_x {
                        input_pulses.push(*last.entry(i.to_string()).or_insert(Pulse::Low));
                    }

                    // if all high send low
                    if input_pulses.iter().all(|&x| x == Pulse::High) {
                        // send low
                        for nd in &outputs[&dest] {
                            pulses.push_back((dest.clone(), Pulse::Low, nd.to_owned()));
                        }
                    } else {
                        // send high
                        for nd in &outputs[&dest] {
                            pulses.push_back((dest.clone(), Pulse::High, nd.to_owned()));
                        }
                    }
                }
            }
        }
    }

    (low_count, high_count, low_gate)
}

fn parse() -> Result<(HashMap<String, Module>, IOMap, IOMap)> {
    let mut modules = HashMap::new();
    let mut outputs: IOMap = HashMap::new();
    let mut inputs: IOMap = HashMap::new();

    for line in io::stdin().lock().lines() {
        let line = line?;

        let tokens: Vec<_> = line.split(" -> ").collect();

        let dest: Vec<String> = tokens[1].split(", ").map(String::from).collect();

        if tokens[0] == "broadcaster" {
            outputs.insert(tokens[0].to_owned(), dest.clone());
            modules.insert(tokens[0].to_owned(), Module::Broadcaster);
        } else if &tokens[0][0..1] == "%" {
            outputs.insert(tokens[0][1..].to_owned(), dest.clone());
            for d in &dest {
                inputs
                    .entry(d.to_owned())
                    .or_insert(vec![])
                    .push(tokens[0][1..].to_owned());
            }
            modules.insert(tokens[0][1..].to_owned(), Module::FlipFlop(State::Off));
        } else if &tokens[0][0..1] == "&" {
            outputs.insert(tokens[0][1..].to_owned(), dest.clone());
            for d in &dest {
                inputs
                    .entry(d.to_owned())
                    .or_insert(vec![])
                    .push(tokens[0][1..].to_owned());
            }
            modules.insert(
                tokens[0][1..].to_owned(),
                Module::Conjunction(HashMap::new()),
            );
        }
    }

    Ok((modules, inputs, outputs))
}
