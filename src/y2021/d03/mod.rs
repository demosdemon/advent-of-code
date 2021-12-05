pub mod part1;
pub mod part2;

mod bit;
mod line;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalit Bit: {0}")]
    InvalidBit(char),
}
