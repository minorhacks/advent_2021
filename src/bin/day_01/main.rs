extern crate advent_2018;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2018::util::open("src/bin/day_01/input.txt")?;
    let mut dupes = advent_2018::device::FrequencyDuplicates::new(input)?;
    println!("Part 1: {}", dupes.drift_sum());
    println!("Part 2: {}", dupes.next().unwrap());
    Ok(())
}
