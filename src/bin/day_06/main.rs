extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2021::util::read_runfile_to_string("src/bin/day_06/input.txt")?;
    let mut school = input.parse::<advent_2021::lanternfish::School>()?;
    school = school.simulate_n_days(80);
    println!("Part 1: {}", school.count());

    school = school.simulate_n_days(256 - 80);
    println!("Part 2: {}", school.count());
    Ok(())
}
