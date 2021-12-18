use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Result;
use itertools::Itertools;

#[derive(Clone)]
pub struct Template(String);
pub struct Rules(HashMap<(char, char), String>);

pub fn template_and_rules(s: &str) -> Result<(Template, Rules)> {
    let (template, rules) = s
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("can't split template from rules"))?;
    Ok((template.parse::<Template>()?, rules.parse::<Rules>()?))
}

impl std::str::FromStr for Template {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Template {
    pub fn step(&self, rules: &Rules) -> Result<Self> {
        let template = self
            .0
            .chars()
            .zip(self.0.chars().skip(1))
            .map(|k| {
                rules
                    .0
                    .get(&k)
                    .ok_or_else(|| anyhow!("rule for '{:?}' not found", k))
            })
            .collect::<Result<Vec<_>>>()?
            .iter()
            .fold(
                self.0
                    .chars()
                    .nth(0)
                    .expect("empty self.0 in step()")
                    .to_string(),
                |mut acc, &k| {
                    acc += k;
                    acc
                },
            );
        Ok(Template(template))
    }

    pub fn step_n(&self, rules: &Rules, n: usize) -> Result<Self> {
        let mut tmpl = self.clone();
        for _ in 0..n {
            tmpl = tmpl.step(rules)?;
        }
        Ok(tmpl)
    }

    pub fn score(&self) -> i32 {
        let char_counts = self.0.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
        let lowest_count = char_counts
            .iter()
            .min_by_key(|k| k.1)
            .expect("no minimum count")
            .1;
        let highest_count = char_counts
            .iter()
            .max_by_key(|k| k.1)
            .expect("no maximum count")
            .1;
        highest_count - lowest_count
    }
}

impl std::str::FromStr for Rules {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rules = s
            .lines()
            .map(|s| {
                let (start, insert) = s
                    .split_once(" -> ")
                    .ok_or_else(|| anyhow!("failed to split on ' -> ': {}", s))?;
                Ok((
                    (start.chars().nth(0).unwrap(), start.chars().nth(1).unwrap()),
                    insert
                        .chars()
                        .interleave(start.chars().skip(1))
                        .collect::<String>(),
                ))
            })
            .collect::<Result<HashMap<_, _>>>()?;
        Ok(Rules(rules))
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_step() {
        let (template, rules) = template_and_rules(INPUT).unwrap();
        let template = template.step(&rules).unwrap();
        assert_eq!("NCNBCHB", template.0);
        let template = template.step(&rules).unwrap();
        assert_eq!("NBCCNBBBCBHCB", template.0);
        let template = template.step(&rules).unwrap();
        assert_eq!("NBBBCNCCNBBNBNBBCHBHHBCHB", template.0);
        let template = template.step(&rules).unwrap();
        assert_eq!(
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
            template.0
        );
        let template = template.step_n(&rules, 6).unwrap();
        assert_eq!(1588, template.score())
    }
}
