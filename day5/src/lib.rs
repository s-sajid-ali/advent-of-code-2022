use fs_err as fs;
use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;

pub fn move_stacks_part_2(
    mut container_ship: HashMap<u32, Vec<char>>,
    filename: String,
) -> Result<(), Box<dyn Error>> {
    //dbg!(container_ship);
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();
    for line in component_lines {
        if line.is_empty() {
            // account for any extra newline at end
            break;
        } else {
            let input_vec: Vec<_> = line.split_whitespace().collect();
            let move_quantity: u32 = input_vec[1]
                .parse::<u32>()
                .expect("Erronous input instruction!");
            let src_stack_idx: u32 = input_vec[3]
                .parse::<u32>()
                .expect("Erronous input instruction!");
            let dst_stack_idx: u32 = input_vec[5]
                .parse::<u32>()
                .expect("Erronous input instruction!");

            let src_stack = container_ship
                .get_mut(&(src_stack_idx as u32))
                .expect("no input stack at source!");
            let mut moved_items: Vec<_> = src_stack
                .drain(src_stack.len() - move_quantity as usize..)
                .collect();
            let dst_stack = container_ship
                .get_mut(&(dst_stack_idx as u32))
                .expect("no input stack at destination!");
            dst_stack.append(&mut moved_items);
        }
    }

    for key in container_ship.keys().sorted() {
        let stack = container_ship.get(key).expect("no stack at this key!");
        println!("{}", stack.last().expect("empty stack!"));
    }

    Ok(())
}

pub fn move_stacks_part_1(
    mut container_ship: HashMap<u32, Vec<char>>,
    filename: String,
) -> Result<(), Box<dyn Error>> {
    //dbg!(container_ship);
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();
    for line in component_lines {
        if line.is_empty() {
            // account for any extra newline at end
            break;
        } else {
            let input_vec: Vec<_> = line.split_whitespace().collect();
            let move_quantity: u32 = input_vec[1]
                .parse::<u32>()
                .expect("Erronous input instruction!");
            let src_stack_idx: u32 = input_vec[3]
                .parse::<u32>()
                .expect("Erronous input instruction!");
            let dst_stack_idx: u32 = input_vec[5]
                .parse::<u32>()
                .expect("Erronous input instruction!");

            for _ in 0..move_quantity {
                let src_stack = container_ship
                    .get_mut(&(src_stack_idx as u32))
                    .expect("no input stack at source!");
                let elem = src_stack
                    .last()
                    .expect("erronous instruction, source stack has too few elements!")
                    .clone();
                src_stack.pop();
                let dst_stack = container_ship
                    .get_mut(&(dst_stack_idx as u32))
                    .expect("no input stack at destination!");
                dst_stack.push(elem);
                //println!("moved elem : {}", elem);
            }
        }
    }

    for key in container_ship.keys().sorted() {
        let stack = container_ship.get(key).expect("no stack at this key!");
        println!("{}", stack.last().expect("empty stack!"));
    }

    Ok(())
}

pub fn starting_configuration(filename: String) -> Result<HashMap<u32, Vec<char>>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();

    let mut indices: Vec<u32> = Vec::new();
    let mut container_ship: HashMap<u32, Vec<char>> = HashMap::new();

    let mut counted_number_stacks: bool = false;
    for line in component_lines.iter().rev() {
        if line.is_empty() {
            // account for any extra newline at end
            continue;
        } else {
            let input: Vec<_> = line.split_whitespace().collect::<Vec<_>>();
            // count the number of stacks first
            if counted_number_stacks == false {
                for elem in input {
                    let idx: u32 = elem.parse::<u32>().expect("expected indices on last line!");
                    indices.push(idx);
                }
                counted_number_stacks = true;
                for elem in &indices {
                    container_ship.insert(*elem, Vec::new());
                    //println!("indices elem is {}", elem);
                }
            } else {
                let iter = (line.chars().count() + 1) / 4;

                for (elem, i) in line.chars().skip(1).step_by(4).zip(1..=iter) {
                    if !(elem == ' ') {
                        if let Some(stack) = container_ship.get_mut(&(i as u32)) {
                            (*stack).push(elem);
                        }
                    }
                }
            }
        }
    }

    Ok(container_ship)
}
