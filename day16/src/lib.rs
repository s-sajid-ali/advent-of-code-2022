use std::error::Error;
mod valves;
use crate::valves::network::Network;

pub fn run1(filename: String) -> Result<(), Box<dyn Error>> {
    let valve_network = Network::new(filename);

    println!("{}", valve_network.unwrap());

    Ok(())
}
