extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2021::util::open("src/bin/day_10/input.txt")?;
    let lines = advent_2021::syntax::Lines::parse(input)?;
    println!("Part 1: {}", lines.syntax_error_score());

    println!("Part 2: {}", lines.middle_completion_score());
    Ok(())
}
