use std::error::Error;
mod reservoir;
use crate::reservoir::reservoir::Canvas;

pub fn run1(filename: String, source_location: (u32, u32)) -> Result<(), Box<dyn Error>> {
    let mut canvas = Canvas::new(filename, source_location);
    println!("canvas is {}", canvas);
    canvas.render();

    canvas.fill_sand();
    println!("canvas after fill is {}", canvas);
    canvas.render();

    Ok(())
}

pub fn run2(filename: String, source_location: (u32, u32)) -> Result<(), Box<dyn Error>> {
    let mut canvas = Canvas::new(filename, source_location);
    println!("canvas is {}", canvas);
    canvas.render();

    canvas.fill_sand_infinite();
    println!("canvas after fill is {}", canvas);
    canvas.render();

    Ok(())
}
