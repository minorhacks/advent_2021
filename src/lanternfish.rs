use std::collections::VecDeque;

use anyhow::Context;
use anyhow::Result;

#[derive(Default, Clone)]
pub struct School(VecDeque<usize>);

impl std::str::FromStr for School {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut school = School::default();
        let ages = s
            .split(',')
            .map(|num| {
                num.parse::<i32>()
                    .context(format!("failed to parse age: {}", num))
            })
            .collect::<Result<Vec<_>>>()?;
        (0..=8).for_each(|i| {
            school.0.push_back(ages.iter().filter(|&&x| x == i).count());
        });
        Ok(school)
    }
}

impl School {
    pub fn simulate_n_days(mut self, n: usize) -> School {
        for _ in 0..n {
            let old_fish_count = self.0.pop_front().unwrap();
            self.0.push_back(old_fish_count);
            self.0[6] += old_fish_count;
        }
        self
    }

    pub fn count(&self) -> usize {
        self.0.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"3,4,3,1,2";

    #[test]
    fn test_simulate_n_days() {
        let mut school = INPUT.parse::<School>().unwrap();
        school = school.simulate_n_days(80);
        assert_eq!(5934, school.count());

        school = school.simulate_n_days(256 - 80);
        assert_eq!(26984457539, school.count());
    }
}
