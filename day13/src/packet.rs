const LEFTBRACE: char = '[';
const RIGHTBRACE: char = ']';

pub mod parse {

    use crate::packet::{LEFTBRACE, RIGHTBRACE};
    use id_tree::InsertBehavior::*;
    use id_tree::*;

    pub fn parse_tree(line: &str) -> Tree<i32> {
        let mut tree: Tree<i32> = TreeBuilder::new().with_node_capacity(5).build();
        // let -3 represent the root node
        let root_id: NodeId = tree.insert(Node::new(-3), AsRoot).unwrap();
        //        let mut curr = Curr {
        //            nodeid: root_id,
        //            parentval: -3,
        //        };
        let mut curr = root_id;

        let mut input = line.chars();
        let mut val_str = String::new();
        let mut outer: bool = true;
        while let Some(in_char) = input.next() {
            //println!("in_char is : {}", in_char);
            match in_char {
                LEFTBRACE => {
                    let mut val = -1;
                    if outer {
                        val = -2;
                        outer = false;
                    }
                    // let -1 represent a [] layer
                    curr = tree
                        .insert(Node::new(val), InsertBehavior::UnderNode(&curr))
                        .unwrap();
                    continue;
                }
                RIGHTBRACE => {
                    if let Ok(elem) = val_str.trim().parse::<i32>() {
                        // there will always be a parentval for a well formed
                        // packet!
                        let parentval = *tree
                            .get(&tree.get(&curr).unwrap().parent().unwrap().clone())
                            .unwrap()
                            .data();
                        if parentval == -3 {
                            curr = tree
                                .insert(Node::new(-1), InsertBehavior::UnderNode(&curr))
                                .unwrap();
                        }
                        tree.insert(Node::new(elem), InsertBehavior::UnderNode(&curr))
                            .unwrap();
                        val_str.clear();
                    }
                    curr = tree.get(&curr).unwrap().parent().unwrap().clone();
                    continue;
                }
                ',' => {
                    if let Ok(elem) = val_str.trim().parse::<i32>() {
                        let parentval = *tree
                            .get(&tree.get(&curr).unwrap().parent().unwrap().clone())
                            .unwrap()
                            .data();
                        if parentval == -3 {
                            curr = tree
                                .insert(Node::new(-1), InsertBehavior::UnderNode(&curr))
                                .unwrap();
                            tree.insert(Node::new(elem), InsertBehavior::UnderNode(&curr))
                                .unwrap();
                            val_str.clear();
                            curr = tree.get(&curr).unwrap().parent().unwrap().clone();
                        } else {
                            tree.insert(Node::new(elem), InsertBehavior::UnderNode(&curr))
                                .unwrap();
                            val_str.clear();
                        }
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
