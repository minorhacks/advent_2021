use anyhow::Context;
use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

use crate::util;

pub struct Id {
    _s: String,
    counts: HashSet<i32>,
    _counts_by_char: HashMap<char, i32>,
}

impl Id {
    pub fn new(s: &str) -> Self {
        let mut counts_by_char = HashMap::new();
        s.chars()
            .for_each(|c| *counts_by_char.entry(c).or_insert(0) += 1);
        let counts = counts_by_char.values().cloned().collect::<HashSet<_>>();
        Id {
            _s: String::from(s),
            counts,
            _counts_by_char: counts_by_char,
        }
    }

    pub fn has_exactly_n(&self, n: i32) -> bool {
        self.counts.contains(&n)
    }
}

pub fn read_ids<T: io::BufRead>(r: T) -> Result<Vec<Id>> {
    r.lines()
        .filter(util::remove_empty_lines)
        .map(|l| l.context("failed to read line").map(|l| Id::new(&l)))
        .collect::<Result<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    extern crate table_test;

    use super::*;

    #[test]
    fn test_id_counts() {
        let table = vec![
            ("abcdef", (false, false)),
            ("bababc", (true, true)),
            ("abbcde", (true, false)),
            ("abcccd", (false, true)),
            ("aabcdd", (true, false)),
            ("abcdee", (true, false)),
            ("ababab", (false, true)),
        ];
        for (validator, input, (has_2_letters, has_3_letters)) in table_test::table_test!(table) {
            let id = Id::new(input);
            validator
                .given(input)
                .then(&format!(
                    "has 2 same letters: {}, has 3 same letters: {}",
                    has_2_letters, has_3_letters
                ))
                .assert_eq(has_2_letters, id.has_exactly_n(2))
                .assert_eq(has_3_letters, id.has_exactly_n(3));
        }
    }
}
