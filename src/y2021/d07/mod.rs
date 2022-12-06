pub(crate) mod part1;
pub(crate) mod part2;

use std::str::FromStr;

use anyhow::Context;
use anyhow::Error;
use anyhow::Result;

#[derive(derive_more::IntoIterator, macros::FromIterator, macros::TryFromStr)]
#[into_iterator(ref)]
#[from_iterator(usize)]
pub struct Ocean(Vec<usize>);

impl FromStr for Ocean {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .next()
            .context("reading input line")?
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .context("parsing input values")
    }
}

impl Ocean {
    pub fn solve<F>(&self, mut cost: F) -> usize
    where
        F: FnMut(usize) -> usize,
    {
        self.range()
            .map(|a| self.into_iter().map(|&b| (cost)(absub(a, b))).sum())
            .min()
            .unwrap_or_default()
    }

    pub fn range(&self) -> std::ops::RangeInclusive<usize> {
        self.min()..=self.max()
    }

    pub fn min(&self) -> usize {
        self.into_iter().min().unwrap_or(&0).to_owned()
    }

    pub fn max(&self) -> usize {
        self.into_iter().max().unwrap_or(&0).to_owned()
    }
}

fn absub(a: usize, b: usize) -> usize {
    if b <= a {
        a - b
    } else {
        b - a
    }
}
