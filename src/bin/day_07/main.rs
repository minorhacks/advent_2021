use std::io::Read;

extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = advent_2021::util::open("src/bin/day_07/input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    let positions = input.parse::<advent_2021::crab::Positions>()?;
    println!("Part 1: {}", positions.cheapest_alignment_cost());

    println!("Part 2: {}", positions.cheapest_alignment_cost_nonlinear());
    Ok(())
}
