use std::collections::HashMap;

use anyhow::anyhow;
use anyhow::Result;

type Pair = (char, char);

#[derive(Clone)]
pub struct Template {
    pair_counts: HashMap<Pair, i64>,
    first_char: char,
}
pub struct Rules(HashMap<Pair, char>);

pub fn template_and_rules(s: &str) -> Result<(Template, Rules)> {
    let (template, rules) = s
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("can't split template from rules"))?;
    Ok((template.parse::<Template>()?, rules.parse::<Rules>()?))
}

impl std::str::FromStr for Template {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair_counts = s
            .chars()
            .zip(s.chars().skip(1))
            .fold(HashMap::new(), |mut acc, pair| {
                *acc.entry(pair).or_insert(0) += 1;
                acc
            });
        let first_char = s
            .chars()
            .next()
            .ok_or_else(|| anyhow!("failed to get first char of: {}", s))?;
        Ok(Template {
            pair_counts,
            first_char,
        })
    }
}

impl Template {
    pub fn step(&self, rules: &Rules) -> Result<Self> {
        let pair_counts = self
            .pair_counts
            .iter()
            .flat_map(|(k, v)| {
                let insert_char = *rules
                    .0
                    .get(k)
                    .unwrap_or_else(|| panic!("failed to find rule for: {:?}", k));
                std::iter::once(((k.0, insert_char), v))
                    .chain(std::iter::once(((insert_char, k.1), v)))
            })
            .fold(HashMap::new(), |mut acc, (k, v)| {
                *acc.entry(k).or_insert(0) += v;
                acc
            });
        Ok(Self {
            pair_counts,
            first_char: self.first_char,
        })
    }

    pub fn step_n(&self, rules: &Rules, n: usize) -> Result<Self> {
        let mut tmpl = self.clone();
        for _ in 0..n {
            tmpl = tmpl.step(rules)?;
        }
        Ok(tmpl)
    }

    pub fn score(&self) -> i64 {
        let mut char_counts =
            self.pair_counts
                .iter()
                .fold(HashMap::new(), |mut acc, (pair, count)| {
                    *acc.entry(pair.1).or_insert(0) += count;
                    acc
                });
        *char_counts.entry(self.first_char).or_insert(0) += 1;
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
                    (start.chars().next().unwrap(), start.chars().nth(1).unwrap()),
                    insert
                        .chars()
                        .next()
                        .ok_or_else(|| anyhow!("can't get first char of 'insert'"))?,
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
        let template = template.step_n(&rules, 10).unwrap();
        assert_eq!(1587, template.score())
    }
}
