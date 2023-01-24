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

    // Now that the name string has been mapped to
    // a u32, Valve can implement Copy!
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct Valve {
        pub name: u32,
        pub flow_rate: u32,
    }

    impl fmt::Display for Valve {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}/{}", self.name, self.flow_rate)
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
    use crate::valves::valve::Valve;
    use fs_err as fs;
    use hashbrown::{HashMap, HashSet};
    use itertools::Itertools;
    use petgraph::algo::astar;
    use petgraph::dot::Dot;
    use petgraph::graph::NodeIndex;
    use petgraph::Graph;
    use std::fmt;

    pub struct Network {
        graph: Graph<Valve, u32>,
        start_valve: Valve,
        to_open: HashSet<Valve>,
        distances_among_to_open: HashMap<(Valve, Valve), u32>,
        // let us cache subproblems of length 3;
        // store time taken to reach valve 2 and valve 3
        // store pressure released at valve 2 and valve 3
        cache_sols: HashMap<[Valve; 3], [u32; 4]>,
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
        pub fn pressure_release(&self) -> u32 {
            let max_pressure_release = self
                .to_open
                .iter()
                .permutations(self.to_open.len())
                .map(|order| {
                    /*println!("begin working on permutation");*/
                    // starting valve is always AA/0!
                    let mut start: Valve = self.start_valve;
                    let mut time_remaining: u32 = 30;
                    let mut pressure_released = 0;

                    for mut valve_pair in &order.into_iter().chunks(2) {
                        let valve_next = valve_pair.next().unwrap();

                        let possible_valve_last = valve_pair.next();

                        match possible_valve_last {
                            Some(valve_last) => {
                                let [time_used_1, time_used_2, pressure_released_1,pressure_released_2] = self
                                    .cache_sols
                                    .get(&[start, *valve_next, *valve_last])
                                    .expect("could not find sub-problem in cache!");
                                /*
                                println!(
                                    "{}/{}/{} | {}/{} | {}/{}",
                                    start.name,
                                    valve_next.name,
                                    valve_last.name,
                                    time_used_1, time_used_2,
                                    pressure_released_1, pressure_released_2
                                );*/

                                // extra 1 minute to open the valve!
                                match time_remaining.checked_sub(*time_used_1 + 1) {
                                    Some(time) => {time_remaining = time;
                                        pressure_released = pressure_released + pressure_released_1*time_remaining;
                                    },
                                    None => {
                                        println!(
                                            "pressure released in this permutation is {}",
                                            pressure_released
                                        );
                                        return pressure_released;
                                    }
                                };

                                // extra 1 minute to open the valve!
                                match time_remaining.checked_sub(*time_used_2 + 1) {
                                    Some(time) => {time_remaining = time;
                                        pressure_released = pressure_released + pressure_released_2*time_remaining;
                                    },
                                    None => {
                                        println!(
                                            "pressure released in this permutation is {}",
                                            pressure_released
                                        );
                                        return pressure_released;
                                    }
                                };


                                start = *valve_last;
                            }
                            None => {
                                let distance: u32 = *self
                                    .distances_among_to_open
                                    .get(&(start, *valve_next))
                                    .expect("could not find distance among valves");

                                // extra 1 minute to open the valve!
                                match time_remaining.checked_sub(distance + 1) {
                                    Some(time) => time_remaining = time,
                                    None => {
                                        println!(
                                            "pressure released in this permutation is {}",
                                            pressure_released
                                        );
                                        return pressure_released;
                                    }
                                };

                                let pressure_released_here = valve_next.flow_rate;
                                pressure_released =
                                    pressure_released + pressure_released_here * time_remaining;
                                /*println!(
                                    "releasing pressure {}; with {} minutes remaining",
                                    pressure_released_here, time_remaining
                                );*/
                                start = *valve_next;
                            }
                        };
                    }

                    println!(
                        "pressure released in this permutation is {}",
                        pressure_released
                    );
                    pressure_released
                })
                .max()
                .expect("error in finding max pressure release");

            max_pressure_release
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
                /*println!(
                    "valve:{}; flow_rate:{}",
                    this_valve.name, this_valve.flow_rate
                );*/

                if this_valve.name == convert_name("AA".to_string()) {
                    println!(
                        "valve to open found! name: {}; flow_rate: {}",
                        this_valve.name, this_valve.flow_rate
                    );
                    to_open.insert(this_valve.clone());
                    possible_start_valve = Some(this_valve);
                }
                if this_valve.flow_rate > 0 {
                    println!(
                        "valve to open found! name: {}; flow_rate: {}",
                        this_valve.name, this_valve.flow_rate
                    );
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

            // build a cache of triplets
            let mut cache_sols: HashMap<[Valve; 3], [u32; 4]> = HashMap::new();

            println!("building cache for starting triplets");
            // first iterate over all possible triplets with AA as the
            // starting valve
            to_open.iter().permutations(2).for_each(|valve_pair| {
                let mut start: Valve = start_valve;
                let mut time_remaining: u32 = 30;
                let mut pressure_released: [u32; 2] = [0, 0];
                let mut time_used: [u32; 2] = [0, 0];

                for i in 0..=1 {
                    let valve_to_open = valve_pair[i];

                    let distance: u32 = *distances_among_to_open
                        .get(&(start, *valve_to_open))
                        .expect("could not find distance among valves");

                    time_used[i] = distance;

                    // extra 1 minute to open the valve!
                    match time_remaining.checked_sub(distance + 1) {
                        Some(time) => time_remaining = time,
                        None => {
                            break;
                        }
                    };
                    pressure_released[i] = valve_to_open.flow_rate;
                    start = *valve_to_open;
                }

                cache_sols.insert(
                    [start_valve, *valve_pair[0], *valve_pair[1]],
                    [
                        time_used[0],
                        time_used[1],
                        pressure_released[0],
                        pressure_released[1],
                    ],
                );
            });

            println!("building cache for all possible triplets");
            // first iterate over all possible triplets without AA
            to_open.iter().permutations(3).for_each(|valve_triplet| {
                let mut start: Valve = *valve_triplet[0];
                let mut time_remaining: u32 = 30;
                let mut pressure_released: [u32; 2] = [0, 0];
                let mut time_used: [u32; 2] = [0, 0];

                for i in 1..=2 {
                    let valve_to_open = valve_triplet[i];
                    let distance: u32 = *distances_among_to_open
                        .get(&(start, *valve_to_open))
                        .expect("could not find distance among valves");
                    time_used[i - 1] = distance;

                    // extra 1 minute to open the valve!
                    match time_remaining.checked_sub(distance + 1) {
                        Some(time) => time_remaining = time,
                        None => {
                            break;
                        }
                    };
                    pressure_released[i - 1] = valve_to_open.flow_rate;
                    start = *valve_to_open;
                }

                cache_sols.insert(
                    [*valve_triplet[0], *valve_triplet[1], *valve_triplet[2]],
                    [
                        time_used[0],
                        time_used[1],
                        pressure_released[0],
                        pressure_released[1],
                    ],
                );
            });

            Some(Network {
                graph: valve_network,
                start_valve,
                to_open,
                distances_among_to_open,
                cache_sols,
            })
        }
    }
}
