extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2021::util::read_runfile_to_string("src/bin/day_11/input.txt")?;
    let grid = input.parse::<advent_2021::octopus::OctopusGrid>()?;
    println!("Part 1: {}", grid.step_n(100).flash_count());

    let grid = input.parse::<advent_2021::octopus::OctopusGrid>()?;
    println!("Part 2: {}", grid.first_synchronized_flash());
    Ok(())
}
