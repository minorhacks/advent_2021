extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2021::util::read_runfile_to_string("src/bin/day_12/input.txt")?;
    let system = input.parse::<advent_2021::cave::System>()?;
    println!(
        "Part 1: {}",
        system.count_paths_small_cave_once("start", "end")
    );

    println!(
        "Part 2: {}",
        system.count_paths_one_small_cave_twice("start", "end")
    );
    Ok(())
}
