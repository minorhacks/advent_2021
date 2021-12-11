use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

type Height = u8;
pub struct HeightMap(array2d::Array2D<Height>);

impl std::str::FromStr for HeightMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s
            .lines()
            .next()
            .ok_or_else(|| anyhow!("expected at least one line"))?
            .len();
        let height = s.lines().count();
        let elems = s
            .lines()
            .map(|l| l.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>();
        let arr = array2d::Array2D::from_row_major(&elems, height, width);
        Ok(HeightMap(arr))
    }
}

impl std::fmt::Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..self.0.num_rows()).for_each(|r| {
            self.0
                .row_iter(r)
                .for_each(|c| write!(f, "{}", (*c + b'0') as char).unwrap());
            writeln!(f).unwrap();
        });
        Ok(())
    }
}

impl HeightMap {
    #[allow(clippy::if_same_then_else)]
    #[allow(clippy::needless_bool)]
    fn low_points(&self) -> Vec<(usize, usize)> {
        (0..self.0.num_rows())
            .cartesian_product(0..self.0.num_columns())
            .filter(|&(r, c)| {
                if r > 0 && self.0[(r - 1, c)] <= self.0[(r, c)] {
                    false
                } else if r < self.0.num_rows() - 1 && self.0[(r + 1, c)] <= self.0[(r, c)] {
                    false
                } else if c > 0 && self.0[(r, c - 1)] <= self.0[(r, c)] {
                    false
                } else if c < self.0.num_columns() - 1 && self.0[(r, c + 1)] <= self.0[(r, c)] {
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn risk_level_sum(&self) -> i32 {
        self.low_points()
            .into_iter()
            .map(|coords| self.0[coords] as i32 + 1)
            .sum()
    }

    fn basin_sizes(&self) -> Vec<usize> {
        let mut basin_sizes = Vec::new();
        let low_points = self.low_points();
        for (r, c) in low_points {
            let mut visit = VecDeque::new();
            let mut seen = HashSet::new();
            visit.push_back((r, c));
            while !visit.is_empty() {
                let (r, c) = visit.pop_front().unwrap();
                seen.insert((r, c));
                for adjacent in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let next = (r as i32 + adjacent.0, c as i32 + adjacent.1);
                    if next.0 < 0 || next.1 < 0 {
                        continue;
                    }
                    let next = (next.0 as usize, next.1 as usize);
                    if next.0 >= self.0.num_rows() || next.1 >= self.0.num_columns() {
                        continue;
                    }
                    if self.0[next] == 9 || seen.contains(&next) {
                        continue;
                    }
                    visit.push_back(next);
                }
            }
            basin_sizes.push(seen.len());
        }
        basin_sizes
    }

    pub fn largest_basins_product(&self, num: usize) -> usize {
        let mut sizes = self.basin_sizes();
        sizes.sort_by(|i, j| j.partial_cmp(i).unwrap());
        sizes.into_iter().take(num).product()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_risk_level_sum() {
        let height_map = INPUT.parse::<HeightMap>().unwrap();
        assert_eq!(15, height_map.risk_level_sum());
    }

    #[test]
    fn test_largest_basins_product() {
        let height_map = INPUT.parse::<HeightMap>().unwrap();
        assert_eq!(1134, height_map.largest_basins_product(3));
    }
}
