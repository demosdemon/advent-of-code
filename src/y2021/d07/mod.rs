pub(crate) mod part1;
pub(crate) mod part2;

use std::str::FromStr;

use anyhow::{Context, Error, Result};

#[derive(derive_more::IntoIterator)]
#[into_iterator(ref)]
pub struct Ocean(Vec<isize>);

::aoc::derive_FromIterator!(Ocean, isize);

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
    pub fn solve<F>(&self, mut cost: F) -> isize
    where
        F: FnMut(isize, isize) -> isize,
    {
        self.range()
            .map(|a| self.into_iter().map(|&b| (cost)(a, b)).sum())
            .min()
            .unwrap_or_default()
    }

    pub fn range(&self) -> std::ops::RangeInclusive<isize> {
        self.min()..=self.max()
    }

    pub fn min(&self) -> isize {
        self.into_iter().min().unwrap_or(&0).to_owned()
    }

    pub fn max(&self) -> isize {
        self.into_iter().max().unwrap_or(&0).to_owned()
    }
}
