const LEFTBRACE: char = '[';
const RIGHTBRACE: char = ']';

pub mod parse {

    use crate::packet::{LEFTBRACE, RIGHTBRACE};
    use id_tree::InsertBehavior::*;
    use id_tree::RemoveBehavior::DropChildren;
    use id_tree::*;
    use itertools::EitherOrBoth::{Both, Left, Right};
    use itertools::Itertools;

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

        let mut early_left: bool = false;
        let mut early_right: bool = false;

        let mut advance_left: bool = true;
        let mut advance_right: bool = true;

        loop {
            let lcurr_width = ltree.children_ids(&lcurr).unwrap().count();
            let rcurr_width = rtree.children_ids(&rcurr).unwrap().count();

            match (lcurr_width, rcurr_width) {
                (1, 1..) => {
                    early_left = true;
                }
                (1.., 1) => {
                    early_right = true;
                }
                (_, _) => {}
            }

            let mut lchildren_ids = ltree.children_ids(&lcurr).unwrap();
            let mut rchildren_ids = rtree.children_ids(&rcurr).unwrap();

            if advance_left {
                match lchildren_ids.next() {
                    Some(child) => {
                        lcurr = child.clone();
                    }
                    None => {
                        advance_left = false;
                    }
                }
            }

            if advance_right {
                match rchildren_ids.next() {
                    Some(child) => {
                        rcurr = child.clone();
                    }
                    None => {
                        advance_right = false;
                    }
                }
            }
            if !advance_left && !advance_right {
                break;
            }
        }

        let lval = ltree.get(&lcurr).unwrap().data().clone();
        let rval = rtree.get(&rcurr).unwrap().data().clone();

        _ = ltree.remove_node(lcurr, DropChildren).unwrap();
        _ = rtree.remove_node(rcurr, DropChildren).unwrap();

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

        return compare_vals(lval, rval);
    }

    pub fn get_value(tree: &mut Tree<i32>, id: &NodeId) -> Option<i32> {
        let mut curr = id.clone();
        let mut width: Vec<usize> = Vec::new();

        loop {
            let curr_width = tree.children_ids(&curr).unwrap().count();
            let mut children_ids = tree.children_ids(&curr).unwrap();
            match children_ids.next() {
                Some(child) => {
                    width.push(curr_width);
                    curr = child.clone();
                }
                None => {
                    break;
                }
            }
        }

        let mut retval: Option<i32> = None;
        let val = tree.get(&curr).unwrap().data();
        if *val != -1 {
            retval = Some(*val);
        }
        println!("elem {}, widths are", *val);
        for w in width {
            print!("{}, ", w);
        }
        println!("");

        _ = tree.remove_node(curr, DropChildren).unwrap();

        retval
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
