use std::error::Error;
mod valves;
use crate::valves::network::Network;

pub fn run1(filename: String) -> Result<(), Box<dyn Error>> {
    let valve_network =
        Network::new(filename).expect("could not create valve network from input file!");
    println!("{}", valve_network);

    let pressure_release = valve_network.pressure_release();
    println!("max possible pressure release is {}", pressure_release);

    Ok(())
}
