use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

use crate::util;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

pub struct PointPair(Point, Point);
pub type PointList = Vec<PointPair>;

#[derive(Default, Debug)]
pub struct PointCounts(HashMap<Point, i32>);

pub fn read_points<T: io::BufRead>(r: T) -> Result<PointList> {
    r.lines()
        .filter(util::remove_empty_lines)
        .map(|l| {
            l.context("failed to read line").and_then(|l| {
                let pair = l
                    .split_once(" -> ")
                    .ok_or_else(|| anyhow!("line missing arrow delimiter: {}", l))?;
                let first_point = pair.0.parse::<Point>()?;
                let second_point = pair.1.parse::<Point>()?;
                Ok(PointPair(first_point, second_point))
            })
        })
        .collect::<Result<Vec<_>>>()
}

impl std::str::FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or_else(|| anyhow!("point missing comma delimiter: {}", s))?;
        let x = x
            .parse::<i32>()
            .context(format!("failed to parse x coordinate as i32: {}", x))?;
        let y = y
            .parse::<i32>()
            .context(format!("failed to parse x coordinate as i32: {}", y))?;
        Ok(Point { x, y })
    }
}

impl PointPair {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }

    fn x_iter(&self) -> Box<dyn Iterator<Item = i32> + '_> {
        match self.0.x.cmp(&self.1.x) {
            Ordering::Equal => Box::new(std::iter::repeat(self.0.x)),
            Ordering::Greater => Box::new((self.1.x..=self.0.x).rev()),
            Ordering::Less => Box::new(self.0.x..=self.1.x),
        }
    }

    fn y_iter(&self) -> Box<dyn Iterator<Item = i32> + '_> {
        match self.0.y.cmp(&self.1.y) {
            Ordering::Equal => Box::new(std::iter::repeat(self.0.y)),
            Ordering::Greater => Box::new((self.1.y..=self.0.y).rev()),
            Ordering::Less => Box::new(self.0.y..=self.1.y),
        }
    }

    fn between_points_iter(&self) -> impl Iterator<Item = Point> + '_ {
        self.x_iter()
            .zip(self.y_iter())
            .map(|(x, y)| Point { x, y })
    }
}

impl PointCounts {
    pub fn count_horizontal_vertical(points: &[PointPair]) -> PointCounts {
        let mut counts = PointCounts::default();
        points
            .iter()
            .filter(|&p| PointPair::is_horizontal_or_vertical(p))
            .for_each(|pair| {
                pair.between_points_iter()
                    .for_each(|p| *counts.0.entry(p).or_insert(0) += 1);
            });
        counts
    }

    pub fn count_all(points: &[PointPair]) -> PointCounts {
        let mut counts = PointCounts::default();
        points.iter().for_each(|pair| {
            pair.between_points_iter()
                .for_each(|p| *counts.0.entry(p).or_insert(0) += 1);
        });
        counts
    }

    pub fn overlap_count(&self) -> usize {
        self.0.values().filter(|&&x| x > 1).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil;

    const INPUT: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_horizontal_vertical_overlap() {
        let input = testutil::string_reader(INPUT);
        let point_list = read_points(input).unwrap();
        let counts = PointCounts::count_horizontal_vertical(&point_list);
        assert_eq!(5, counts.overlap_count());
    }

    #[test]
    fn test_all_overlap() {
        let input = testutil::string_reader(INPUT);
        let point_list = read_points(input).unwrap();
        let counts = PointCounts::count_all(&point_list);
        assert_eq!(12, counts.overlap_count());
    }
}
