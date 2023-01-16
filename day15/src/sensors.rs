mod parse {

    use nom::bytes::complete::tag;
    use nom::character::complete::i64;
    use nom::sequence::preceded;
    use nom::sequence::separated_pair;
    use nom::IResult;

    fn parserx(s: &str) -> IResult<&str, i64> {
        preceded(tag("x="), i64)(s)
    }

    fn parsery(s: &str) -> IResult<&str, i64> {
        preceded(tag("y="), i64)(s)
    }

    fn parser_basic(s: &str) -> IResult<&str, (i64, i64)> {
        separated_pair(parserx, tag(", "), parsery)(s)
    }

    fn parser_sensor(s: &str) -> IResult<&str, (i64, i64)> {
        preceded(tag("Sensor at "), parser_basic)(s)
    }

    fn parser_beacon(s: &str) -> IResult<&str, (i64, i64)> {
        preceded(tag("closest beacon is at "), parser_basic)(s)
    }

    pub fn parser(s: &str) -> ((i64, i64), (i64, i64)) {
        let (_, (sensor_loc, beacon_loc)) =
            separated_pair(parser_sensor, tag(": "), parser_beacon)(s)
                .expect("could not parse sensor/beacon pair!");

        (sensor_loc, beacon_loc)
    }
}

pub mod sensor_beacon_pairs {

    use crate::sensors::parse::parser;
    use fs_err as fs;
    use hashbrown::HashMap;
    use indicatif::ProgressBar;
    use rayon::prelude::*;
    use std::collections::HashSet;

    pub struct Sensors {
        pairs: HashMap<(i64, i64), (i64, i64)>,
    }

    impl Sensors {
        pub fn new(filename: String) -> Sensors {
            let contents = fs::read_to_string(filename).expect("input file missing!");
            let component_lines = contents.lines().collect::<Vec<_>>();
            let mut sensor_beacon_pairs: HashMap<(i64, i64), (i64, i64)> = HashMap::new();

            for line in component_lines {
                let (sensor_loc, beacon_loc) = parser(line);
                sensor_beacon_pairs.insert(sensor_loc, beacon_loc);
            }

            Sensors {
                pairs: sensor_beacon_pairs,
            }
        }

        pub fn get_distress_beacon_loc(&self, ymin: i64, ymax: i64) -> Option<(i64, i64)> {
            let bar_size: u64 = (ymax + 1 - ymin)
                .try_into()
                .expect("progress bar size calc. failed");
            //let bar = ProgressBar::new(bar_size);

            let result = (ymin..=ymax).into_par_iter().find_map_any(|yloc| {
                let mut locs: Vec<bool> =
                    vec![true; bar_size.try_into().expect("u64->usize overflow")];

                for (sensor, beacon) in self.pairs.iter() {
                    /*
                    let idx = locs.iter().position(|elem| *elem == true);
                    if idx.is_none() {
                        continue;
                    }*/

                    let dist_x: u64 = u64::try_from((sensor.0).abs_diff(beacon.0)).expect(
                        "failed to convert distance between sensor/beacon on x-axis to u64",
                    );
                    let dist_y: u64 = u64::try_from((sensor.1).abs_diff(beacon.1)).expect(
                        "failed to convert distance between sensor/beacon on y-axis to u64",
                    );
                    let dist = dist_x
                        .checked_add(dist_y)
                        .expect("distance computation overflow!");
                    let test: u64 = u64::try_from((sensor.1).abs_diff(yloc))
                        .expect("failed to convert distance between sensor/yloc on y-axis to u64");

                    if test < dist {
                        let x_loc: u64 = dist - test;
                        let loc1_i64: i64 = (sensor.0)
                            .checked_sub(
                                x_loc
                                    .try_into()
                                    .expect("i64->u64 overflow when computing x-locs!"),
                            )
                            .expect("cannot convert potential loc to i64");
                        let loc1: usize = if loc1_i64 < 0 {
                            0_usize
                        } else {
                            usize::try_from(loc1_i64)
                                .expect("i64->usize overflow when computing index")
                        };
                        let loc2_i64: i64 = (sensor.0)
                            .checked_add(
                                x_loc
                                    .try_into()
                                    .expect("i64->u64 overflow when computing x-locs!"),
                            )
                            .expect("cannot convert potential loc to i64");
                        let loc2: usize = if loc2_i64 > ymax {
                            usize::try_from(ymax).expect("ymax cannot be converted to usize!")
                        } else {
                            usize::try_from(loc2_i64)
                                .expect("loc2_i64 cannot be converted to usize!")
                        };

                        let slice = &mut locs[loc1..(loc2 + 1)];
                        slice.fill(false);
                    }
                }

                match locs.iter().position(|elem| *elem == true) {
                    None => None,
                    Some(val_idx) => {
                        println!("val_idx is {}", val_idx);
                        let tmp: i64 = i64::try_from(val_idx).expect("result does not fit in i64!");
                        Some((tmp, yloc))
                    }
                }
            });

            result
        }

        pub fn get_empty_locs(&self, yloc: i64) -> usize {
            let mut locs: HashSet<i64> = HashSet::new();

            for (sensor, beacon) in self.pairs.iter() {
                println!("-------------------------------");
                println!(
                    "procesing sensor/beacon pair: ({},{})/({},{})",
                    sensor.0, sensor.1, beacon.0, beacon.1
                );

                let dist_x: u64 = u64::try_from((sensor.0).abs_diff(beacon.0))
                    .expect("failed to convert distance between sensor/beacon on x-axis to u64");
                let dist_y: u64 = u64::try_from((sensor.1).abs_diff(beacon.1))
                    .expect("failed to convert distance between sensor/beacon on y-axis to u64");

                let dist = dist_x
                    .checked_add(dist_y)
                    .expect("distance computation overflow!");

                let test: u64 = u64::try_from((sensor.1).abs_diff(yloc))
                    .expect("failed to convert distance between sensor/yloc on y-axis to u64");

                if test < dist {
                    let x_locs: u64 = dist - test;
                    for x in 0..=x_locs {
                        let x1: i64 = x
                            .try_into()
                            .expect("cannot convert potential x-location to i64");
                        let loc1 = (sensor.0)
                            .checked_add(x1)
                            .expect("cannot convert potential loc to i64");
                        let _ = locs.insert(loc1);
                        let loc2 = (sensor.0)
                            .checked_sub(x1)
                            .expect("cannot convert potential loc to i64");
                        let _ = locs.insert(loc2);
                        //println!("adding locs {}/{}", loc1, loc2);
                    }
                } else {
                    println!(
                        "distance to y-loc from sensor {} is larger than distance to beacon {}!",
                        test, dist
                    );
                }
            }

            // remove any beacon locations form locs
            for (_, beacon) in self.pairs.iter() {
                if beacon.1 == yloc {
                    let is_present = locs.get(&beacon.0);
                    match is_present {
                        None => {}
                        Some(_) => {
                            locs.remove(&beacon.0);
                        }
                    }
                }
            }

            println!("-------------------------------");

            /*
            let mut locs_vec = locs.into_iter().collect::<Vec<_>>();
            locs_vec.sort();
            dbg!(&locs_vec);*/
            locs.len()
        }
    }
}
