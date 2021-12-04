extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = advent_2021::util::open("src/bin/day_04/input.txt")?;
    let mut game = advent_2021::bingo::Game::load(input)?;
    let (last_num, winning_score) = game.play()?;
    println!("Part 1: {}", last_num as i32 * winning_score);

    Ok(())
}
