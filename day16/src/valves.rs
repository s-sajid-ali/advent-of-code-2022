pub mod valve {
    use std::fmt;

    // convert a string of 2 characters to a u32 number
    // AA -> 10*100+10 -> 1010
    // EK -> 14*100+20 -> 1420
    pub fn convert_name(name_in: String) -> u32 {
        let mut val: u32 = 0;
        let mut factor: i32 = 100;
        for y in name_in.chars() {
            if factor < 1 {
                eprintln!("input is greater than 2 chars!");
            }
            val = val + u32::try_from(factor).unwrap() * y.to_digit(36).unwrap();
            factor = factor / 100;
        }
        val
    }

    // convert a u32 number to valve name
    pub fn convert_num(num_in: u32) -> String {
        let digi1: u32 = num_in % 100;
        let digi2: u32 = num_in / 100;

        let mut s = String::with_capacity(2);
        s.push(char::from_digit(digi2, 36).unwrap().to_ascii_uppercase());
        s.push(char::from_digit(digi1, 36).unwrap().to_ascii_uppercase());
        s
    }

    // Now that the name string has been mapped to
    // a u32, Valve can implement Copy!
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Valve {
        pub name: u32,
        pub flow_rate: u32,
    }

    impl fmt::Display for Valve {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let fist_part: u32 = self.name / 100;
            let second_part: u32 = self.name - fist_part * 100;
            let first_char = char::from_digit(fist_part, 36)
                .expect("could not convert digit to alphabet!")
                .to_uppercase();
            let second_char = char::from_digit(second_part, 36)
                .expect("could not convert digit to alphabet!")
                .to_uppercase();
            write!(
                f,
                "{}{}/{}{}/{}",
                fist_part, second_part, first_char, second_char, self.flow_rate
            )
        }
    }
}

mod parse {

    use crate::valves::valve::convert_name;
    use crate::valves::valve::Valve;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::bytes::complete::take_till;
    use nom::character::complete::u32;
    use nom::character::is_newline;
    use nom::multi::many1;
    use nom::sequence::preceded;
    use nom::sequence::tuple;
    use nom::{Finish, IResult};

    fn parser_flow_rate(s: &str) -> IResult<&str, u32> {
        preceded(tag(" has flow rate="), u32)(s)
    }

    // this does not work as expected in many1. Why?
    /* fn parser_valve_comma(s: &str) -> IResult<&str, &str> {
        preceded(tag(" "), take_till(|c| c == ','))(s)
    }*/

    fn parser_valve_space(s: &str) -> IResult<&str, &str> {
        preceded(tag(" "), take_till(|c| c == ' '))(s)
    }

    fn parser_valve_last(s: &str) -> IResult<&str, &str> {
        preceded(tag(" "), take_till(|c| is_newline(c as u8)))(s)
    }

    fn parser_valve_first(s: &str) -> IResult<&str, &str> {
        preceded(tag("Valve"), parser_valve_space)(s)
    }

    fn parser_valves(s: &str) -> IResult<&str, Vec<&str>> {
        preceded(
            alt((
                tag("; tunnels lead to valves"),
                tag("; tunnel leads to valve"),
            )),
            // use separated_list1 in the future instead
            many1(alt((parser_valve_space, parser_valve_last))),
        )(s)
    }

    pub fn parse_input(s: &str) -> Option<(Valve, Vec<&str>)> {
        match tuple((parser_valve_first, parser_flow_rate, parser_valves))(s).finish() {
            Ok((_, (valve, flow_rate, tunnels))) => {
                let this_valve = Valve {
                    name: convert_name(valve.to_string()),
                    flow_rate,
                };

                return Some((this_valve, tunnels));
            }
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        }
    }
}

pub mod network {

    use crate::valves::parse::parse_input;
    use crate::valves::valve::convert_name;
    use crate::valves::valve::convert_num;
    use crate::valves::valve::Valve;
    use bit_set::BitSet;
    use fs_err as fs;
    use itertools::Itertools;
    use petgraph::algo::astar;
    use petgraph::dot::Dot;
    use petgraph::graph::NodeIndex;
    use petgraph::Graph;
    use std::collections::{HashMap, HashSet};
    use std::fmt;

    #[derive(Eq, Hash, PartialEq)]
    pub struct Path {
        elems: BitSet,
        start_valve: Valve,
    }

    pub struct Result {
        next: Valve,
        cost: u32,
    }

    pub struct Network {
        graph: Graph<Valve, u32>,
        start_valve: Valve,
        to_open: HashSet<Valve>,
        distances_among_to_open: HashMap<(Valve, Valve), u32>,
    }

    impl fmt::Display for Network {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "--------------------------
valve network is {}-----------------------------
number of active valves is {}
-----------------------------",
                Dot::new(&self.graph),
                self.to_open.len()
            )
        }
    }

    impl Network {
        fn get_valve_from_name(&self, name_in: u32) -> Valve {
            let res = self.to_open.iter().find(|x| x.name == name_in);

            match res {
                None => panic!("requested valve not in list of valves to open!"),
                Some(res_valve) => return *res_valve,
            }
        }

        //https://en.wikipedia.org/wiki/Held%E2%80%93Karp_algorithm#Example[3]
        //
        // re-interpret the problem of finding the opening order of valves
        // as a TSP. For example, given valves A,{B,C,D}, with A->B->C->D being
        // the best order, we have the final pressure release as:
        // 30 * A + (30 - d'AB) * B + (30 - d'AB - d'BC) * C +
        // (30 - d'AB -d'BC - d'CD) * D
        // = 30 * (A + B + C + D) - ( d'AB * (B + C + D)
        //                            + d'BC * (C + D)
        //                            + d'CD * D
        //                          )
        // where d'AB is just dAB + 1.
        // and to determine the above, we could have minized the function
        // min{B, C, D} =  min {
        // (path1) B + 2C + 3D + dBC * (C + D) + dCD * D
        // (path2) B + 2D + 3C + dBD * (C + D) + dCD * C
        // ....
        pub fn pressure_release(&self) -> u32 {
            // cache the best travelsal path given a set of valves
            // to traverse and a starting valve
            let mut cache: HashMap<Path, Result> = HashMap::new();

            for length in 2..(self.to_open.len() + 1) {
                println!("solving for problem of length: {}", length);

                let iter_comb = self.to_open.iter().combinations(length);

                iter_comb.for_each(|combination| {
                    let mut all_valves_in_combination = BitSet::with_capacity(length);
                    combination.iter().for_each(|valve| {
                        all_valves_in_combination.insert(valve.to_owned().name as usize);
                    });

                    combination.iter().for_each(|start_valve| {
                        let mut traversal_valves = all_valves_in_combination.clone();
                        traversal_valves.remove(start_valve.name as usize);

                        let path: Path = Path {
                            elems: traversal_valves.clone(),
                            start_valve: **start_valve,
                        };

                        let next = traversal_valves
                            .clone()
                            .iter()
                            .min_by_key(|test_val| {
                                let test_valve: Valve = *(self
                                    .to_open
                                    .iter()
                                    .find(|&&x| x.name == *test_val as u32)
                                    .expect("missing valve"));

                                let next_dist = self
                                    .distances_among_to_open
                                    .get(&(**start_valve, test_valve))
                                    .expect("missing distance!");

                                let mut remaining_traversal_valves = traversal_valves.clone();
                                remaining_traversal_valves.remove(*test_val);
                                let sum_rel: u32 = traversal_valves.iter().fold(0, |acc, tmp| {
                                    let val_test: Valve = *(self
                                        .to_open
                                        .iter()
                                        .find(|&&x| x.name == tmp as u32)
                                        .expect("missing valve"));
                                    acc + val_test.flow_rate
                                });
                                let remaining_path: Path = Path {
                                    elems: remaining_traversal_valves,
                                    start_valve: test_valve,
                                };
                                if length == 2 {
                                    (next_dist + 1) * sum_rel
                                } else {
                                    let res: &Result =
                                        cache.get(&(remaining_path)).expect("cache missing path!");
                                    ((next_dist + 1) * sum_rel) + res.cost
                                }
                            })
                            .expect("cannot find next valve to traverse to!");

                        let next_valve: Valve = *(self
                            .to_open
                            .iter()
                            .find(|&&x| x.name == next as u32)
                            .expect("missing valve"));
                        let next_dist = self
                            .distances_among_to_open
                            .get(&(**start_valve, next_valve))
                            .expect("missing distance!");

                        let sum_rel: u32 = traversal_valves.iter().fold(0, |acc, tmp| {
                            let val_test: Valve = *(self
                                .to_open
                                .iter()
                                .find(|&&x| x.name == tmp as u32)
                                .expect("missing valve"));

                            acc + val_test.flow_rate
                        });

                        let mut remaining_traversal_valves = traversal_valves.clone();
                        remaining_traversal_valves.remove(next_valve.name as usize);

                        let remaining_path: Path = Path {
                            elems: remaining_traversal_valves,
                            start_valve: next_valve,
                        };

                        if length == 2 {
                            let next_res: Result = Result {
                                next: next_valve,
                                cost: ((*next_dist + 1) * sum_rel),
                            };
                            cache.insert(path, next_res);
                        } else {
                            let res: &Result =
                                cache.get(&(remaining_path)).expect("cache missing path!");
                            let next_res: Result = Result {
                                next: next_valve,
                                cost: ((*next_dist + 1) * sum_rel) + res.cost,
                            };
                            cache.insert(path, next_res);
                        }
                    });
                });
            }

            let mut valves_to_open = BitSet::with_capacity(self.to_open.len());
            self.to_open.iter().for_each(|valve| {
                valves_to_open.insert(valve.to_owned().name as usize);
            });

            let sum_rel: u32 = valves_to_open.iter().fold(0, |acc, tmp| {
                let val_test: Valve = *(self
                    .to_open
                    .iter()
                    .find(|&&x| x.name == tmp as u32)
                    .expect("missing valve"));
                acc + val_test.flow_rate
            });
            println!("sum-rel is {}", sum_rel);

            let next = valves_to_open
                .clone()
                .iter()
                .min_by_key(|test_val| {
                    let test_valve: Valve = *(self
                        .to_open
                        .iter()
                        .find(|&&x| x.name == *test_val as u32)
                        .expect("missing valve"));
                    let next_dist = self
                        .distances_among_to_open
                        .get(&(self.start_valve, test_valve))
                        .expect("missing distance!");
                    let mut remaining_traversal_valves = valves_to_open.clone();
                    remaining_traversal_valves.remove(*test_val);

                    let remaining_path: Path = Path {
                        elems: remaining_traversal_valves.clone(),
                        start_valve: test_valve,
                    };

                    let res: &Result = cache.get(&(remaining_path)).expect("cache missing path!");

                    let dist = ((*next_dist + 1) * sum_rel) + res.cost;

                    println!(
                        "next-dist for test-valve {} is {}, dist is {}, res-cost is {}",
                        convert_num(test_valve.name),
                        next_dist,
                        dist,
                        res.cost
                    );

                    dist
                })
                .expect("cannot find next valve to traverse to!");

            let next_valve: Valve = *(self
                .to_open
                .iter()
                .find(|&&x| x.name == next as u32)
                .expect("missing valve"));

            let next_dist = self
                .distances_among_to_open
                .get(&(self.start_valve, next_valve))
                .expect("missing distance!");

            let mut press_release = 0;
            let mut time = 30;

            let mut path: Path = Path {
                elems: valves_to_open.clone(),
                start_valve: self.start_valve,
            };

            print!(
                "path status: start_valve is {}, time is {}, pressure released is {}, remaining valves are:", 
                convert_num(self.start_valve.name), time, press_release
            );
            for elem in &path.elems {
                print!(" {},", convert_num(elem as u32));
            }
            print!("\n");

            time = time - 1 - next_dist;

            press_release = press_release + time * next_valve.flow_rate;

            // reset search path
            path.elems.remove(next);
            path.start_valve = next_valve;

            print!(
                "path status: start_valve is {}, time is {}, pressure released is {}, remaining valves are:", 
                convert_num(path.start_valve.name), time, press_release
            );
            for elem in &path.elems {
                print!(" {},", convert_num(elem as u32));
            }
            print!("\n");

            while path.elems.len() > 1 {
                let res = cache.get(&(path)).expect("missing path from cache!");

                let next_dist = self
                    .distances_among_to_open
                    .get(&(path.start_valve, res.next))
                    .expect("missing distance!");

                let test = time.checked_sub(next_dist + 1);
                match test {
                    Some(nonnegtime) => {
                        time = nonnegtime;
                        press_release = press_release + time * res.next.flow_rate;
                    }
                    None => {
                        return press_release;
                    }
                }

                path.elems.remove(res.next.name as usize);
                path.start_valve = res.next;

                print!(
                "path status: start_valve is {}, time is {}, pressure released is {}, remaining valves are:", 
                convert_num(path.start_valve.name), time, press_release
                );
                for elem in &path.elems {
                    print!(" {},", convert_num(elem as u32));
                }
                print!("\n");
            }

            assert_eq!(path.elems.len(), 1);

            let bitvec = path.elems.iter().collect_vec();
            let lastval = bitvec
                .get(0)
                .expect("empty bit vec found, when expecting 2 elements!");

            let lastvalve: Valve = *(self
                .to_open
                .iter()
                .find(|&&x| x.name == *lastval as u32)
                .expect("missing valve"));

            let lastdist = self
                .distances_among_to_open
                .get(&(path.start_valve, lastvalve))
                .expect("missing distance!");

            let test = time.checked_sub(lastdist + 1);
            match test {
                Some(nonnegtime) => {
                    time = nonnegtime;
                    press_release = press_release + time * lastvalve.flow_rate;
                }
                None => {
                    return press_release;
                }
            }

            press_release
        }

        pub fn new(filename: String) -> Option<Network> {
            let mut valve_network = Graph::<Valve, u32>::new();
            let mut to_open: HashSet<Valve> = HashSet::new();
            let mut distances_among_to_open: HashMap<(Valve, Valve), u32> = HashMap::new();
            let mut possible_start_valve: Option<Valve> = None;

            // traverse through the file, adding nodes with a default weight(flow_rate) of 0
            // until the true flow_rate is found
            let contents = fs::read_to_string(filename).expect("input file missing!");
            let component_lines = contents.lines().collect::<Vec<_>>();
            for line in component_lines {
                //println!("------------------------");
                let (this_valve, tunnels) = parse_input(line).expect("could not parse input!");
                /*println!( "valve:{}; flow_rate:{}", this_valve.name, this_valve.flow_rate);*/

                if this_valve.name == convert_name("AA".to_string()) {
                    println!("valve to open found! details: {}", this_valve);
                    to_open.insert(this_valve.clone());
                    possible_start_valve = Some(this_valve);
                }
                if this_valve.flow_rate > 0 {
                    println!("valve to open found! details: {}", this_valve);
                    to_open.insert(this_valve.clone());
                }
                let mut this_valve_nidx: Option<NodeIndex> = None;
                match valve_network
                    .node_indices()
                    .position(|x| valve_network[x].name == this_valve.name)
                {
                    None => this_valve_nidx = Some(valve_network.add_node(this_valve)),
                    Some(idx) => {
                        this_valve_nidx = Some(NodeIndex::new(idx));
                        let weight = valve_network
                            .node_weight_mut(this_valve_nidx.unwrap())
                            .expect("missing node!");
                        *weight = this_valve;
                    }
                }

                tunnels.iter().for_each(|tunnel_valve| {
                    //println!("adding connection to {};", tunnel_valve);
                    let this_tunnel_valve = Valve {
                        name: convert_name(tunnel_valve.to_string().replace(",", "")),
                        flow_rate: 0,
                    };
                    let mut this_tunnel_valve_nidx: Option<NodeIndex> = None;
                    match valve_network
                        .node_indices()
                        .position(|x| valve_network[x].name == this_tunnel_valve.name)
                    {
                        None => {
                            this_tunnel_valve_nidx = Some(valve_network.add_node(this_tunnel_valve))
                        }
                        Some(idx) => {
                            this_tunnel_valve_nidx = Some(NodeIndex::new(idx));
                        }
                    }
                    _ = valve_network.add_edge(
                        this_valve_nidx.expect("current node index is empty!"),
                        this_tunnel_valve_nidx.expect("tunnel node index is empty"),
                        1,
                    );
                });
            }

            to_open.iter().permutations(2).for_each(|valve_pair| {
                if distances_among_to_open
                    .contains_key(&(valve_pair[0].clone(), valve_pair[1].clone()))
                {
                    return;
                }

                let start_idx = valve_network
                    .node_indices()
                    .position(|x| valve_network[x].name == valve_pair[0].name)
                    .expect("end loc not in network!");
                let end_idx = valve_network
                    .node_indices()
                    .position(|x| valve_network[x].name == valve_pair[1].name)
                    .expect("end loc not in network!");

                let (distance, _) = astar(
                    &valve_network,
                    NodeIndex::new(start_idx),
                    |finish| finish == NodeIndex::new(end_idx),
                    |_| 1,
                    |_| 0,
                )
                .expect("could not find a path between start and end locations!");

                distances_among_to_open.insert(
                    (valve_pair[0].to_owned(), valve_pair[1].to_owned()),
                    distance,
                );
            });

            let start_valve = possible_start_valve.expect("missing starting valve (AA)!");

            // now remove AA from the to_open set
            to_open.remove(&start_valve);

            Some(Network {
                graph: valve_network,
                start_valve,
                to_open,
                distances_among_to_open,
            })
        }
    }
}
