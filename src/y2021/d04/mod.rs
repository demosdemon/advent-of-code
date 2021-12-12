pub(crate) mod part1;
pub(crate) mod part2;

mod builder;
mod matrix;

use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("expected length of tiles to be a perfect square; got {0} (sqrt: {1})")]
    InvalidTileLength(usize, f32),

    #[error("expected all tiles to be unique, found {0} duplicates: {1}")]
    DuplicateTiles(usize, String),
}
