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
    #[arg(short, long)]
    part1: bool,

    /// yloc for part 1
    #[arg(short, long)]
    yloc: Option<i64>,

    /// flag for part 2
    #[arg(short, long)]
    part2: bool,

    /// ymin for part 2
    #[arg(short, long)]
    ymin: Option<i64>,

    /// ymax for part 2
    #[arg(short, long)]
    ymax: Option<i64>,
}

fn main() {
    let args = Args::parse();

    match (args.part1, args.part2) {
        (true, false) => match args.yloc {
            None => {
                eprintln!("Please provide yloc for part1!");
                process::exit(1);
            }
            Some(ylocval) => {
                if let Err(e) = day15::run1(args.filename, ylocval) {
                    eprintln!("Application error: {}", e);
                    process::exit(1);
                }
            }
        },
        (false, true) => match (args.ymin, args.ymax) {
            (Some(yminval), Some(ymaxval)) => {
                if let Err(e) = day15::run2(args.filename, yminval, ymaxval) {
                    eprintln!("Application error: {}", e);
                    process::exit(1);
                }
            }
            (_, _) => {
                eprintln!("Please provide ymin/ymax for part1!");
                process::exit(1);
            }
        },

        (_, _) => {
            eprintln!("Please choose either part1 or part2!");
            process::exit(1);
        }
    }
}
