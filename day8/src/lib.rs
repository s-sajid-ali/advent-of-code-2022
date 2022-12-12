use fs_err as fs;
use itertools::Itertools;
use nalgebra::DMatrix;
use std::error::Error;

pub fn get_tree_matrix(filename: String) -> Result<DMatrix<u32>, Box<dyn Error>> {
    let mut contents = fs::read_to_string(filename)?;
    contents = contents.replace("\r\n", "\n"); // cool bear windows!
    contents = contents.replace("\n", "");

    if (contents.len() as f64).sqrt().floor() != (contents.len() as f64).sqrt() {
        return Err("Could not convert input to a square matrix!".into());
    }

    let dim: usize = (contents.len() as f64).sqrt().floor() as usize;

    println!("dim is {}x{}", dim, dim);

    let tree_matrix = DMatrix::<u32>::from_row_iterator(
        dim,
        dim,
        contents
            .chars()
            .map(|x| x.to_digit(10).expect("parsing input sequence failed"))
            .into_iter(),
    );

    Ok(tree_matrix)
}

pub fn run_treescore(filename: String) -> Result<(), Box<dyn Error>> {
    let tree_matrix =
        get_tree_matrix(filename).expect("could not convert input sequence to tree matrix");

    let (dim, _) = tree_matrix.shape();

    let it = (1..dim - 1).cartesian_product(1..dim - 1);

    let highest_treescore: u64 = it
        .map(|(rowidx, colidx)| {
            let tree_height = tree_matrix[(rowidx, colidx)];

            let mut score_left = 0;
            for i in (0..colidx).rev() {
                if tree_height > tree_matrix[(rowidx, i)] {
                    //println!("left tree at {},{}", rowidx, i);
                    score_left += 1;
                } else {
                    score_left += 1;
                    break;
                }
            }

            let mut score_right = 0;
            for i in colidx + 1..dim {
                if tree_height > tree_matrix[(rowidx, i)] {
                    score_right += 1;
                } else {
                    score_right += 1;
                    break;
                }
            }

            let mut score_top = 0;
            for i in (0..rowidx).rev() {
                if tree_height > tree_matrix[(i, colidx)] {
                    //println!("top tree at {},{}", rowidx, i);
                    score_top += 1;
                } else {
                    score_top += 1;
                    break;
                }
            }

            let mut score_bottom = 0;
            for i in rowidx + 1..dim {
                if tree_height > tree_matrix[(i, colidx)] {
                    score_bottom += 1;
                } else {
                    score_bottom += 1;
                    break;
                }
            }

            /*
            println!(
                "tree with height {} at {},{} has treescore is {},{},{},{} -> {}",
                tree_height,
                rowidx,
                colidx,
                score_left,
                score_right,
                score_top,
                score_bottom,
                score_left * score_right * score_top * score_bottom
            );*/

            score_left * score_right * score_top * score_bottom
        })
        .max()
        .expect("error computation highest treescore from set of tree scores");

    println!("highest treescore is {}", highest_treescore);

    Ok(())
}

pub fn run_visibility(filename: String) -> Result<(), Box<dyn Error>> {
    let tree_matrix =
        get_tree_matrix(filename).expect("could not convert input sequence to tree matrix");

    let (dim, _) = tree_matrix.shape();

    let it = (1..dim - 1).cartesian_product(1..dim - 1);

    let visible_trees: u64 = it
        .map(|(rowidx, colidx)| {
            let tree_height = tree_matrix[(rowidx, colidx)];

            let slice_left = &tree_matrix.slice((rowidx, 0), (1, colidx));
            let left: bool = tree_height > slice_left.max();

            let slice_right = &tree_matrix.slice((rowidx, colidx + 1), (1, dim - colidx - 1));
            let right: bool = tree_height > slice_right.max();

            let slice_top = &tree_matrix.slice((0, colidx), (rowidx, 1));
            let top: bool = tree_height > slice_top.max();

            let slice_bottom = &tree_matrix.slice((rowidx + 1, colidx), (dim - rowidx - 1, 1));
            let bottom: bool = tree_height > slice_bottom.max();

            if left | right | top | bottom {
                /*
                println!(
                    "tree height of {}   visible at {},{} due to reason {},{},{},{}",
                    tree_height, rowidx, colidx, left, right, top, bottom
                );*/
                1
            } else {
                /*
                println!(
                    "tree height of {} invisible at {},{} due to reason {},{},{},{}",
                    tree_height, rowidx, colidx, left, right, top, bottom
                );*/
                0
            }
        })
        .sum();

    // account the trees on the edges!
    let edge_trees: u64 = ((dim * dim) - ((dim - 2) * (dim - 2)))
        .try_into()
        .expect("tree edge count overflow!");
    println!("number of visible trees is {}", visible_trees + edge_trees);

    //dbg!(tree_matrix);

    Ok(())
}
