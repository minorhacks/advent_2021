use std::error::Error;

extern crate advent_2021;

use advent_2021::hello_world;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Test output: {}", hello_world::hello_world());
    Ok(())
}
