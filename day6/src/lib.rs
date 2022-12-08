use fs_err as fs;
use itertools::Itertools;
use std::collections::VecDeque;
use std::error::Error;

fn check_marker(length: usize, buffer: &VecDeque<char>) -> bool {
    // check that we are given the number of chars == marker size
    assert_eq!(buffer.len(), length);

    let cmp_vec: Vec<u32> = (0..length)
        .cartesian_product(0..length)
        .map(|(i, j)| {
            if i != j && buffer[i] == buffer[j] {
                1
            } else {
                0
            }
        })
        .collect();

    if cmp_vec.iter().sum::<u32>() == 0 {
        return true;
    }

    false
}

pub fn run(length: usize, filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines();

    for line in component_lines {
        if line.is_empty() {
            break;
        } else {
            let mut buf: VecDeque<char> = VecDeque::with_capacity(length);
            let mut input = line.chars().enumerate();
            // fill the first 4 input characters
            for _ in 0..length {
                let (_, elem) = input
                    .next()
                    .expect("need at least as many characters as marker length!");
                buf.push_back(elem);
            }
            // check if first few chars are same
            if check_marker(length, &buf) == true {
                println!("start of packet marker is at {}", length + 1);
            }

            loop {
                match input.next() {
                    Some((idx, elem)) => {
                        buf.pop_front();
                        buf.push_back(elem);
                        if check_marker(length, &buf) == true {
                            println!("start of packet marker is at {}", idx + 1);
                            break;
                        }
                        continue;
                    }
                    None => {
                        println!("No start of packet marker found!");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
