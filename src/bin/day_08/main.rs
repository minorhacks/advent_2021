extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2021::util::open("src/bin/day_08/input.txt")?;
    let entries = advent_2021::seven_segment::Entries::parse(input)?;
    println!("Part 1: {}", entries.output_unique_digit_count());

    println!("Part 2: {}", entries.output_sum()?);
    Ok(())
}
