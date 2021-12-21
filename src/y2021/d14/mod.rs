mod part1;
mod part2;

use std::collections::BTreeMap;
use std::str::FromStr;

use anyhow::{anyhow, Context, Error};

#[derive(Debug)]
struct InsertionRule {
    matching_pair: (u8, u8),
    insert: u8,
}

impl From<InsertionRule> for ((u8, u8), u8) {
    fn from(value: InsertionRule) -> Self {
        (value.matching_pair, value.insert)
    }
}

impl FromStr for InsertionRule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s
            .split_once(" -> ")
            .with_context(|| format!("splitting {} on ` -> `", s))?;

        let lhs = lhs.as_bytes();
        if lhs.len() != 2 {
            return Err(anyhow!(
                "expected instructions to start with two bytes; got {}",
                s
            ));
        }
        let rhs = rhs.as_bytes();
        if rhs.len() != 1 {
            return Err(anyhow!(
                "expected instructions to end with one byte; got {}",
                s
            ));
        }
        Ok(Self {
            matching_pair: (lhs[0], lhs[1]),
            insert: rhs[0],
        })
    }
}

#[derive(Clone)]
struct Instructions {
    tuples: BTreeMap<(u8, u8), usize>,
    rules: BTreeMap<(u8, u8), u8>,
}

impl std::fmt::Debug for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tuples = self
            .tuples
            .iter()
            .map(|(&(a, b), &count)| {
                let b = [a, b];
                let s = std::str::from_utf8(&b).unwrap();
                format!("{} -> {}", s, count)
            })
            .collect::<Vec<_>>();
        let rules = self
            .rules
            .iter()
            .map(|(&(a, b), &next)| {
                let b1 = [a, b];
                let s1 = std::str::from_utf8(&b1).unwrap();
                let b2 = [next];
                let s2 = std::str::from_utf8(&b2).unwrap();
                format!("{} -> {}", s1, s2)
            })
            .collect::<Vec<_>>();
        f.debug_struct("Instructions")
            .field("tuples", &tuples)
            .field("rules", &rules)
            .field("score", &self.score())
            .finish()
    }
}

impl FromStr for Instructions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let tuples = lines
            .next()
            .context("reading polymer template from input")?
            .as_bytes()
            .windows(2)
            .map(|slice| (slice[0], slice[1]))
            .fold(BTreeMap::new(), |mut map, pair| {
                *map.entry(pair).or_default() += 1;
                map
            });
        lines
            .next()
            .context("reading empty line separator")
            .and_then(aoc::expect_empty_line)?;
        Ok(Self {
            tuples,
            rules: lines
                .map(str::parse::<InsertionRule>)
                .map(|r| r.map(Into::into))
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Instructions {
    pub fn step(self) -> Self {
        let mut tuples = BTreeMap::new();
        let rules = self.rules;
        for ((a, b), count) in self.tuples {
            if let Some(&ins) = rules.get(&(a, b)) {
                *tuples.entry((a, ins)).or_default() += count;
                *tuples.entry((ins, b)).or_default() += count;
            }
        }
        Self { tuples, rules }
    }

    pub fn score(&self) -> usize {
        let mut a_counts: BTreeMap<u8, usize> = BTreeMap::new();
        let mut b_counts: BTreeMap<u8, usize> = BTreeMap::new();
        for (&(a, b), &count) in self.tuples.iter() {
            *a_counts.entry(a).or_default() += count;
            *b_counts.entry(b).or_default() += count;
        }

        let mut counts = a_counts;
        for (k, v) in b_counts {
            let rv = counts.entry(k).or_default();
            *rv = std::cmp::max(*rv, v);
        }

        let (min, max) = counts
            .into_iter()
            .fold((usize::MAX, usize::MIN), |(min, max), (_, cnt)| {
                (std::cmp::min(min, cnt), std::cmp::max(max, cnt))
            });

        max - min
    }
}
