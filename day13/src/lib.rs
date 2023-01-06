use fs_err as fs;
use id_tree::*;
use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::error::Error;

mod packet;
use crate::packet::parse::{compare_vals, get_value, parse_tree};
use crate::packet::parse::{Correct, Incorrect, Order};

pub fn test(mut left_tree: Tree<i32>, mut right_tree: Tree<i32>) -> Order {
    let left_root = left_tree
        .root_node_id()
        .expect("left packet tree has no root!");
    let right_root = right_tree
        .root_node_id()
        .expect("right packet tree has no root!");

    let mut left_children: Vec<NodeId> = Vec::new();
    let mut right_children: Vec<NodeId> = Vec::new();
    for lchild in left_tree.children_ids(&left_root).unwrap() {
        left_children.push(lchild.clone());
    }
    for rchild in right_tree.children_ids(&right_root).unwrap() {
        right_children.push(rchild.clone());
    }

    let it = left_children
        .into_iter()
        .zip_longest(right_children.into_iter());

    for elem in it {
        match elem {
            Both(left_child, right_child) => loop {
                let left = get_value(&mut left_tree, &left_child);
                let right = get_value(&mut right_tree, &right_child);
                let empty_left = left_tree.get(&left_child).is_err();
                let empty_right = right_tree.get(&right_child).is_err();
                match (left, right) {
                    (Some(left_val), Some(right_val)) => {
                        println!("pair of vals extracted are : {}/{}", left_val, right_val);
                        let result = compare_vals(left_val, right_val);
                        if result.is_some() {
                            return result.unwrap();
                        }
                    }
                    (Some(_), None) => {
                        println!("number only on left");
                        return Order::Incorrect(Incorrect::RightSideRanOutItems);
                    }
                    (None, Some(_)) => {
                        println!("number only on right");
                        return Order::Correct(Correct::LeftSideRanOutItems);
                    }
                    (None, None) => {
                        println!("no number on both sides");
                    }
                }
                match (empty_left, empty_right) {
                    (false, true) => {
                        return Order::Incorrect(Incorrect::RightSideRanOutItems);
                    }
                    (true, false) => {
                        return Order::Correct(Correct::LeftSideRanOutItems);
                    }
                    (false, false) => {
                        continue;
                    }
                    (true, true) => {
                        break;
                    }
                }
            },
            Left(_) => {
                return Order::Incorrect(Incorrect::RightSideRanOutItems);
            }
            Right(_) => {
                return Order::Correct(Correct::LeftSideRanOutItems);
            }
        }
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

            let left_tree = parse_tree(&left_packet);
            let right_tree = parse_tree(&right_packet);

            let mut leftstr = String::new();
            left_tree
                .clone()
                .write_formatted(&mut leftstr)
                .expect("cannot format tree");
            println!("left tree is \n{}", leftstr);
            let mut rightstr = String::new();
            right_tree
                .clone()
                .write_formatted(&mut rightstr)
                .expect("cannot format tree");
            println!("right tree is \n{}", rightstr);

            let result = test(left_tree, right_tree);
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
