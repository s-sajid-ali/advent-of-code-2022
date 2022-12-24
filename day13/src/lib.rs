use fs_err as fs;
use std::error::Error;

mod packet;
use crate::packet::parse::{get_outer_elements, trim_packet};

#[derive(Copy, Clone, Debug)]
pub enum Correct {
    RightSideRanOutItems,
    LeftSideIsSmaller,
}

#[derive(Copy, Clone, Debug)]
pub enum Incorrect {
    LeftSideRanOutItems,
    RightSideIsSmaller,
}

#[derive(Copy, Clone, Debug)]
pub enum Order {
    Correct(Correct),
    Incorrect(Incorrect),
}

pub fn trim_test(left_packet: &mut String, right_packet: &mut String) -> Option<Order> {
    // trim the outermost braces first!
    loop {
        let left = trim_packet(&left_packet);
        let right = trim_packet(&right_packet);

        match (left, right) {
            (Some(leftval), Some(rightval)) => {
                left_packet.clone_from(&(leftval));
                right_packet.clone_from(&(rightval));
                continue;
            }
            (Some(_), None) => {
                return Some(Order::Incorrect(Incorrect::LeftSideRanOutItems));
            }
            (None, Some(rightval)) => {
                right_packet.clone_from(&(rightval));
                continue;
            }
            (None, None) => {
                break;
            }
        }
    }
    None
}

pub fn run(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();
    let mut pair_id: usize = 0;
    let mut right_pair_ids: Vec<usize> = Vec::new();

    for chunk in component_lines.chunks(3) {
        pair_id += 1;
        println!("--------------------------------");
        println!("processing pair_id: {}", pair_id);

        if chunk.len() == 2 || chunk.len() == 3 {
            let mut left_packet: String = chunk.get(0).expect("no packet on left!").to_string();
            let mut right_packet: String = chunk.get(1).expect("no packet on right!").to_string();

            let check = trim_test(&mut left_packet, &mut right_packet);

            if check.is_some() {
                // check for early conclusion
                println!(
                    "pair-id: {}; conclusion-reached: {:?}",
                    pair_id,
                    check.unwrap()
                );
                continue;
            }
            println!("left packer is {}", left_packet);
            println!("right packet is {}", right_packet);

            let left_elems = get_outer_elements(&left_packet);
            let right_elems = get_outer_elements(&right_packet);

            let mut item_to_compare = 0;
            let mut conclusion_reached = false;
            loop {
                let left_item = left_elems.get(&item_to_compare);

                let right_item = right_elems.get(&item_to_compare);

                match (left_item, right_item) {
                    (Some(mut leftval), Some(mut rightval)) => {
                        let check = trim_test(&mut leftval, &mut rightval);
                        if check.is_some() {
                            conclusion_reached = true;
                            break;
                        }
                    }
                    (Some(leftval), None) => {
                        //right side has run out of items!
                        conclusion_reached = true;
                        break;
                    }
                    (None, Some(rightval)) => {
                        //right side has run out items!
                        conclusion_reached = true;
                        break;
                    }
                    (None, None) => {
                        break;
                    }
                }
            }

            println!("left elems are:");
            dbg!(left_elems);

            println!("right elems are:");
            dbg!(right_elems);
        } else {
            return Err("packets must occur in pairs".into());
        };
    }

    Ok(())
}
