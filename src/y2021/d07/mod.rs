mod part1;
mod part2;

use std::num::ParseIntError;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("unexpected end of input")]
    EndOfInput,

    #[error("unable to parse positions; got {0}; err {1}")]
    Parse(String, #[source] ParseIntError),
}

#[derive(derive_more::IntoIterator)]
#[into_iterator(ref)]
struct Ocean(Vec<isize>);

crate::derive_FromIterator!(Ocean, isize);

impl FromStr for Ocean {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .next()
            .ok_or(Error::EndOfInput)?
            .split(',')
            .map(str::parse)
            .try_collect()
            .map_err(|e| Error::Parse(s.to_owned(), e))
    }
}

impl Ocean {
    pub fn solve<F>(self, mut cost: F) -> isize
    where
        F: FnMut(isize, isize) -> isize,
    {
        self.range()
            .map(|a| (&self).into_iter().map(|&b| (cost)(a, b)).sum())
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
