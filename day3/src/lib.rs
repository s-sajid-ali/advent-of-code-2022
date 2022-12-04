use fs_err as fs;
use itertools::Itertools;
use std::error::Error;

pub fn run_part2(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();
    let component_groups = component_lines.chunks_exact(3);

    let mut results: Vec<u32> = Vec::new();

    for group in component_groups {
        if group.len() != 3 {
            break;
        }

        let elf0: Vec<_> = group[0].chars().collect();

        let common: Vec<_> = elf0
            .iter()
            .map(|x| {
                let mut matchval: u32 = 0;
                for (y1, y2) in group[1].chars().cartesian_product(group[2].chars()) {
                    if (*x == y1) && (y1 == y2) {
                        matchval = x.to_digit(36).unwrap() - 9;
                        if x.is_ascii_uppercase() {
                            matchval = matchval + 26
                        }
                    }
                }
                matchval
            })
            .filter(|x| *x > 0)
            .collect::<Vec<_>>();

        results.push(common[0]);
    }

    println!("result sum is {}", results.iter().sum::<u32>());

    Ok(())
}
#[allow(unused)]
pub fn run_part1(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();
    let mut results: Vec<u32> = Vec::new();

    for item in component_lines {
        if item.is_empty() {
            break;
        }
        let mut chars: Vec<char> = item.chars().collect();
        let length = chars.len() / 2;
        let (left, right) = chars.split_at_mut(length);

        let common: Vec<_> = left
            .iter()
            .map(|x| {
                let mut matchval: u32 = 0;
                for y in right.iter() {
                    if x == y {
                        matchval = x.to_digit(36).unwrap() - 9;
                        if x.is_ascii_uppercase() {
                            matchval = matchval + 26
                        }
                    }
                }
                matchval
            })
            .filter(|x| *x > 0)
            .collect::<Vec<_>>();

        results.push(common[0]);
    }

    println!("result sum is {}", results.iter().sum::<u32>());

    Ok(())
}
