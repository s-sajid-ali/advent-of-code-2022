use fs_err as fs;
use itertools::Itertools;
use petgraph::algo::astar;
//use petgraph::dot::Dot;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::error::Error;

pub struct SignalGraph {
    graph: Graph<char, i32>,
    locs: Vec<NodeIndex>,
    rows: usize,
    cols: usize,
    source: (u32, u32),
    dest: (u32, u32),
}

pub fn valid_neighbor(src_val: u32, dst: char) -> Option<i32> {
    let dst_val = if dst == 'E' {
        'z'.to_digit(36)
            .expect("failed to calcualte destination value!")
    } else {
        dst.to_digit(36)
            .expect("failed to calcualte destination value!")
    };

    let srcval = if src_val == u32::MAX {
        i32::MAX
    } else {
        src_val.try_into().expect("error computing source value!")
    };
    let dstval: i32 = dst_val.try_into().expect("error computing dest value!");

    let distance: i32 = dstval
        .checked_sub(srcval as i32)
        .expect("subtraction overflow when calcuating distance!");

    if distance < 2 {
        Some(distance)
    } else {
        None
    }
}

pub fn get_graph(filename: String) -> Result<SignalGraph, Box<dyn Error>> {
    let mut contents = fs::read_to_string(filename)?;
    // remove any empty newlines at the end!
    let mut component_lines = contents.lines().collect::<Vec<_>>();
    if let Some(line) = component_lines.last() {
        if line.is_empty() {
            component_lines.pop();
        }
    }

    let cols: usize = component_lines
        .first()
        .expect("first line of input is empty!")
        .len();
    let rows: usize = component_lines.len();
    let size: usize = rows.checked_mul(cols).expect("rows * cols overflow!");
    println!("rows: {}, cols: {}, size: {}", rows, cols, size);

    contents = contents.replace("\r\n", "\n"); // cool bear windows!
    contents = contents.replace("\n", "");

    println!("length of contents vector is {}", contents.len());

    let mut field = Graph::<char, i32>::new();

    let locs: Vec<NodeIndex> = contents.chars().map(|elem| field.add_node(elem)).collect();

    let mut sourceloc: Option<(u32, u32)> = None;
    let mut destloc: Option<(u32, u32)> = None;

    let mut it = (0..rows).cartesian_product(0..cols);
    while let Some((rowidx, colidx)) = it.next() {
        let locidx = rowidx * cols + colidx;
        if field.raw_nodes().get(locidx).expect("missing node!").weight == 'S' {
            sourceloc = Some((
                rowidx.try_into().expect("rowidx is too large!"),
                colidx.try_into().expect("colidx is too large!"),
            ));
        }
        if field.raw_nodes().get(locidx).expect("missing node!").weight == 'E' {
            destloc = Some((
                rowidx.try_into().expect("rowidx is too large!"),
                colidx.try_into().expect("colidx is too large!"),
            ));
        }

        let src = field.raw_nodes()[locidx].weight;
        let srcval = if src == 'S' {
            'a'.to_digit(36)
                .expect("failed to calcualte source location!")
        } else {
            src.to_digit(36)
                .expect("failed to calcualte source location!")
        };

        // neighbor right
        if colidx != cols - 1 {
            let neighboridx = locidx + 1;
            let elem = locs.get(locidx).expect("missing node!");
            let neighbor = locs.get(neighboridx).expect("mising node!");
            let dst = field.raw_nodes()[neighboridx].weight;
            if let Some(dist) = valid_neighbor(srcval, dst) {
                //println!("neighbor right at distance {}!", dist);
                field.add_edge(*elem, *neighbor, dist);
            }
        }

        // neighbor left
        if colidx != 0 {
            let neighboridx = locidx - 1;
            let elem = locs.get(locidx).expect("missing node!");
            let neighbor = locs.get(neighboridx).expect("mising node!");
            let dst = field.raw_nodes()[neighboridx].weight;
            if let Some(dist) = valid_neighbor(srcval, dst) {
                //println!("neighbor left at distance {}!", dist);
                field.add_edge(*elem, *neighbor, dist);
            }
        }

        // neighbor up
        if rowidx != 0 {
            let neighboridx = locidx - cols;
            let elem = locs.get(locidx).expect("missing node!");
            let neighbor = locs.get(neighboridx).expect("mising node!");
            let dst = field.raw_nodes()[neighboridx].weight;
            if let Some(dist) = valid_neighbor(srcval, dst) {
                //println!("neighbor up at distance {}!", dist);
                field.add_edge(*elem, *neighbor, dist);
            }
        }

        // neighbor down
        if rowidx != rows - 1 {
            let neighboridx = locidx + cols;
            let elem = locs.get(locidx).expect("missing node!");
            let neighbor = locs.get(neighboridx).expect("mising node!");
            let dst = field.raw_nodes()[neighboridx].weight;
            if let Some(dist) = valid_neighbor(srcval, dst) {
                //println!("neighbor down at distance {}!", dist);
                field.add_edge(*elem, *neighbor, dist);
            }
        }
    }
    let srcloc = sourceloc.expect("there is no source in input!");
    let dstloc = destloc.expect("there is no destination in input1");

    Ok(SignalGraph {
        graph: field,
        locs,
        rows,
        cols,
        source: srcloc,
        dest: dstloc,
    })
}

pub fn run_part2(filename: String) -> Result<(), Box<dyn Error>> {
    let signalgraph = get_graph(filename).expect("could not construct graph!");

    let field = signalgraph.graph;
    let locs = signalgraph.locs;
    let rows = signalgraph.rows;
    let cols = signalgraph.cols;
    let _srcloc = signalgraph.source;
    let dstloc = signalgraph.dest;

    // optionally visualize field
    // println!("{}", Dot::new(&field));

    let mut sources: Vec<(u32, u32)> = Vec::new();
    let mut it = (0..rows).cartesian_product(0..cols);
    while let Some((rowidx, colidx)) = it.next() {
        let locidx = rowidx * cols + colidx;
        if field.raw_nodes().get(locidx).expect("missing node!").weight == 'S' {
            sources.push((
                rowidx.try_into().expect("rowidx is too large!"),
                colidx.try_into().expect("colidx is too large!"),
            ));
        }
        if field.raw_nodes().get(locidx).expect("missing node!").weight == 'a' {
            sources.push((
                rowidx.try_into().expect("rowidx is too large!"),
                colidx.try_into().expect("colidx is too large!"),
            ));
        }
    }

    let shortest_distance = sources
        .iter()
        .filter_map(|srcloc| {
            let srclocidx = srcloc.0 as usize * cols + srcloc.1 as usize;
            let dstlocidx = dstloc.0 as usize * cols + dstloc.1 as usize;

            if let Some((distance, _path)) = astar(
                &field,
                *locs.get(srclocidx).expect("missing source!"),
                |finish| finish == *locs.get(dstlocidx).expect("missing dst!"),
                |_| 1,
                |_| 1,
            ) {
                return Some(distance);
            } else {
                return None;
            }
        })
        .min();

    println!(
        "shortest distance for all possible starting points is {}",
        shortest_distance.expect("no shortest distance found!")
    );

    Ok(())
}

pub fn run_part1(filename: String) -> Result<(), Box<dyn Error>> {
    let signalgraph = get_graph(filename).expect("could not construct graph!");

    let field = signalgraph.graph;
    let locs = signalgraph.locs;
    let _rows = signalgraph.rows;
    let cols = signalgraph.cols;
    let srcloc = signalgraph.source;
    let dstloc = signalgraph.dest;

    // optionally visualize field
    // println!("{}", Dot::new(&field));

    let srclocidx = srcloc.0 as usize * cols + srcloc.1 as usize;
    let dstlocidx = dstloc.0 as usize * cols + dstloc.1 as usize;

    println!(
        "source is at : {},{}; srcloc-idx is {}",
        srcloc.0, srcloc.1, srclocidx
    );
    println!(
        "dst is at : {},{}; dstloc-idx is {}",
        dstloc.0, dstloc.1, dstlocidx
    );

    if let Some((distance, _path)) = astar(
        &field,
        *locs.get(srclocidx).expect("missing source!"),
        |finish| finish == *locs.get(dstlocidx).expect("missing dst!"),
        |_| 1,
        |_| 1,
    ) {
        println!("shortest path between S and E is {} steps", distance);
        //dbg!(path);
    } else {
        return Err("error calculating shortest path".into());
    }

    Ok(())
}
