pub mod reservoir {

    use fs_err as fs;
    use std::collections::HashMap;
    use std::fmt;

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Matter {
        Rock,
        Sand,
    }

    pub struct Canvas {
        x_min: u32,
        x_max: u32,
        y_min: u32,
        y_max: u32,
        source_x: u32,
        source_y: u32,
        fill: HashMap<(u32, u32), Matter>,
    }

    impl fmt::Display for Canvas {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let num_rocks: usize = self
                .fill
                .values()
                .filter_map(|value| {
                    if *value == Matter::Rock {
                        Some(1)
                    } else {
                        None
                    }
                })
                .sum();
            let num_sand_particles: usize = self
                .fill
                .values()
                .filter_map(|value| {
                    if *value == Matter::Sand {
                        Some(1)
                    } else {
                        None
                    }
                })
                .sum();

            write!(
                f,
                "corners are ({},{}); ({},{}), with source at ({},{}) with {} rocks and {} sand particles",
                self.x_min,
                self.y_min,
                self.x_max,
                self.y_max,
                self.source_x,
                self.source_y,
                num_rocks,
                num_sand_particles
            )
        }
    }

    impl Canvas {
        pub fn render(&self) {
            for y in self.y_min..(self.y_max + 2) {
                let mut tmp: String = String::new();
                for x in self.x_min..self.x_max {
                    let elem = &self.fill.get(&(x, y));
                    match elem {
                        None => tmp.push('.'),
                        Some(Matter::Rock) => tmp.push('#'),
                        Some(Matter::Sand) => tmp.push('o'),
                    }
                }
                println!("{tmp}");
            }
        }

        fn possible_move(&self, test_loc: (u32, u32)) -> bool {
            if (test_loc.0 < self.x_min) || (test_loc.0 > self.x_max) || (test_loc.1 > self.y_max) {
                false
            } else {
                true
            }
        }

        pub fn fill_sand_infinite(&mut self) {
            let mut sand_loc = (self.source_x, self.source_y);
            let mut test_loc: (u32, u32);

            loop {
                // try to move down
                test_loc = (sand_loc.0, sand_loc.1 + 1);
                match self.fill.get(&(test_loc)) {
                    Some(_) => {}
                    None => {
                        sand_loc = test_loc;
                        if test_loc.1 == self.y_max + 1 {
                            self.fill.insert(sand_loc, Matter::Sand);
                            if test_loc.0 < self.x_min {
                                self.x_min = test_loc.0
                            }
                            if test_loc.1 > self.x_max {
                                self.x_max = test_loc.0
                            }
                            sand_loc = (self.source_x, self.source_y);
                        }
                        continue;
                    }
                }

                // try to move down and left
                test_loc = (sand_loc.0 - 1, sand_loc.1 + 1);
                match self.fill.get(&test_loc) {
                    Some(_) => {}
                    None => {
                        sand_loc = test_loc;
                        if test_loc.1 == self.y_max + 1 {
                            self.fill.insert(sand_loc, Matter::Sand);
                            if test_loc.0 < self.x_min {
                                self.x_min = test_loc.0
                            }
                            if test_loc.1 > self.x_max {
                                self.x_max = test_loc.0
                            }
                            sand_loc = (self.source_x, self.source_y);
                        }
                        continue;
                    }
                }

                // try to move down and right
                test_loc = (sand_loc.0 + 1, sand_loc.1 + 1);
                match self.fill.get(&test_loc) {
                    Some(_) => {}
                    None => {
                        sand_loc = test_loc;
                        if test_loc.1 == self.y_max + 1 {
                            self.fill.insert(sand_loc, Matter::Sand);
                            if test_loc.0 < self.x_min {
                                self.x_min = test_loc.0
                            }
                            if test_loc.1 > self.x_max {
                                self.x_max = test_loc.0
                            }
                            sand_loc = (self.source_x, self.source_y);
                        }
                        continue;
                    }
                }

                match (sand_loc.0 == self.source_x) && (sand_loc.1 == self.source_y) {
                    false => {
                        println!("inserting sand particle! at {}/{}", sand_loc.0, sand_loc.1);
                        self.fill.insert(sand_loc, Matter::Sand);

                        //reset to new sand particle
                        sand_loc = (self.source_x, self.source_y);

                        // optionally see where the particle was added!
                        //self.render();
                        continue;
                    }
                    true => {
                        println!(
                            "last sand particle inserted at source {}/{}",
                            sand_loc.0, sand_loc.1
                        );
                        self.fill.insert(sand_loc, Matter::Sand);

                        break;
                    }
                }
            }
        }

        pub fn fill_sand(&mut self) {
            let mut sand_loc = (self.source_x, self.source_y);
            let mut test_loc: (u32, u32);

            loop {
                // try to move down
                test_loc = (sand_loc.0, sand_loc.1 + 1);
                match self.fill.get(&(test_loc)) {
                    Some(_) => {}
                    None => match self.possible_move(test_loc) {
                        true => {
                            sand_loc = test_loc;
                            continue;
                        }
                        false => {
                            break;
                        }
                    },
                }

                // try to move down and left
                test_loc = (sand_loc.0 - 1, sand_loc.1 + 1);
                match self.fill.get(&test_loc) {
                    Some(_) => {}
                    None => match self.possible_move(test_loc) {
                        true => {
                            sand_loc = test_loc;
                            continue;
                        }
                        false => {
                            break;
                        }
                    },
                }

                // try to move down and right
                test_loc = (sand_loc.0 + 1, sand_loc.1 + 1);
                match self.fill.get(&test_loc) {
                    Some(_) => {}
                    None => match self.possible_move(test_loc) {
                        true => {
                            sand_loc = test_loc;
                            continue;
                        }
                        false => {
                            break;
                        }
                    },
                }

                println!("inserting sand particle! at {}/{}", sand_loc.0, sand_loc.1);
                self.fill.insert(sand_loc, Matter::Sand);

                //reset to new sand particle
                sand_loc = (self.source_x, self.source_y);

                //self.render();
                continue;
            }
        }

        pub fn new(filename: String, source_location: (u32, u32)) -> Canvas {
            let contents = fs::read_to_string(filename).expect("input file not found!");
            let mut lines = contents.lines();

            let mut x_min = u32::MAX;
            let mut x_max = u32::MIN;
            let mut y_min = u32::MAX;
            let mut y_max = u32::MIN;

            let mut fill: HashMap<(u32, u32), Matter> = HashMap::new();

            while let Some(line) = lines.next() {
                let mut start: Option<(u32, u32)> = None;
                let mut end: (u32, u32);
                //println!("parsing line {}", line);
                let elems = line.split("->");
                for elem in elems {
                    let pair: Vec<_> = elem
                        .split(",")
                        .map(|x| {
                            x.trim()
                                .parse::<u32>()
                                .expect("non-numeric coordinate given!")
                        })
                        .collect();
                    assert_eq!(
                        pair.len(),
                        2,
                        "coordinates should be specified as pairs of numbers x,y!"
                    );
                    //println!("elem is {}/{}", pair[0], pair[1]);

                    if pair[0] > x_max {
                        x_max = pair[0];
                    }
                    if pair[0] < x_min {
                        x_min = pair[0];
                    }

                    if pair[1] > y_max {
                        y_max = pair[1];
                    }
                    if pair[1] < y_min {
                        y_min = pair[1];
                    }

                    match start {
                        None => {
                            start = Some((pair[0], pair[1]));
                        }
                        Some(start_loc) => {
                            end = (pair[0], pair[1]);
                            // line along y
                            if start_loc.0 == end.0 {
                                if start_loc.1 < end.1 {
                                    for y in start_loc.1..=end.1 {
                                        fill.insert((start_loc.0, y), Matter::Rock);
                                    }
                                } else {
                                    for y in end.1..=start_loc.1 {
                                        fill.insert((start_loc.0, y), Matter::Rock);
                                    }
                                }
                            }
                            // line along x
                            if start_loc.1 == end.1 {
                                if start_loc.0 < end.0 {
                                    for x in start_loc.0..=end.0 {
                                        fill.insert((x, start_loc.1), Matter::Rock);
                                    }
                                } else {
                                    for x in end.0..=start_loc.0 {
                                        fill.insert((x, start_loc.1), Matter::Rock);
                                    }
                                }
                            }
                            start = Some(end);
                        }
                    }
                }
            }

            if source_location.0 > x_max {
                x_max = source_location.0;
                eprintln!("source not above any rocks!")
            }
            if source_location.0 < x_min {
                x_min = source_location.0;
                eprintln!("source not above any rocks!")
            }
            if source_location.1 > y_max {
                y_max = source_location.1;
                eprintln!("source not above any rocks!")
            }
            if source_location.1 < y_min {
                y_min = source_location.1;
            }

            Canvas {
                x_min,
                x_max,
                y_min,
                y_max,
                source_x: source_location.0,
                source_y: source_location.1,
                fill,
            }
        }
    }
}
