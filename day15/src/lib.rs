use std::error::Error;
mod sensors;
use crate::sensors::sensor_beacon_pairs::Sensors;

pub fn run1(filename: String, y_loc: i64) -> Result<(), Box<dyn Error>> {
    let sensor_beacon_pairs = Sensors::new(filename);
    let num_locs: usize = sensor_beacon_pairs.get_empty_locs(y_loc);
    println!("num-locs is {}", num_locs);
    Ok(())
}

pub fn run2(filename: String, y_min: i64, y_max: i64) -> Result<(), Box<dyn Error>> {
    println!("given ymin/ymax : {}/{}", y_min, y_max);
    let sensor_beacon_pairs = Sensors::new(filename);
    let distress_beacon_loc = sensor_beacon_pairs
        .get_distress_beacon_loc(y_min, y_max)
        .expect("could not find distress beacon location!");
    println!(
        "distress beacon location is {}/{}",
        distress_beacon_loc.0, distress_beacon_loc.1
    );

    let tuning_freq = distress_beacon_loc
        .0
        .checked_mul(4000000)
        .expect("tuning freq overflow!")
        .checked_add(distress_beacon_loc.1)
        .expect("tuning freq overflow!");
    println!("tuning frequency is {}", tuning_freq);

    Ok(())
}
