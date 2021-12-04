use std::io::Read;

extern crate advent_2021;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = advent_2021::util::open("src/bin/day_04/input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    let mut game = advent_2021::bingo::Game::load(&input)?;
    let (last_num, winning_score) = game.first_winning_board()?;
    println!("Part 1: {}", last_num as i32 * winning_score);

    let mut game = advent_2021::bingo::Game::load(&input)?;
    let (last_num, winning_score) = game.last_winning_board()?;
    println!("Part 2: {}", last_num as i32 * winning_score);

    Ok(())
}
