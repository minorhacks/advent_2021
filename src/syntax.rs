use std::collections::VecDeque;
use std::io;

use anyhow::Context;
use anyhow::Result;

use crate::util;

pub struct Line(String);
pub struct Lines(Vec<Line>);

enum ParseResult {
    IllegalChar(char),
    CompletionChars(Vec<char>),
}

impl Lines {
    pub fn parse<T: io::BufRead>(r: T) -> Result<Lines> {
        Ok(Lines(
            r.lines()
                .filter(util::remove_empty_lines)
                .map(|l| l.context("failed to read line").map(Line::new))
                .collect::<Result<Vec<_>>>()?,
        ))
    }

    pub fn syntax_error_score(&self) -> i32 {
        self.0
            .iter()
            .filter_map(|l| match l.parse_result() {
                ParseResult::IllegalChar(c) => Some(c),
                _ => None,
            })
            .map(Line::illegal_char_score)
            .sum()
    }

    pub fn middle_completion_score(&self) -> i64 {
        let mut completion_scores = self
            .0
            .iter()
            .filter_map(|l| match l.parse_result() {
                ParseResult::CompletionChars(c) => Some(c),
                _ => None,
            })
            .map(|chars| chars.into_iter().fold(0, Line::completion_chars_score))
            .collect::<Vec<_>>();
        completion_scores.sort_unstable();
        completion_scores[completion_scores.len() / 2]
    }
}

impl Line {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    fn parse_result(&self) -> ParseResult {
        let mut stack = VecDeque::new();
        let first_illegal = self
            .0
            .chars()
            .filter_map(|c| match c {
                '(' | '{' | '[' | '<' => {
                    stack.push_front(c);
                    None
                }
                '}' => {
                    if Some('{') == stack.pop_front() {
                        None
                    } else {
                        Some(c)
                    }
                }
                ')' => {
                    if Some('(') == stack.pop_front() {
                        None
                    } else {
                        Some(c)
                    }
                }
                ']' => {
                    if Some('[') == stack.pop_front() {
                        None
                    } else {
                        Some(c)
                    }
                }
                '>' => {
                    if Some('<') == stack.pop_front() {
                        None
                    } else {
                        Some(c)
                    }
                }
                _ => panic!("unexpected char: {}", c),
            })
            .next();
        match first_illegal {
            Some(c) => ParseResult::IllegalChar(c),
            None => ParseResult::CompletionChars(
                stack
                    .into_iter()
                    .map(Line::completion_char_for)
                    .collect::<Vec<_>>(),
            ),
        }
    }

    fn illegal_char_score(c: char) -> i32 {
        match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("unexpected illegal char: {}", c),
        }
    }

    fn completion_chars_score(acc: i64, c: char) -> i64 {
        acc * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("unexpected completion char: {}", c),
            }
    }

    fn completion_char_for(c: char) -> char {
        match c {
            '(' => ')',
            '[' => ']',
            '<' => '>',
            '{' => '}',
            _ => panic!("unexpected opening char: {}", c),
        }
    }
}

#[cfg(test)]

mod tests {
    use crate::testutil;

    use super::*;

    const INPUT: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_syntax_error_score() {
        let lines = Lines::parse(testutil::string_reader(INPUT)).unwrap();
        assert_eq!(26397, lines.syntax_error_score());
    }

    #[test]
    fn test_middle_completion_score() {
        let lines = Lines::parse(testutil::string_reader(INPUT)).unwrap();
        assert_eq!(288957, lines.middle_completion_score());
    }
}
