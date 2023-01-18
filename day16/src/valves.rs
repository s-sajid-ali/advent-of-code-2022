pub mod valve {
    use std::fmt;

    #[derive(Clone, PartialEq)]
    pub struct Valve {
        pub name: String,
        pub flow_rate: u32,
    }

    impl fmt::Display for Valve {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}/{}", self.name, self.flow_rate)
        }
    }
}

mod parse {

    use crate::valves::valve::Valve;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::bytes::complete::take_till1;
    use nom::character::complete::u32;
    use nom::character::is_newline;
    use nom::multi::many1;
    use nom::sequence::preceded;
    use nom::sequence::tuple;
    use nom::{Finish, IResult};

    fn parser_flow_rate(s: &str) -> IResult<&str, u32> {
        preceded(tag(" has flow rate="), u32)(s)
    }

    fn parser_valve_comma(s: &str) -> IResult<&str, &str> {
        preceded(tag(" "), take_till1(|c| c == ','))(s)
    }

    fn parser_valve_space(s: &str) -> IResult<&str, &str> {
        preceded(tag(" "), take_till1(|c| c == ' '))(s)
    }

    fn parser_valve_last(s: &str) -> IResult<&str, &str> {
        preceded(tag(" "), take_till1(|c| is_newline(c as u8)))(s)
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
            many1(alt((
                //parser_valve_space,
                parser_valve_comma,
                parser_valve_last,
            ))),
        )(s)
    }

    pub fn parse_input(s: &str) -> Option<(Valve, Vec<&str>)> {
        match tuple((parser_valve_first, parser_flow_rate, parser_valves))(s).finish() {
            Ok((_, (valve, flow_rate, tunnels))) => {
                let this_valve = Valve {
                    name: valve.to_string(),
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
    use crate::valves::valve::Valve;
    use fs_err as fs;
    use petgraph::graph::NodeIndex;
    //use petgraph::algo::astar;
    use petgraph::dot::Dot;
    use petgraph::Graph;
    use std::collections::HashSet;
    use std::fmt;

    pub struct Network {
        graph: Graph<Valve, u32>,
        to_open: HashSet<String>,
    }

    impl fmt::Display for Network {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "valve network is {}",
                //                self.to_open.clone().into_iter().collect::<String>()
                Dot::new(&self.graph)
            )
        }
    }

    impl Network {
        pub fn new(filename: String) -> Option<Network> {
            let mut valve_network = Graph::<Valve, u32>::new();
            let mut to_open: HashSet<String> = HashSet::new();

            // traverse through the file, adding nodes with a default weight(flow_rate) of 0
            // until the true flow_rate is found
            let contents = fs::read_to_string(filename).expect("input file missing!");
            let component_lines = contents.lines().collect::<Vec<_>>();
            for line in component_lines {
                println!("------------------------");
                let (this_valve, tunnels) = parse_input(line).expect("could not parse input!");
                println!(
                    "valve:{}; flow_rate:{}",
                    this_valve.name, this_valve.flow_rate
                );
                if this_valve.flow_rate > 0 {
                    to_open.insert(this_valve.name.to_string());
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
                println!(
                    "this valvel nidx is : {}",
                    this_valve_nidx.expect("nidx is empty!").index()
                );

                //dbg!(tunnels);

                tunnels.iter().for_each(|tunnel_valve| {
                    println!("adding connection to {};", tunnel_valve);
                    let this_tunnel_valve = Valve {
                        name: tunnel_valve.to_string(),
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

            Some(Network {
                graph: valve_network,
                to_open,
            })
        }
    }
}
