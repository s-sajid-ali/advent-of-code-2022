use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Provide only one input, the filename move instructions for head!")
    }

    let filename_in: String = args[1].clone();
    let rope_length: usize = 10;

    if let Err(e) = day9::run(filename_in, rope_length) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
