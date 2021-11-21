extern crate advent_2018;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2018::util::open("src/bin/day_02/input.txt")?;
    let ids = advent_2018::boxes::read_ids(input)?;
    let num_with_2 = ids.iter().filter(|id| id.has_exactly_n(2)).count();
    let num_with_3 = ids.iter().filter(|id| id.has_exactly_n(3)).count();
    println!("Part 1: {}", num_with_2 * num_with_3);
    Ok(())
}
