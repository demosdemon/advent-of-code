use std::str::FromStr;

use itertools::Itertools;

use anyhow::{Context, Error, Result};

use super::matrix::{Board, Tile};

#[derive(Debug)]
pub(super) struct SolutionBuilder {
    pub pulls: Vec<u8>,

    pub boards: Vec<Board>,
}

struct BoardReader<I>(I);

impl<I, V> Iterator for BoardReader<I>
where
    I: Iterator<Item = V>,
    V: AsRef<str>,
{
    type Item = Result<Board>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut tiles: Vec<Tile> = Vec::new();
        loop {
            match self.0.next() {
                None => break,
                Some(l) if l.as_ref().is_empty() => break,
                Some(l) => {
                    for atom in l.as_ref().split_ascii_whitespace() {
                        match atom.parse().context("parsing a tile") {
                            Ok(tile) => tiles.push(tile),
                            Err(err) => return Some(Err(err)),
                        }
                    }
                }
            }
        }

        (!tiles.is_empty()).then(|| Board::new(tiles))
    }
}

impl FromStr for SolutionBuilder {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let pulls = lines.next().context("reading pulls line")?;
        let pulls = pulls
            .split(',')
            .map(str::parse)
            .try_collect()
            .context("parsing pulls")?;
        lines
            .next()
            .context("reading empty line separator")
            .and_then(crate::expect_empty_line)?;
        Ok(Self {
            pulls,
            boards: BoardReader(lines).try_collect()?,
        })
    }
}
