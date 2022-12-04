use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Provide only one input, the filename of values!")
    }

    let filename_in: String = args[1].clone();

    if let Err(e) = day3::run_part2(filename_in) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
