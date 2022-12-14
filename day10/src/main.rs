#[allow(unused)]
use std::collections::VecDeque;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Provide only one input, the filename move instructions for head!")
    }

    let filename_in: String = args[1].clone();
    // used only for part1
    let _query_cycles: VecDeque<u32> = [20, 60, 100, 140, 180, 220].into();

    if let Err(e) = day10::draw(filename_in) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
