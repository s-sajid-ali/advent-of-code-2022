const LEFTBRACE: char = '[';
const RIGHTBRACE: char = ']';

pub mod parse {

    use crate::packet::{LEFTBRACE, RIGHTBRACE};
    use id_tree::InsertBehavior::*;
    use id_tree::RemoveBehavior::DropChildren;
    use id_tree::*;

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

    pub fn compare_vals(left_val: i32, right_val: i32) -> Option<Order> {
        match left_val.cmp(&right_val) {
            std::cmp::Ordering::Less => {
                return Some(Order::Correct(Correct::LeftSideIsSmaller));
            }
            std::cmp::Ordering::Greater => {
                return Some(Order::Incorrect(Incorrect::RightSideIsSmaller));
            }
            std::cmp::Ordering::Equal => {
                return None;
            }
        }
    }

    pub fn compare(
        ltree: &mut Tree<i32>,
        lid: &NodeId,
        rtree: &mut Tree<i32>,
        rid: &NodeId,
    ) -> Option<Order> {
        let mut lcurr = lid.clone();
        let mut rcurr = rid.clone();

        let mut lcurr_parent = lid.clone();
        let mut rcurr_parent = rid.clone();

        let mut early_left: bool = false;
        let mut early_right: bool = false;

        loop {
            let lcurr_width = ltree.children_ids(&lcurr).unwrap().count();
            let rcurr_width = rtree.children_ids(&rcurr).unwrap().count();
            //println!("curr width pair is : {}/{}", lcurr_width, rcurr_width);

            match (lcurr_width, rcurr_width) {
                // can't match to 0..1 as of now :(
                (0, 2..) => {
                    early_left = true;
                }
                (1, 2..) => {
                    early_left = true;
                }
                (2.., 0) => {
                    early_right = true;
                }
                (2.., 1) => {
                    early_right = true;
                }
                (_, _) => {}
            }

            let mut lchildren_ids = ltree.children_ids(&lcurr).unwrap();
            let mut rchildren_ids = rtree.children_ids(&rcurr).unwrap();

            match (lchildren_ids.next(), rchildren_ids.next()) {
                (Some(lchild), Some(rchild)) => {
                    lcurr_parent = lcurr.clone();
                    rcurr_parent = rcurr.clone();
                    lcurr = lchild.clone();
                    rcurr = rchild.clone();
                }

                (Some(lchild), None) => {
                    if rtree.get(&rcurr).unwrap().data().clone() != -1 {
                        lcurr_parent = lcurr.clone();
                        lcurr = lchild.clone();
                        continue;
                    } else {
                        return Some(Order::Incorrect(Incorrect::RightSideRanOutItems));
                    }
                }
                (None, Some(rchild)) => {
                    if ltree.get(&lcurr).unwrap().data().clone() != -1 {
                        rcurr_parent = rcurr.clone();
                        rcurr = rchild.clone();
                        continue;
                    } else {
                        return Some(Order::Correct(Correct::LeftSideRanOutItems));
                    }
                }
                (None, None) => {
                    break;
                }
            }
        }

        let lval = ltree.get(&lcurr).unwrap().data().clone();
        let rval = rtree.get(&rcurr).unwrap().data().clone();

        /*
        println!(
            "found val pair {}/{} with early status {}/{}",
            lval, rval, early_left, early_right
        );*/

        if lval == rval {
            if early_left {
                return Some(Order::Correct(Correct::LeftSideRanOutItems));
            }
            if early_right {
                return Some(Order::Incorrect(Incorrect::RightSideRanOutItems));
            }
        }

        if lval == -1 && rval != -1 {
            return Some(Order::Correct(Correct::LeftSideRanOutItems));
        }

        if lval != -1 && rval == -1 {
            return Some(Order::Incorrect(Incorrect::RightSideRanOutItems));
        }

        let result = compare_vals(lval, rval);
        match result {
            Some(conclusion) => {
                return Some(conclusion);
            }
            None => {
                let valid_left = lcurr.clone() == lcurr_parent.clone();
                let valid_right = rcurr.clone() == rcurr_parent.clone();
                _ = ltree.remove_node(lcurr, DropChildren).unwrap();
                _ = rtree.remove_node(rcurr, DropChildren).unwrap();

                /*
                println!(
                    "at packet.rs no conclusion reached, with pair {}/{}",
                    valid_left, valid_right
                );*/

                match (valid_left, valid_right) {
                    (true, true) => {
                        return None;
                    }
                    (false, false) => compare(ltree, &lcurr_parent, rtree, &rcurr_parent),

                    (false, true) => {
                        return Some(Order::Correct(Correct::LeftSideRanOutItems));
                    }
                    (true, false) => {
                        return Some(Order::Incorrect(Incorrect::RightSideRanOutItems));
                    }
                }
            }
        }
    }

    pub fn parse_tree(line: &str) -> Tree<i32> {
        let mut tree: Tree<i32> = TreeBuilder::new().with_node_capacity(5).build();
        // let -2 represent the root node
        //let root_id: NodeId = tree.insert(Node::new(-2), AsRoot).unwrap();
        let mut curr: Option<NodeId> = None;

        let mut input = line.chars();
        let mut val_str = String::new();
        let mut outer_brace = true;
        while let Some(in_char) = input.next() {
            //println!("in_char is : {}", in_char);
            match in_char {
                LEFTBRACE => {
                    // let -1 represent a [] layer
                    if outer_brace {
                        curr = Some(tree.insert(Node::new(-1), AsRoot).unwrap());
                        outer_brace = false;
                        continue;
                    } else {
                        curr = Some(
                            tree.insert(
                                Node::new(-1),
                                InsertBehavior::UnderNode(&curr.as_ref().unwrap()),
                            )
                            .unwrap(),
                        );
                        continue;
                    }
                }
                RIGHTBRACE => {
                    if let Ok(elem) = val_str.trim().parse::<i32>() {
                        // there will always be a parentval for a well formed
                        // packet!
                        tree.insert(
                            Node::new(elem),
                            InsertBehavior::UnderNode(&curr.as_ref().unwrap()),
                        )
                        .unwrap();
                        val_str.clear();
                    }

                    if tree
                        .get(&curr.as_ref().unwrap())
                        .unwrap()
                        .parent()
                        .is_some()
                    {
                        curr = Some(
                            tree.get(&curr.as_ref().unwrap())
                                .unwrap()
                                .parent()
                                .unwrap()
                                .clone(),
                        );
                    }
                    continue;
                }
                ',' => {
                    if let Ok(elem) = val_str.trim().parse::<i32>() {
                        tree.insert(
                            Node::new(elem),
                            InsertBehavior::UnderNode(&curr.as_ref().unwrap()),
                        )
                        .unwrap();
                        val_str.clear();
                    }
                    continue;
                }
                _ => {
                    val_str.push(in_char);
                }
            }
        }

        tree
    }
}
