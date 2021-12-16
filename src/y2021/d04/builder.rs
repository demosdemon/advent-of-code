use std::str::FromStr;

use itertools::Itertools;

use super::matrix::{Board, Tile};
use super::Error;

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
    type Item = Result<Board, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut tiles: Vec<Tile> = Vec::new();
        loop {
            match self.0.next() {
                None => break,
                Some(l) if l.as_ref().is_empty() => break,
                Some(l) => {
                    for atom in l.as_ref().split_ascii_whitespace() {
                        match atom.parse() {
                            Ok(tile) => tiles.push(tile),
                            Err(err) => {
                                return Some(Err(Error::BoardInt(l.as_ref().to_owned(), err)))
                            }
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
        let pulls = lines.next().ok_or(Error::EndOfInput)?;
        let pulls = pulls
            .split(',')
            .map(str::parse)
            .try_collect()
            .map_err(|e| Error::ParsePulls(pulls.to_owned(), e))?;
        lines
            .next()
            .ok_or(Error::EndOfInput)
            .and_then(Error::from_empty_line)?;
        Ok(Self {
            pulls,
            boards: BoardReader(lines).try_collect()?,
        })
    }
}
