use std::collections::HashSet;
use std::mem::swap;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

pub enum Instruction {
    FoldX(i32),
    FoldY(i32),
}

pub struct Page {
    dots: HashSet<(i32, i32)>,
}

pub struct Instructions(Vec<Instruction>);

pub fn page_and_instructions(s: &str) -> Result<(Page, Instructions)> {
    let (page_str, instructions_str) = s
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("failed to find page/instructions split"))?;
    let dots = page_str
        .lines()
        .map(|l| {
            l.split_once(',')
                .ok_or_else(|| anyhow!("failed to split coordinate: {}", l))
                .and_then(|(x, y)| {
                    let x = x.parse::<i32>().context("failed to parse x coord")?;
                    let y = y.parse::<i32>().context("failed to parse y coord")?;
                    Ok((x, y))
                })
        })
        .collect::<Result<HashSet<_>>>()?;
    let instructions = instructions_str
        .lines()
        .map(|l| {
            l.split_once('=')
                .ok_or_else(|| anyhow!("failed to split instruction: {}", l))
                .and_then(|(inst, num)| {
                    let num = num.parse::<i32>().context("failed to parse fold amount")?;
                    match inst {
                        "fold along x" => Ok(Instruction::FoldX(num)),
                        "fold along y" => Ok(Instruction::FoldY(num)),
                        _ => Err(anyhow!("unrecognized instruction: {}", inst)),
                    }
                })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok((Page { dots }, Instructions(instructions)))
}

impl Page {
    fn step(&mut self, inst: &Instruction) {
        let mut new_set = HashSet::new();
        self.dots.iter().for_each(|&(x, y)| match *inst {
            Instruction::FoldX(num) => {
                if x > num {
                    new_set.insert((-x + 2 * num, y));
                } else {
                    new_set.insert((x, y));
                }
            }
            Instruction::FoldY(num) => {
                if y > num {
                    new_set.insert((x, -y + 2 * num));
                } else {
                    new_set.insert((x, y));
                }
            }
        });
        swap(&mut self.dots, &mut new_set);
    }

    pub fn follow_first(&mut self, instructions: &Instructions) {
        self.step(instructions.0.get(0).unwrap());
    }

    pub fn follow_all(&mut self, instructions: &Instructions) {
        instructions.0.iter().for_each(|i| self.step(i));
    }

    pub fn dot_count(&self) -> usize {
        self.dots.len()
    }
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = self.dots.iter().fold((0, 0), |mut acc, cur| {
            if cur.0 > acc.0 {
                acc.0 = cur.0;
            }
            if cur.1 > acc.1 {
                acc.1 = cur.1;
            }
            acc
        });
        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.dots.contains(&(x, y)) {
                    write!(f, "█")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    const INPUT: &str = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_fold_once() {
        let (mut page, instructions) = page_and_instructions(INPUT).unwrap();
        page.follow_first(&instructions);
        assert_eq!(17, page.dot_count());
    }
}
