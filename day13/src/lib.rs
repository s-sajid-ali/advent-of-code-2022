use fs_err as fs;
use id_tree::*;
use std::error::Error;

mod packet;
use crate::packet::parse::parse_tree;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Correct {
    LeftSideRanOutItems,
    LeftSideIsSmaller,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Incorrect {
    RightSideRanOutItems,
    RightSideIsSmaller,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Order {
    Correct(Correct),
    Incorrect(Incorrect),
}

pub fn test(left_tree: Tree<i32>, right_tree: Tree<i32>) -> Order {
    let left_root = left_tree
        .root_node_id()
        .expect("left packet tree has no root!");
    let right_root = right_tree
        .root_node_id()
        .expect("right packet tree has no root!");

    let mut left_iter = left_tree.traverse_pre_order(&left_root).unwrap();
    let mut right_iter = right_tree.traverse_pre_order(&right_root).unwrap();

    loop {
        let left_elem = left_iter.next();
        let right_elem = right_iter.next();

        match (left_elem, right_elem) {
            (Some(left_node), Some(right_node)) => {
                let left_val = *left_node.data();
                let right_val = *right_node.data();

                if left_val == -3 && right_val == -3 {
                    // at root nodes, skip this comparision
                    continue;
                }

                let left_parent_is_root =
                    *left_tree.get(&left_node.parent().unwrap()).unwrap().data() == -2;
                let right_parent_is_root = *right_tree
                    .get(&right_node.parent().unwrap())
                    .unwrap()
                    .data()
                    == -2;
                println!(
                    "comparing pair : {}/{}, parents are roots: {}/{}",
                    left_val, right_val, left_parent_is_root, right_parent_is_root
                );

                match (left_parent_is_root, right_parent_is_root) {
                    (true, false) => {
                        return Order::Correct(Correct::LeftSideRanOutItems);
                    }
                    (false, true) => {
                        return Order::Incorrect(Incorrect::RightSideRanOutItems);
                    }
                    _ => {
                        if left_val == -1 && right_val != -1 {
                            return Order::Correct(Correct::LeftSideRanOutItems);
                        }
                        if left_val != -1 && right_val == -1 {
                            return Order::Incorrect(Incorrect::RightSideRanOutItems);
                        }
                        if left_val < right_val {
                            return Order::Correct(Correct::LeftSideIsSmaller);
                        }

                        if left_val > right_val {
                            return Order::Incorrect(Incorrect::RightSideIsSmaller);
                        }
                    }
                }
            }

            (Some(_), None) => {
                return Order::Incorrect(Incorrect::RightSideRanOutItems);
            }

            (None, Some(_)) => {
                return Order::Correct(Correct::LeftSideRanOutItems);
            }

            (None, None) => {
                return Order::Correct(Correct::LeftSideIsSmaller);
            }
        }
    }
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
