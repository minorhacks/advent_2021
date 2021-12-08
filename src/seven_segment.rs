use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashSet;
use std::io;

use crate::util;

type Digit = HashSet<char>;

pub struct Entry {
    signal_patterns: [Digit; 10],
    output: [Digit; 4],
}

pub struct Entries(Vec<Entry>);

impl std::str::FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s
            .split_once(" | ")
            .ok_or_else(|| anyhow!("failed to split '{}' on '|' delimiter", s))?;
        let signal_patterns = first
            .trim_matches(char::is_whitespace)
            .split_whitespace()
            .map(|s| s.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();
        if signal_patterns.len() != 10 {
            return Err(anyhow!(
                "signal_patterns got {} elements; want 10",
                signal_patterns.len()
            ));
        }
        let output = second
            .trim_matches(char::is_whitespace)
            .split_whitespace()
            .map(|s| s.chars().collect::<HashSet<_>>())
            .collect::<Vec<_>>();
        if output.len() != 4 {
            return Err(anyhow!("output got {} elements; want 4", output.len()));
        }
        let signal_patterns = signal_patterns.try_into().map_err(|v: Vec<Digit>| {
            anyhow!("signal_patterns got {} elements; want 10", v.len())
        })?;
        let output = output
            .try_into()
            .map_err(|v: Vec<Digit>| anyhow!("output got {} elements; want 4", v.len()))?;
        Ok(Self {
            signal_patterns,
            output,
        })
    }
}

impl Entry {
    fn output_unique_digit_count(&self) -> usize {
        self.output
            .iter()
            .map(|d| match d.len() {
                2 | 4 | 3 | 7 => 1,
                _ => 0,
            })
            .sum()
    }

    fn digit_with_num_segments(&self, num: usize) -> Option<&HashSet<char>> {
        self.all_digits().find(|hs| hs.len() == num)
    }

    fn all_digits(&self) -> impl Iterator<Item = &std::collections::HashSet<char>> + '_ {
        self.signal_patterns.iter().chain(self.output.iter())
    }

    pub fn output_value(&self) -> Result<i32> {
        let one = self
            .digit_with_num_segments(2)
            .ok_or_else(|| anyhow!("no 1 in: {}", self.to_string()))?;
        let four = self
            .digit_with_num_segments(4)
            .ok_or_else(|| anyhow!("no 4 in: {}", self.to_string()))?;
        let seven = self
            .digit_with_num_segments(3)
            .ok_or_else(|| anyhow!("no 3 in: {}", self.to_string()))?;

        let output = self
            .output
            .iter()
            .map(|hs| {
                let intersect_one = hs.intersection(one).count();
                let intersect_four = hs.intersection(four).count();
                let intersect_seven = hs.intersection(seven).count();
                match (hs.len(), intersect_one, intersect_four, intersect_seven) {
                    (2, _, _, _) => 1,
                    (5, 1, 2, 2) => 2,
                    (5, 2, 3, 3) => 3,
                    (4, _, _, _) => 4,
                    (5, 1, 3, 2) => 5,
                    (6, 1, 3, 2) => 6,
                    (3, _, _, _) => 7,
                    (7, _, _, _) => 8,
                    (6, 2, 4, 3) => 9,
                    (6, 2, 3, 3) => 0,
                    _ => panic!(
                        "failed to match profile: {:?}",
                        (hs.len(), intersect_one, intersect_four, intersect_seven)
                    ),
                }
            })
            .fold(0, |acc, digit| acc * 10 + digit);
        Ok(output)
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for hs in &self.signal_patterns {
            write!(f, "{} ", hs.iter().collect::<String>())?;
        }
        write!(f, "| ")?;
        for hs in &self.output {
            write!(f, "{} ", hs.iter().collect::<String>())?;
        }
        Ok(())
    }
}

impl Entries {
    pub fn parse<T: io::BufRead>(r: T) -> Result<Entries> {
        let entries = r
            .lines()
            .filter(util::remove_empty_lines)
            .map(|s| {
                s.context("failed to read line")
                    .and_then(|s| s.parse::<Entry>())
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Entries(entries))
    }

    pub fn output_unique_digit_count(&self) -> usize {
        self.0.iter().map(Entry::output_unique_digit_count).sum()
    }

    pub fn output_sum(&self) -> Result<i32> {
        Ok(self
            .0
            .par_iter()
            .map(Entry::output_value)
            .collect::<Result<Vec<_>>>()?
            .iter()
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil;

    const INPUT: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_output_unique_digit_count() {
        let input = testutil::string_reader(INPUT);
        let entries = Entries::parse(input).unwrap();
        assert_eq!(26, entries.output_unique_digit_count());
    }

    #[test]
    fn test_output_sum() {
        let input = testutil::string_reader(INPUT);
        let entries = Entries::parse(input).unwrap();
        assert_eq!(61229, entries.output_sum().unwrap());
    }
}
