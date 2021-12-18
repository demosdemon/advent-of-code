mod part1;
mod part2;

use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};

#[derive(Debug)]
enum Direction {
    Forward(isize),
    Up(isize),
    Down(isize),
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (dir, amt) = s
            .split_once(" ")
            .context("splitting direction on whitespace")?;
        let amt = amt.parse()?;
        match dir {
            "forward" => Ok(Self::Forward(amt)),
            "up" => Ok(Self::Up(amt)),
            "down" => Ok(Self::Down(amt)),
            _ => Err(anyhow!("invalid direction keyword; got {}", dir)),
        }
    }
}

#[derive(derive_more::IntoIterator)]
#[into_iterator(ref)]
struct DirectionList(Vec<Direction>);

crate::derive_FromIterator!(DirectionList, Direction);
crate::derive_FromStr_for_FromIterator!(DirectionList, Direction);
