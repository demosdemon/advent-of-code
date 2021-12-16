mod part1;
mod part2;

use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("unexpected end of input")]
    EndOfInput,

    #[error("unable to parse fuel rating; got {0}; err {1}")]
    Parse(String, #[source] ParseIntError),
}

struct Ocean(Vec<u8>);

crate::derive_FromIterator!(Ocean, u8);

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
    pub fn count(self, days: usize) -> i128 {
        let mut lanterns = [0i128; 9];
        for lantern in self.0 {
            assert!(lantern < 9);
            lanterns[lantern as usize] += 1;
        }
        for _ in 0..days {
            let finished = lanterns[0];
            for idx in 0..8 {
                lanterns[idx] = lanterns[idx + 1];
            }
            lanterns[8] = finished;
            lanterns[6] += finished;
        }
        lanterns.into_iter().sum()
    }
}
