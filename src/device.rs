use anyhow::{Context, Result};
use std::{collections::HashSet, io};

use crate::util;

pub struct FrequencyDuplicates {
    freq_drift_list: Vec<i32>,
    seen_frequencies: HashSet<i32>,

    current_frequency: i32,
    current_idx: usize,
}

impl FrequencyDuplicates {
    pub fn new<T: io::BufRead>(r: T) -> Result<Self> {
        let freq_drift_list = r
            .lines()
            .filter(util::remove_empty_lines)
            .map(|l| {
                l.context("failed to read line").and_then(|s| {
                    s.parse::<i32>()
                        .with_context(|| format!("failed to parse num from: {:?}", s))
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let mut seen_frequencies = HashSet::new();
        seen_frequencies.insert(0);

        Ok(Self {
            freq_drift_list,
            seen_frequencies,
            current_frequency: Default::default(),
            current_idx: Default::default(),
        })
    }

    pub fn drift_sum(&self) -> i32 {
        self.freq_drift_list.iter().sum()
    }
}

impl Iterator for FrequencyDuplicates {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current_frequency += self.freq_drift_list[self.current_idx];
            self.current_idx = (self.current_idx + 1) % self.freq_drift_list.len();
            if self.seen_frequencies.contains(&self.current_frequency) {
                return Some(self.current_frequency);
            }
            self.seen_frequencies.insert(self.current_frequency);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate table_test;

    use super::*;
    use crate::testutil;

    #[test]
    fn test_sum_frequency_drift() {
        let table = vec![
            ("+1\n+1\n+1\n", 3),
            ("+1\n+1\n-2\n", 0),
            ("-1\n-2\n-3\n", -6),
        ];
        for (validator, input, expected) in table_test::table_test!(table) {
            let r = testutil::string_reader(input);
            let dupes = FrequencyDuplicates::new(r).unwrap();
            let sum = dupes.drift_sum();
            validator
                .given(&format!("{:?}", input))
                .when("sum frequency drift")
                .then(&format!("it should be {}", expected))
                .assert_eq(expected, sum);
        }
    }

    #[test]
    fn test_frequency_duplicates() {
        let table = vec![
            ("+1\n-1\n", 0),
            ("+3\n+3\n+4\n-2\n-4\n", 10),
            ("-6\n+3\n+8\n+5\n-6\n", 5),
            ("+7\n+7\n-2\n-7\n-4\n", 14),
        ];
        for (validator, input, expected) in table_test::table_test!(table) {
            let r = testutil::string_reader(input);
            let mut dupes = FrequencyDuplicates::new(r).unwrap();
            let first_dupe = dupes.next();
            validator
                .given(&format!("{:?}", input))
                .when("finding first duplicate frequency")
                .then(&format!("it should be {}", expected))
                .assert_eq(Some(expected), first_dupe);
        }
    }
}
