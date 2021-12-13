extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2021::util::read_runfile_to_string("src/bin/day_13/input.txt")?;
    let (mut page, instructions) = advent_2021::origami::page_and_instructions(&input)?;
    page.follow_first(&instructions);
    println!("Part 1: {}", page.dot_count());

    let (mut page, instructions) = advent_2021::origami::page_and_instructions(&input)?;
    page.follow_all(&instructions);
    println!("Part 2:\n{}", page);
    Ok(())
}
