use fs_err as fs;
use std::error::Error;

mod packet;
use crate::packet::parse::{Correct, Incorrect, Order};
use crate::packet::{LEFTBRACE, RIGHTBRACE};

pub fn test(left_packet: &str, right_packet: &str) -> Order {
    let mut advance_left = true;
    let mut advance_right = true;
    let mut left_elem: Option<char> = None;
    let mut right_elem: Option<char> = None;

    let mut left: Option<i32> = None;
    let mut right: Option<i32> = None;
    let mut left_depth: i32 = 0;
    let mut right_depth: i32 = 0;

    let mut transition_left = false;
    let mut transition_right = false;

    let mut left_iter = left_packet.chars().peekable();
    let mut right_iter = right_packet.chars().peekable();

    loop {
        if advance_left {
            left_elem = left_iter.next();
        }
        if advance_right {
            right_elem = right_iter.next();
        }

        match (left_elem, right_elem) {
            (Some(left_val), Some(right_val)) => {
                // try to see if they are numbers
                if left_val.is_digit(10) {
                    let mut left_str = String::new();
                    left_str.push(left_val);
                    if left_iter.peek().is_some() {
                        if left_iter.peek().unwrap().is_digit(10) {
                            left_str.push(*left_iter.peek().unwrap());
                        }
                    }
                    left = Some(left_str.parse::<i32>().expect("non-numeric input!"));
                }
                if right_val.is_digit(10) {
                    let mut right_str = String::new();
                    right_str.push(right_val);
                    if right_iter.peek().is_some() {
                        if right_iter.peek().unwrap().is_digit(10) {
                            right_str.push(*right_iter.peek().unwrap());
                        }
                    }
                    right = Some(right_str.parse::<i32>().expect("non-numeric input!"));
                }
                match (left, right) {
                    // both are numbers, compare them
                    (Some(lnum), Some(rnum)) => {
                        println!("comapring number pair {}/{}", lnum, rnum);

                        if lnum < rnum {
                            return Order::Correct(Correct::LeftSideIsSmaller);
                        }
                        if rnum < lnum {
                            return Order::Incorrect(Incorrect::RightSideIsSmaller);
                        }
                        if lnum == rnum {
                            left = None;
                            right = None;
                            advance_left = true;
                            advance_right = true;
                        }
                    }
                    (Some(_), None) => {
                        advance_left = false;
                        advance_right = true;
                        if transition_right {
                            return Order::Incorrect(Incorrect::RightSideRanOutItems);
                        }
                    }
                    (None, Some(_)) => {
                        advance_left = true;
                        advance_right = false;
                        if transition_left {
                            return Order::Correct(Correct::LeftSideRanOutItems);
                        }
                    }
                    _ => {}
                }

                if left_val == LEFTBRACE {
                    left_depth += 1;
                    transition_left = false;
                }
                if left_val == RIGHTBRACE {
                    left_depth -= 1;
                    transition_left = true;
                }
                if right_val == LEFTBRACE {
                    right_depth += 1;
                    transition_right = false;
                }
                if right_val == RIGHTBRACE {
                    right_depth -= 1;
                    transition_right = true;
                }

                println!(
                    "reading character pair at is {}/{}; at depth difference {}; transition status {}/{}",
                    left_val, right_val, (left_depth - right_depth), transition_left, transition_right
                );

                if left_iter.peek().is_some()
                    && *left_iter.peek().unwrap() == ','
                    && (left_depth - right_depth).abs() > 1
                {
                    return Order::Correct(Correct::LeftSideRanOutItems);
                }
                if right_iter.peek().is_some()
                    && *right_iter.peek().unwrap() == ','
                    && (left_depth - right_depth).abs() > 1
                {
                    return Order::Incorrect(Incorrect::RightSideRanOutItems);
                }

                if transition_left && !transition_right {
                    return Order::Correct(Correct::LeftSideRanOutItems);
                }
                if !transition_left && transition_right {
                    return Order::Incorrect(Incorrect::RightSideRanOutItems);
                }

                continue;
            }

            (Some(_), None) => {
                return Order::Incorrect(Incorrect::RightSideRanOutItems);
            }
            (None, Some(_)) => {
                return Order::Correct(Correct::LeftSideRanOutItems);
            }
            _ => {}
        }

        break;
    }

    return Order::Correct(Correct::LeftSideIsSmaller);
}

pub fn run(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();
    let right_pair_ids: usize = component_lines
        .chunks(3)
        .enumerate()
        .filter_map(|(pair_id, chunk)| {
            println!("--------------------------------");
            println!("processing pair_id: {}", pair_id + 1);

            if !(chunk.len() == 2 || chunk.len() == 3) {
                eprintln!("packets must occur in pairs");
                return None::<usize>;
            }

            let left_packet: String = chunk.get(0).expect("no packet on left!").to_string();
            let right_packet: String = chunk.get(1).expect("no packet on right!").to_string();
            /*
            let check = trim_test(&mut left_packet, &mut right_packet);
            if check.is_some() {
                // check for early conclusion
                println!(
                    "pair-id: {}; conclusion-reached: {:?}",
                    pair_id,
                    check.unwrap()
                );
                return None;
            }*/

            println!("left packet is {}", left_packet);
            println!("right packet is {}", right_packet);

            let result = test(&left_packet, &right_packet);
            println!("result is {:?}", result);

            match result {
                Order::Correct(..) => {
                    println!("found correct ordering at count {}", pair_id + 1);
                    return Some(pair_id + 1);
                }
                _ => {
                    return None;
                }
            }
        })
        .sum();

    println!("");
    println!("--------------------------------");
    println!("sum of right pair ids is : {}", right_pair_ids);

    Ok(())
}
