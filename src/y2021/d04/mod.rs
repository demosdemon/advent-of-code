mod part1;
mod part2;

mod builder;
mod matrix;

use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("expected length of tiles to be a perfect square; got {0} (sqrt: {1})")]
    InvalidTileLength(usize, f32),

    #[error("expected all tiles to be unique, found {0} duplicates: {1}")]
    DuplicateTiles(usize, String),

    #[error("unexpected end of input")]
    EndOfInput,

    #[error("expected an empty line; got {0}")]
    NonEmptyLine(String),

    #[error("unable to parse pulls; got {0}; err {1}")]
    ParsePulls(String, #[source] ParseIntError),

    #[error("unable to parse the board; got an invalid integer; got {0}; err {1}")]
    BoardInt(String, #[source] ParseIntError),
}

impl Error {
    fn from_empty_line(s: &str) -> Result<(), Self> {
        if s.is_empty() {
            Ok(())
        } else {
            Err(Self::NonEmptyLine(s.to_owned()))
        }
    }
}
