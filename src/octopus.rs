use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;

type Octopus = u8;
pub struct OctopusGrid {
    grid: array2d::Array2D<Octopus>,
    flashes: usize,
}

impl std::str::FromStr for OctopusGrid {
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
        let grid = array2d::Array2D::from_row_major(&elems, height, width);
        Ok(OctopusGrid { grid, flashes: 0 })
    }
}

impl std::fmt::Display for OctopusGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..self.grid.num_rows()).for_each(|r| {
            self.grid
                .row_iter(r)
                .for_each(|c| write!(f, "{}", (*c + b'0') as char).unwrap());
            writeln!(f).unwrap();
        });
        Ok(())
    }
}

impl OctopusGrid {
    pub fn step_n(mut self, n: usize) -> Self {
        for _ in 0..n {
            self = self.step_single();
        }
        self
    }

    pub fn first_synchronized_flash(mut self) -> usize {
        let mut step = 0;
        let total = self.grid.num_elements();
        loop {
            let before = self.flashes;
            self = self.step_single();
            step += 1;
            if self.flashes - before == total {
                return step;
            }
        }
    }

    fn step_single(mut self) -> Self {
        let mut blinked =
            array2d::Array2D::filled_with(false, self.grid.num_rows(), self.grid.num_columns());
        self.coord_iter().for_each(|(r, c)| self.grid[(r, c)] += 1);
        let mut changed = true;
        while changed {
            changed = false;
            self.coord_iter()
                .filter(|&c| self.grid[c] > 9 && !blinked[c])
                .collect::<Vec<_>>()
                .into_iter()
                .for_each(|c| {
                    changed = true;
                    blinked[c] = true;
                    self.adjacent_coords(c)
                        .into_iter()
                        .for_each(|c| self.grid[c] += 1);
                });
        }
        self.coord_iter().filter(|&c| blinked[c]).for_each(|c| {
            self.flashes += 1;
            self.grid[c] = 0
        });

        self
    }

    fn coord_iter(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.grid.num_rows()).cartesian_product(0..self.grid.num_columns())
    }

    fn adjacent_coords(&self, c: (usize, usize)) -> Vec<(usize, usize)> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&c| c != (0, 0))
            .map(move |offset| (c.0 as i32 + offset.0, c.1 as i32 + offset.1))
            .filter(|c| {
                c.0 >= 0
                    && c.1 >= 0
                    && c.0 < self.grid.num_rows() as i32
                    && c.1 < self.grid.num_columns() as i32
            })
            .map(|c| (c.0 as usize, c.1 as usize))
            .collect::<Vec<_>>()
    }

    pub fn flash_count(&self) -> usize {
        self.flashes
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_blink_10_steps() {
        let grid = INPUT.parse::<OctopusGrid>().unwrap();
        assert_eq!(204, grid.step_n(10).flash_count());
    }

    #[test]
    fn test_blink_100_steps() {
        let grid = INPUT.parse::<OctopusGrid>().unwrap();
        assert_eq!(1656, grid.step_n(100).flash_count());
    }

    #[test]
    fn test_first_synchronized_flash() {
        let grid = INPUT.parse::<OctopusGrid>().unwrap();
        assert_eq!(195, grid.first_synchronized_flash());
    }
}
