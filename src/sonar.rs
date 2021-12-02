use anyhow::Context;
use anyhow::Result;
use std::io;

use crate::util;

type Depth = i32;
type DepthList = Vec<Depth>;

pub fn depth_list<T: io::BufRead>(r: T) -> Result<DepthList> {
    r.lines()
        .filter(util::remove_empty_lines)
        .map(|l| {
            l.context("failed to read line").and_then(|l| {
                l.parse::<i32>()
                    .context(format!("failed to parse line: {:?}", &l))
            })
        })
        .collect::<Result<Vec<_>>>()
}

pub fn num_depth_increases(list: &[Depth]) -> usize {
    let (_last_elem, num_increases) = list.iter().fold((None, 0), |mut acc, elem| {
        acc.1 += match acc.0 {
            Some(last_value) if elem > last_value => 1,
            _ => 0,
        };
        acc.0 = Some(elem);
        acc
    });
    num_increases
}

pub fn num_increases_windowed(list: &[Depth], window_size: usize) -> usize {
    let mut num_increases = 0;
    for i in 0..list.len() - window_size {
        num_increases += if list.iter().skip(i + 1).take(window_size).sum::<i32>()
            > list.iter().skip(i).take(window_size).sum::<i32>()
        {
            1
        } else {
            0
        }
    }
    num_increases
}

#[cfg(test)]
mod tests {
    extern crate table_test;

    use super::*;
    use crate::testutil;

    #[test]
    fn test_num_depth_increases() {
        let input = testutil::string_reader("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
        let depth_list = depth_list(input).unwrap();
        assert_eq!(7, num_depth_increases(&depth_list));
    }

    #[test]
    fn test_num_increases_windowed() {
        let input = testutil::string_reader("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
        let depth_list = depth_list(input).unwrap();
        assert_eq!(5, num_increases_windowed(&depth_list, 3));
    }
}
