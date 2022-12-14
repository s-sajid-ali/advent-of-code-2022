use fs_err as fs;
use std::collections::HashSet;
use std::error::Error;

mod rope;
use crate::rope::headtail::Direction;
use crate::rope::headtail::Point;

pub fn run(filename: String, rope_length: usize) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let component_lines = contents.lines().collect::<Vec<_>>();

    let mut rope: Vec<Point> = Vec::with_capacity(rope_length);
    for _ in 0..rope_length {
        rope.push(Point::new(0, 0));
    }

    let mut tail_state: HashSet<(i32, i32)> = HashSet::new();

    for line in component_lines.iter() {
        if line.is_empty() {
            // account for any extra newline at end
            continue;
        } else {
            let input_vec: Vec<_> = line.split_whitespace().collect();
            assert_eq!(
                input_vec.len(),
                2,
                "Each line in the input file should have only two characters!"
            );

            let dir = input_vec[0];
            let dir: Option<Direction> = match dir {
                "R" => Some(Direction::Right),
                "L" => Some(Direction::Left),
                "U" => Some(Direction::Up),
                "D" => Some(Direction::Down),
                _ => None,
            };
            let dir = dir.expect("invalid direction given in input");

            let dist = input_vec[1]
                .parse::<u32>()
                .expect("invalid distance in input");

            println!("moving {} distance in direction {:?}", dist, dir);

            for _ in 0..dist {
                rope[0].update(dir);
                for idx in 0..rope_length - 1 {
                    let possibly_move_tail = rope[idx].to_move_tail(&rope[idx + 1]);
                    match possibly_move_tail {
                        Some(tail_move_dir) => {
                            println!("at idx {}, tail is moving along : {:?}", idx, tail_move_dir);
                            rope[idx + 1].update(tail_move_dir);
                        }
                        None => {}
                    }
                }
                for elem in &rope {
                    println!("rope element at {}", elem);
                }
                _ = tail_state.insert(rope[rope_length - 1].get_coords());
            }
        };
    }

    println!(
        "number of unique locations tail has covered: {}",
        tail_state.len()
    );

    //dbg!(tail_state);

    Ok(())
}
