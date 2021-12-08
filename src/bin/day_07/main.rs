extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2021::util::read_runfile_to_string("src/bin/day_07/input.txt")?;
    let positions = input.parse::<advent_2021::crab::Positions>()?;
    println!("Part 1: {}", positions.cheapest_alignment_cost());

    println!("Part 2: {}", positions.cheapest_alignment_cost_nonlinear());
    Ok(())
}
