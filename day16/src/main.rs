use clap::Parser;
use std::process;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// filename
    #[arg(short, long)]
    filename: String,

    /// flag for part 1
    #[arg(long)]
    part1: bool,

    /// flag for part 2
    #[arg(long)]
    part2: bool,
}

fn main() {
    let args = Args::parse();

    match (args.part1, args.part2) {
        (true, false) => {
            if let Err(e) = day16::run1(args.filename) {
                eprintln!("Application error: {}", e);
                process::exit(1);
            }
        }
        (_, _) => {
            eprintln!("Please choose either part1 or part2!");
            process::exit(1);
        }
    }
}
