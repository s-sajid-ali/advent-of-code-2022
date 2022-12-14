use fs_err as fs;
use std::collections::VecDeque;
use std::error::Error;

mod crt;
use crate::crt::cpu::Instruction;
use crate::crt::cpu::State;
use crate::crt::render::Screen;

pub fn draw(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();

    let mut state = State::new(0, 1);
    let mut screen = Screen::new();

    for line in component_lines {
        if line.is_empty() {
            continue;
        } else {
            let input_vec: Vec<_> = line.split_whitespace().collect();

            let instr: Instruction = match input_vec[0] {
                "addx" => Some(Instruction::Addx {
                    v: input_vec[1]
                        .parse::<i32>()
                        .expect("invalid V value for addx"),
                }),
                "noop" => Some(Instruction::Noop),
                _ => None,
            }
            .expect("invalid instruction in input file");

            match instr {
                Instruction::Noop => {
                    state.cycle += 1;
                    screen.render(&state);
                }
                Instruction::Addx { v } => {
                    state.cycle += 1;
                    screen.render(&state);
                    state.cycle += 1;
                    screen.render(&state);
                    state.register += v;
                }
            }
        }
    }
    Ok(())
}

pub fn run(filename: String, mut query_cycles: VecDeque<u32>) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();

    let mut state = State::new(0, 1);

    let signal_strengths_sum: i64 = component_lines
        .into_iter()
        .map(|line| {
            if line.is_empty() {
                return 0;
            } else {
                let input_vec: Vec<_> = line.split_whitespace().collect();

                let instr: Instruction = match input_vec[0] {
                    "addx" => Some(Instruction::Addx {
                        v: input_vec[1]
                            .parse::<i32>()
                            .expect("invalid V value for addx"),
                    }),
                    "noop" => Some(Instruction::Noop),
                    _ => None,
                }
                .expect("invalid instruction in input file");

                match instr {
                    Instruction::Noop => {
                        state.cycle += 1;
                        if let Some(signal_strength) = state.query(&mut query_cycles) {
                            return signal_strength;
                        }
                        //println!("state is {}, end of noop", state);
                    }
                    Instruction::Addx { v } => {
                        let mut signal_strength = 0;
                        state.cycle += 1;
                        //println!("state is {}, middle of addx", state);
                        if let Some(signal_strength2) = state.query(&mut query_cycles) {
                            signal_strength += signal_strength2;
                        }
                        state.cycle += 1;
                        //println!("state is {}, middle of addx", state);
                        if let Some(signal_strength3) = state.query(&mut query_cycles) {
                            signal_strength += signal_strength3;
                        }
                        state.register += v;
                        return signal_strength;
                    }
                }
                0
            }
        })
        .sum();

    println!(
        "sum of requested signal strengths is {}",
        signal_strengths_sum
    );

    println!("final state is {}", state);

    Ok(())
}
