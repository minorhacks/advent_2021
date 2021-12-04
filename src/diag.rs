use anyhow::Context;
use anyhow::Result;
use std::io;

use crate::util;

pub fn parse_report<T: io::BufRead>(r: T) -> Result<Vec<String>> {
    r.lines()
        .filter(util::remove_empty_lines)
        .map(|l| l.context("failed to read line"))
        .collect::<Result<Vec<_>>>()
}

pub fn power_consumption(report: &[String], num_bits: usize) -> u32 {
    let bit_counts = count_bits(report, num_bits);

    let (mut gamma, mut epsilon) = (0u32, 0u32);
    bit_counts.into_iter().for_each(|c| {
        if c > report.len() / 2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
        gamma *= 2;
        epsilon *= 2;
    });
    gamma /= 2;
    epsilon /= 2;

    gamma * epsilon
}

pub fn life_support_rating(report: &[String]) -> Result<u32> {
    let mut most_common = Vec::from(report);
    let mut most_common_prefix = String::new();

    while most_common.len() > 1 {
        let zero_prefix = most_common_prefix.clone() + "0";
        let one_prefix = most_common_prefix.clone() + "1";
        let zero_count = most_common
            .iter()
            .filter(|e| e.starts_with(&zero_prefix))
            .count();
        let one_count = most_common
            .iter()
            .filter(|e| e.starts_with(&one_prefix))
            .count();
        most_common_prefix = if one_count >= zero_count {
            one_prefix
        } else {
            zero_prefix
        };
        most_common.retain(|x| x.starts_with(&most_common_prefix));
    }

    let mut least_common = Vec::from(report);
    let mut least_common_prefix = String::new();

    while least_common.len() > 1 {
        let zero_prefix = least_common_prefix.clone() + "0";
        let one_prefix = least_common_prefix.clone() + "1";
        let zero_count = least_common
            .iter()
            .filter(|e| e.starts_with(&zero_prefix))
            .count();
        let one_count = least_common
            .iter()
            .filter(|e| e.starts_with(&one_prefix))
            .count();
        least_common_prefix = if one_count >= zero_count {
            zero_prefix
        } else {
            one_prefix
        };
        least_common.retain(|x| x.starts_with(&least_common_prefix));
    }

    Ok(u32::from_str_radix(&most_common[0], 2)? * u32::from_str_radix(&least_common[0], 2)?)
}

fn count_bits(report: &[String], num_bits: usize) -> Vec<usize> {
    let bit_counts = report.iter().fold(vec![0; num_bits], |mut counts, elem| {
        elem.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                counts[i] += 1
            }
        });
        counts
    });
    bit_counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil;

    #[test]
    fn test_gamma_epsilon_rate() {
        let input = testutil::string_reader(
            r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
",
        );
        let report = parse_report(input).unwrap();
        let power = power_consumption(&report, 5);
        assert_eq!(198, power);
    }

    #[test]
    fn test_life_support_rating() {
        let input = testutil::string_reader(
            r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
",
        );
        let report = parse_report(input).unwrap();
        let got = life_support_rating(&report).unwrap();
        assert_eq!(230, got);
    }
}
