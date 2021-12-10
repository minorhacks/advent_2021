use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;

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
    pub fn risk_level_sum(&self) -> i32 {
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
            .map(|coords| self.0[coords] as i32 + 1)
            .sum()
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
}
