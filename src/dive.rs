use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use std::io;

use crate::util;

pub enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

type CommandList = Vec<Command>;

pub fn command_list<T: io::BufRead>(r: T) -> Result<CommandList> {
    r.lines()
        .filter(util::remove_empty_lines)
        .map(|l| {
            l.context("failed to read line").and_then(|l| {
                let (direction, amount) = l
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("failed to split line: {}", l))?;
                let amount = amount
                    .parse::<i32>()
                    .context(format!("failed to parse amount '{}' as number", amount))?;
                match direction.to_lowercase().as_ref() {
                    "forward" => Ok(Command::Forward(amount)),
                    "down" => Ok(Command::Down(amount)),
                    "up" => Ok(Command::Up(amount)),
                    _ => Err(anyhow!("unknown direction: {}", direction)),
                }
            })
        })
        .collect::<Result<Vec<_>>>()
}

pub struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    pub fn new() -> Self {
        Position {
            horizontal: 0,
            depth: 0,
        }
    }

    pub fn follow(&mut self, command_list: &[Command]) {
        command_list.iter().for_each(|c| match c {
            Command::Forward(amount) => self.horizontal += amount,
            Command::Down(amount) => self.depth += amount,
            Command::Up(amount) => self.depth -= amount,
        });
    }

    pub fn checksum(&self) -> i32 {
        self.horizontal * self.depth
    }
}

pub struct AimPosition {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl AimPosition {
    pub fn new() -> Self {
        AimPosition {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn follow(&mut self, command_list: &[Command]) {
        command_list.iter().for_each(|c| match c {
            Command::Forward(amount) => {
                self.horizontal += amount;
                self.depth += self.aim * amount;
            }
            Command::Down(amount) => self.aim += amount,
            Command::Up(amount) => self.aim -= amount,
        });
    }

    pub fn checksum(&self) -> i32 {
        self.horizontal * self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil;

    #[test]
    fn test_position_follow() {
        let input = testutil::string_reader(
            r"forward 5
down 5
forward 8
up 3
down 8
forward 2",
        );
        let command_list = command_list(input).unwrap();
        let mut pos = Position::new();
        pos.follow(&command_list);
        assert_eq!(150, pos.checksum());
    }

    #[test]
    fn test_aim_position_follow() {
        let input = testutil::string_reader(
            r"forward 5
down 5
forward 8
up 3
down 8
forward 2",
        );
        let command_list = command_list(input).unwrap();
        let mut pos = AimPosition::new();
        pos.follow(&command_list);
        assert_eq!(900, pos.checksum());
    }
}
