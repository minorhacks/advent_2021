use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

type NumList = VecDeque<i8>;

pub struct Board {
    bingos: Vec<HashSet<i8>>,
}

impl Board {
    pub fn mark(&mut self, num: i8) {
        self.bingos.iter_mut().for_each(|h| {
            h.remove(&num);
        });
    }

    fn has_bingo(&self) -> Option<i32> {
        if self.bingos.iter().any(|h| h.is_empty()) {
            let dedup = self.bingos.iter().fold(HashSet::new(), |combined, h| {
                combined.into_iter().chain(h).collect()
            });
            let sum = dedup.into_iter().map(|&i| i as i32).sum();
            return Some(sum);
        }
        None
    }
}

pub struct Game {
    num_list: NumList,
    boards: Vec<Board>,
}

impl Game {
    pub fn load<T: io::BufRead>(mut r: T) -> Result<Game> {
        let mut whole_ass_input = String::new();
        r.read_to_string(&mut whole_ass_input)
            .context("failed to read input")?;
        let mut elems = whole_ass_input.split("\n\n");
        let num_list = elems
            .next()
            .ok_or(anyhow!("failed to find number list"))?
            .split(',')
            .map(|n| {
                n.parse::<i8>()
                    .with_context(|| format!("failed to parse num '{}' in list", n))
            })
            .collect::<Result<VecDeque<_>>>()?;
        let boards = elems
            .map(|e| {
                let nums = e
                    .trim_matches(char::is_whitespace)
                    .split_whitespace()
                    .map(|num| {
                        num.parse::<i8>()
                            .with_context(|| format!("failed to parse num '{}' in board", num))
                    })
                    .collect::<Result<Vec<_>>>()?;
                let mut bingos = vec![HashSet::new(); 10];
                nums.into_iter().enumerate().for_each(|(i, num)| {
                    bingos[i / 5].insert(num);
                    bingos[(i % 5) + 5].insert(num);
                });
                Ok(Board { bingos })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Game { num_list, boards })
    }

    pub fn play(&mut self) -> Result<(i8, i32)> {
        let mut current = 0;
        while !self.boards.iter().any(|b| b.has_bingo().is_some()) {
            current = self
                .num_list
                .pop_front()
                .ok_or(anyhow!("ran out of numbers"))?;
            self.boards.iter_mut().for_each(|b| b.mark(current));
        }
        let winning_score = self
            .boards
            .iter()
            .filter_map(|b| b.has_bingo())
            .next()
            .ok_or(anyhow!("no winning boards"))?;
        Ok((current, winning_score))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil;

    #[test]
    fn test_play_bingo() {
        let input = testutil::string_reader(
            r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
",
        );
        let mut game = Game::load(input).unwrap();
        let (last_num, winning_board_score) = game.play().unwrap();
        assert_eq!(24, last_num);
        assert_eq!(188, winning_board_score);
    }
}
