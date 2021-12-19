extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2021::util::read_runfile_to_string("src/bin/day_14/input.txt")?;
    let (template, rules) = advent_2021::polymer::template_and_rules(&input)?;
    let template = template.step_n(&rules, 10)?;
    println!("Part 1: {}", template.score());

    let template = template.step_n(&rules, 40 - 10)?;
    println!("Part 2: {}", template.score());
    Ok(())
}
