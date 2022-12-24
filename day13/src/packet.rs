const LEFTBRACE: char = '[';
const RIGHTBRACE: char = ']';

pub mod parse {

    use crate::packet::{LEFTBRACE, RIGHTBRACE};
    use std::collections::HashMap;

    pub fn trim_packet(line: &str) -> Option<String> {
        let message: String = line.to_string();
        //println!("trimming is being attempted on message {}", message);

        // first trim the braces at the ends
        if (message.chars().next() == Some(LEFTBRACE))
            && (message.chars().rev().next() == Some(RIGHTBRACE))
        {
            let (_, str1) = message.split_at(1);
            let (result, _) = str1.split_at(str1.len() - 1);
            let leftbracelocs: Vec<_> = result.match_indices('[').collect();
            let rightbracelocs: Vec<_> = result.match_indices(']').collect();
            // the following is just to assert that the packet message
            // is well formed!
            assert_eq!(
                leftbracelocs.len(),
                rightbracelocs.len(),
                "ill formed packet message!"
            );
            for i in 0..rightbracelocs.len() {
                if leftbracelocs
                    .get(i)
                    .expect("missing location for left brace!")
                    > rightbracelocs
                        .get(i)
                        .expect("missing location for right brace!")
                {
                    return None;
                }
            }
            //println!("trimming succesful, message is {}", str2);
            return Some(result.to_string());
        }
        None
    }

    pub fn get_outer_elements(input: &str) -> HashMap<usize, String> {
        let leftbracelocs: Vec<_> = input.match_indices('[').collect();
        let rightbracelocs: Vec<_> = input.match_indices(']').collect();

        // the following is just to assert that the packet message
        // is well formed!
        assert_eq!(
            leftbracelocs.len(),
            rightbracelocs.len(),
            "ill formed packet message!"
        );

        let numbracepairs = leftbracelocs.len();

        /*
        for elem in input.char_indices() {
            println!("elem at {} is {}", elem.0, elem.1);
        }
        for i in 0..rightbracelocs.len() {
            println!(
                "brace pair at : {}, {}",
                leftbracelocs[i].0, rightbracelocs[i].0
            );
        }*/

        let commalocs: Vec<_> = input.match_indices(',').collect();

        let locs: Vec<usize> = commalocs
            .iter()
            .filter_map(|loc| {
                let test: usize = leftbracelocs
                    .iter()
                    .zip(rightbracelocs.iter())
                    .map(|(leftloc, rightloc)| {
                        if (loc.0 > leftloc.0) && (loc.0 < rightloc.0) {
                            0
                        } else {
                            1
                        }
                    })
                    .sum();
                if test == numbracepairs {
                    Some(loc.0)
                } else {
                    None
                }
            })
            .collect();
        //println!("found {} locs!", locs.len());

        let mut outer_elems: HashMap<usize, String> = HashMap::new();
        if numbracepairs == 0 {
            let elems: Vec<_> = input.split(",").collect();
            let mut i: usize = 0;
            for elem in elems {
                outer_elems.insert(i, elem.to_string());
                i += 1;
            }
            return outer_elems;
        }

        for i in 0..=locs.len() {
            if i == 0 {
                let (str1, _) = input.split_at(locs[0]);
                /*
                println!(
                    "i is 0, loc is at {}, string to insert is {}",
                    locs[0], str1
                );*/
                outer_elems.insert(i, str1.to_string());
            } else if i == locs.len() {
                let (_, str1) = input.split_at(locs[i - 1] + 1);
                let (str2, _) = str1.split_at(input.len() - locs[i - 1] - 1);
                /*
                println!(
                    "last i is {}, loc is at {}, string to insert is {}",
                    i,
                    locs[i - 1],
                    str2
                );*/
                outer_elems.insert(i, str2.to_string());
            } else {
                let (_, str1) = input.split_at(locs[i - 1] + 1);
                let (str2, _) = str1.split_at(locs[i] - locs[i - 1] - 1);
                /*
                println!(
                    "i is {}, loc is at {}, previous loc is at {}, str1 is {}, string to insert is {}",
                    i,
                    locs[i],
                    locs[i - 1],
                    str1,
                    str2
                );*/
                outer_elems.insert(i, str2.to_string());
            }
        }

        outer_elems
    }
}
