use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Provide only one input, the filename of values!")
    }

    let filename_in: String = args[1].clone();

    let source_loc = (500, 0);

    if let Err(e) = day14::run2(filename_in, source_loc) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
