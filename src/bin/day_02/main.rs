extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2021::util::open("src/bin/day_02/input.txt")?;
    let command_list = advent_2021::dive::command_list(input)?;
    let mut pos = advent_2021::dive::Position::new();
    pos.follow(&command_list);
    println!("Part 1: {}", pos.checksum());

    let mut pos = advent_2021::dive::AimPosition::new();
    pos.follow(&command_list);
    println!("Part 2: {}", pos.checksum());
    Ok(())
}
