use fs_err as fs;
use id_tree::InsertBehavior::*;
use id_tree::*;
use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::cmp::Ordering;
use std::error::Error;

mod packet;
use crate::packet::parse::{compare, parse_tree};
use crate::packet::parse::{Correct, Incorrect, Order};

pub fn test(mut left_tree: Tree<i32>, mut right_tree: Tree<i32>) -> Option<Order> {
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
                let result = compare(&mut left_tree, &left_child, &mut right_tree, &right_child);
                match result {
                    Some(conclusion) => {
                        return Some(conclusion);
                    }
                    None => {
                        //println!("at lib.rs no conclusion reached, checking next item!");
                        break;
                    }
                }
            },
            Left(_) => {
                return Some(Order::Incorrect(Incorrect::RightSideRanOutItems));
            }
            Right(_) => {
                return Some(Order::Correct(Correct::LeftSideRanOutItems));
            }
        }
    }

    None
}

pub fn run_part2(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let mut packets: Vec<Tree<i32>> = contents
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                let packet = parse_tree(line);
                return Some(packet);
            }
        })
        .collect();

    let mut marker1: Tree<i32> = TreeBuilder::new().with_node_capacity(3).build();
    let mut marker2: Tree<i32> = TreeBuilder::new().with_node_capacity(3).build();
    let rootid_1: NodeId = marker1.insert(Node::new(-1), AsRoot).unwrap();
    let rootid_2 = marker2.insert(Node::new(-1), AsRoot).unwrap();
    let childid_1 = marker1.insert(Node::new(-1), UnderNode(&rootid_1)).unwrap();
    let childid_2 = marker2.insert(Node::new(-1), UnderNode(&rootid_2)).unwrap();
    marker1.insert(Node::new(2), UnderNode(&childid_1)).unwrap();
    marker2.insert(Node::new(6), UnderNode(&childid_2)).unwrap();

    packets.push(marker1.clone());
    packets.push(marker2.clone());

    packets.sort_by(|a, b| {
        let a_tmp = a.clone();
        let b_tmp = b.clone();
        let result = test(a_tmp, b_tmp);
        if result.is_none() {
            eprintln!("packet comparision unsuccessfull!");
            let mut leftstr = String::new();
            a.clone()
                .write_formatted(&mut leftstr)
                .expect("cannot format tree");
            println!("left tree is \n{}", leftstr);
            let mut rightstr = String::new();
            b.clone()
                .write_formatted(&mut rightstr)
                .expect("cannot format tree");
            println!("right tree is \n{}", rightstr);
        }
        match result {
            Some(Order::Correct(..)) => Ordering::Less,
            Some(Order::Incorrect(..)) => Ordering::Greater,
            None => Ordering::Equal,
        }
    });

    let mut pos1: Option<usize> = None;
    let mut pos2: Option<usize> = None;
    for (pos, elem) in packets.iter().enumerate() {
        if *elem == marker1 {
            pos1 = Some(pos + 1);
        }
        if *elem == marker2 {
            pos2 = Some(pos + 1);
        }
    }

    println!(
        "position of first marker is {}",
        pos1.expect("first marker packet missing!")
    );
    println!(
        "position of second marker is {}",
        pos2.expect("first marker packet missing!")
    );

    Ok(())
}

pub fn run_part1(filename: String) -> Result<(), Box<dyn Error>> {
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
            match result {
                None => {
                    eprintln!("failed to reach conclusion for pair id {}", pair_id + 1);
                    return None;
                }
                Some(conclusion) => {
                    println!("result is {:?}", conclusion);
                    match conclusion {
                        Order::Correct(..) => {
                            println!("found correct ordering at count {}", pair_id + 1);
                            return Some(pair_id + 1);
                        }
                        _ => {
                            return None;
                        }
                    }
                }
            }
        })
        .sum();

    println!("");
    println!("--------------------------------");
    println!("sum of right pair ids is : {}", right_pair_ids);

    Ok(())
}
