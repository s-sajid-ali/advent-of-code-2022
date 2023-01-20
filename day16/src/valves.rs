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
        pub fn pressure_release(&self) -> u32 {
            // let us cache subproblems of length 3; store pressure released and time taken
            //let cache_sols: HashMap<[u32; 3], [u32; 2]> = HashMap::new();

            let max_pressure_release = self
                .to_open
                .iter()
                .permutations(self.to_open.len())
                .map(|order| {
                    /*println!("begin working on permutation");*/
                    // starting valve is always AA/0!
                    let mut start: Valve = Valve {
                        name: 1010,
                        flow_rate: 0,
                    };
                    let mut time_remaining: u32 = 30;
                    let mut pressure_released = 0;

                    for valve_to_open in order {
                        let distance: u32 = *self
                            .distances_among_to_open
                            .get(&(start, *valve_to_open))
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

                        let pressure_released_here = valve_to_open.flow_rate;
                        pressure_released =
                            pressure_released + pressure_released_here * time_remaining;
                        /*println!(
                            "releasing pressure {}; with {} minutes remaining",
                            pressure_released_here, time_remaining
                        );*/
                        start = *valve_to_open;
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
                if this_valve.flow_rate > 0 || this_valve.name == convert_name("AA".to_string()) {
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

            to_open.remove(&Valve {
                name: convert_name("AA".to_string()),
                flow_rate: 0,
            });

            Some(Network {
                graph: valve_network,
                to_open,
                distances_among_to_open,
            })
        }
    }
}
