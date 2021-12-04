extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2021::util::open("src/bin/day_03/input.txt")?;
    let report = advent_2021::diag::parse_report(input)?;
    let power_consumption = advent_2021::diag::power_consumption(&report, 12);
    println!("Part 1: {}", power_consumption);

    let life_support_rating = advent_2021::diag::life_support_rating(&report)?;
    println!("Part 2: {}", life_support_rating);
    Ok(())
}
