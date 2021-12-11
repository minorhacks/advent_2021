extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2021::util::read_runfile_to_string("src/bin/day_09/input.txt")?;
    let height_map = input.parse::<advent_2021::lava_tube::HeightMap>()?;
    println!("Part 1: {}", height_map.risk_level_sum());

    println!("Part 2: {}", height_map.largest_basins_product(3));
    Ok(())
}
