mod part1;
mod part2;

use std::str::FromStr;

use anyhow::{Context, Error, Result};

struct Ocean(Vec<u8>);

::aoc::derive_FromIterator!(Ocean, u8);

impl FromStr for Ocean {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .next()
            .context("readling input line")?
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .context("parsing input values")
    }
}

impl Ocean {
    pub fn count(&self, days: usize) -> isize {
        let mut lanterns = [0; 9];
        for &lantern in self.0.iter() {
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
