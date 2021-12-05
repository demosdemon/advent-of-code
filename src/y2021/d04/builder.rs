use std::io::BufRead;

use super::matrix::{Board, Tile};

#[derive(Debug)]
pub struct SolutionBuilder {
    pub pulls: Vec<u8>,

    pub boards: Vec<Board>,
}
struct BoardReader<R: BufRead>(crate::Problem<R>);

impl<R: BufRead> Iterator for BoardReader<R> {
    type Item = Result<Board, crate::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut tiles: Vec<Tile> = Vec::new();
        loop {
            match self.0.take_line() {
                None => break,
                Some(Ok(line)) if line.is_empty() => break,
                Some(Ok(line)) => {
                    for atom in line.split_ascii_whitespace() {
                        match atom.parse() {
                            Ok(v) => tiles.push(v),
                            Err(err) => return Some(Err(crate::Error::from_parse(err))),
                        }
                    }
                }
                Some(Err(err)) => return Some(Err(err)),
            }
        }

        if tiles.is_empty() {
            None
        } else {
            Some(Board::new(tiles))
        }
    }
}

impl<R: BufRead> TryFrom<crate::Problem<R>> for SolutionBuilder {
    type Error = crate::Error;

    fn try_from(mut value: crate::Problem<R>) -> Result<Self, Self::Error> {
        let pulls: Vec<u8> = value.expect_map_line(",", str::parse)?;
        value.expect_empty_line()?;

        Ok(Self {
            pulls,
            boards: BoardReader(value).collect::<Result<_, _>>()?,
        })
    }
}
