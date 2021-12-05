extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = advent_2021::util::open("src/bin/day_05/input.txt")?;
    let points = advent_2021::hydrothermal::read_points(f)?;
    let counts = advent_2021::hydrothermal::PointCounts::count_horizontal_vertical(&points);
    println!("Part 1: {}", counts.overlap_count());

    let counts = advent_2021::hydrothermal::PointCounts::count_all(&points);
    println!("Part 2: {}", counts.overlap_count());
    Ok(())
}
