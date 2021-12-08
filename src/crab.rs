use anyhow::Context;
use anyhow::Result;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

type Position = i32;
pub struct Positions(Vec<Position>);

impl std::str::FromStr for Positions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions = s
            .split(',')
            .map(|num| {
                num.parse::<Position>()
                    .context(format!("failed to parse position: {}", num))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Positions(positions))
    }
}

impl Positions {
    pub fn cheapest_alignment_cost(&self) -> i32 {
        let (min, max) = (*self.0.iter().min().unwrap(), *self.0.iter().max().unwrap());
        (min..=max)
            .par_bridge()
            .map(|pos| self.0.iter().map(|val| (val - pos).abs()).sum())
            .min()
            .unwrap()
    }

    pub fn cheapest_alignment_cost_nonlinear(&self) -> i32 {
        let (min, max) = (*self.0.iter().min().unwrap(), *self.0.iter().max().unwrap());
        (min..=max)
            .par_bridge()
            .map(|pos| {
                self.0
                    .iter()
                    .map(|val| {
                        let steps = (val - pos).abs();
                        steps * (steps + 1) / 2
                    })
                    .sum()
            })
            .min()
            .unwrap()
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_cheapest_alignment_cost() {
        let positions = INPUT.parse::<Positions>().unwrap();
        assert_eq!(37, positions.cheapest_alignment_cost());
    }

    #[test]
    fn test_cheapest_alignment_cost_nonlinear() {
        let positions = INPUT.parse::<Positions>().unwrap();
        assert_eq!(168, positions.cheapest_alignment_cost_nonlinear());
    }
}
