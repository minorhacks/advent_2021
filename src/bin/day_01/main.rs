use advent_2021::sonar::num_increases_windowed;

extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2021::util::open("src/bin/day_01/input.txt")?;
    let depth_list = advent_2021::sonar::depth_list(input)?;
    let num_increases = advent_2021::sonar::num_depth_increases(&depth_list);
    println!("Part 1: {}", num_increases);

    let num_windowed = num_increases_windowed(&depth_list, 3);
    println!("Part 2: {}", num_windowed);
    Ok(())
}
