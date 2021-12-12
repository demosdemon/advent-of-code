mod part1;
mod part2;

use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("invalid direction: {0}")]
    InvalidDirection(String),

    #[error("unable to parse number: {0}")]
    Parse(#[from] ParseIntError),
}

#[derive(Debug)]
enum Direction {
    Forward(isize),
    Up(isize),
    Down(isize),
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amt) = s
            .split_once(" ")
            .ok_or_else(|| Error::InvalidInput(s.to_owned()))?;
        let amt = amt.parse()?;
        match dir {
            "forward" => Ok(Self::Forward(amt)),
            "up" => Ok(Self::Up(amt)),
            "down" => Ok(Self::Down(amt)),
            _ => Err(Error::InvalidDirection(dir.to_owned())),
        }
    }
}
