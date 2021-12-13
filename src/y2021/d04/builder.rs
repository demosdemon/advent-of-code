use itertools::Itertools;

use crate::{Error, ParseProblem, Problem};

use super::matrix::{Board, Tile};

#[derive(Debug)]
pub(super) struct SolutionBuilder {
    pub pulls: Vec<u8>,

    pub boards: Vec<Board>,
}

struct BoardReader<'a, 'b>(&'b mut Problem<'a>);

impl<'a, 'b> Iterator for BoardReader<'a, 'b> {
    type Item = Result<Board, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut tiles: Vec<Tile> = Vec::new();
        loop {
            match self.0.take_line() {
                None => break,
                Some(line) if line.is_empty() => break,
                Some(line) => {
                    for atom in line.split_ascii_whitespace() {
                        match atom.parse() {
                            Ok(v) => tiles.push(v),
                            Err(err) => return Some(Err(Error::from_parse(err))),
                        }
                    }
                }
            }
        }

        (!tiles.is_empty()).then(|| Board::new(tiles))
    }
}

impl ParseProblem for SolutionBuilder {
    type Error = Error;

    fn parse_problem(problem: &mut Problem<'_>) -> Result<Self, Self::Error> {
        let pulls: Vec<u8> = problem.expect_map_line(",", str::parse)?;
        problem.expect_empty_line()?;

        Ok(Self {
            pulls,
            boards: BoardReader(problem).try_collect()?,
        })
    }
}
