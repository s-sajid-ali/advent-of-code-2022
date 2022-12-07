use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Provide only two input, the filenames of the starting stack configuration and move instructions!")
    }

    let filename_config: String = args[1].clone();
    let filename_moves: String = args[2].clone();

    match day5::starting_configuration(filename_config) {
        Ok(stacks_on_ship) => {
            if let Err(e) = day5::move_stacks_part_2(stacks_on_ship, filename_moves) {
                eprintln!(
                    "Application error while parsing initial configuration: {}",
                    e
                );
                process::exit(1);
            }
        }

        Err(e) => {
            eprintln!(
                "Application error while parsing initial configuration: {}",
                e
            );
            process::exit(1);
        }
    }
}
