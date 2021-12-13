use std::collections::BTreeSet;
use std::fmt::{Display, Write};
use std::ops::Add;
use std::str::FromStr;

use itertools::Itertools;

mod part1;
mod part2;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("coordinate missing comma; got {0}")]
    CoordinateMissingComma(String),

    #[error("unable to parse coordinate integer; got {1}")]
    CoordinateParseInt(#[source] std::num::ParseIntError, String),

    #[error("fold instructions missing an equal sign; got {0}")]
    FoldMissingEqual(String),

    #[error("unable to parse integer in fold instructions; got {1}")]
    FoldParseInt(#[source] std::num::ParseIntError, String),

    #[error("fold instruction did not match known pattern; got {0}")]
    FoldParsePattern(String),

    #[error("unexpected end of input")]
    EndOfInput,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
#[display(fmt = "{},{}\n", _0, _1)]
struct Coordinate(usize, usize);

impl Coordinate {
    fn fold(&self, f: &Fold) -> Self {
        match *f {
            Fold::X(v) => self.fold_x(v),
            Fold::Y(v) => self.fold_y(v),
        }
    }

    fn fold_x(&self, idx: usize) -> Self {
        Self(
            if self.0 < idx {
                self.0
            } else {
                idx - (self.0 - idx)
            },
            self.1,
        )
    }

    fn fold_y(&self, idx: usize) -> Self {
        Self(
            self.0,
            if self.1 < idx {
                self.1
            } else {
                idx - (self.1 - idx)
            },
        )
    }
}

impl<'a> Add<&'a Fold> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: &'a Fold) -> Self::Output {
        self.fold(rhs)
    }
}

impl FromStr for Coordinate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once(',')
            .ok_or_else(|| Error::CoordinateMissingComma(s.to_owned()))?;
        Ok(Self(
            a.parse()
                .map_err(|e| Error::CoordinateParseInt(e, s.to_owned()))?,
            b.parse()
                .map_err(|e| Error::CoordinateParseInt(e, s.to_owned()))?,
        ))
    }
}

#[derive(Debug, derive_more::Display)]
enum Fold {
    #[display(fmt = "fold along x={}\n", _0)]
    X(usize),

    #[display(fmt = "fold along y={}\n", _0)]
    Y(usize),
}

impl FromStr for Fold {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once('=')
            .ok_or_else(|| Error::FoldMissingEqual(s.to_owned()))?;
        let b = b
            .parse()
            .map_err(|e| Error::FoldParseInt(e, s.to_owned()))?;
        match a {
            "fold along x" => Ok(Fold::X(b)),
            "fold along y" => Ok(Fold::Y(b)),
            _ => Err(Error::FoldParsePattern(s.to_string())),
        }
    }
}

#[derive(Debug)]
struct Instructions {
    coordinates: Vec<Coordinate>,
    folds: Vec<Fold>,
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in self.coordinates.iter() {
            v.fmt(f)?;
        }
        f.write_str("\n")?;
        for v in self.folds.iter() {
            v.fmt(f)?;
        }
        Ok(())
    }
}

impl FromStr for Instructions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut coordinates = Vec::new();
        loop {
            let line = lines.next().ok_or(Error::EndOfInput)?;
            if line.is_empty() {
                break;
            }
            coordinates.push(line.parse::<Coordinate>()?);
        }
        Ok(Self {
            coordinates,
            folds: lines.map(str::parse).try_collect()?,
        })
    }
}

struct Matrix(BTreeSet<Coordinate>);

impl Matrix {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = self.0.iter().fold((0, 0), |p, n| {
            (std::cmp::max(p.0, n.0), std::cmp::max(p.1, n.1))
        });
        let mut v = Vec::new();
        v.resize((max_x + 1) * (max_y + 1), '.');
        for c in self.0.iter() {
            let idx = (c.1 * max_x) + c.0;
            v[idx] = '#';
        }
        for y in 0..=max_y {
            for x in 0..=max_x {
                let idx = (y * max_x) + x;
                f.write_char(v[idx])?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl<'a> Add<&'a Fold> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: &'a Fold) -> Self::Output {
        self.0.iter().map(|c| c.fold(rhs)).collect()
    }
}

impl FromIterator<Coordinate> for Matrix {
    fn from_iter<T: IntoIterator<Item = Coordinate>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> FromIterator<&'a Coordinate> for Matrix {
    fn from_iter<T: IntoIterator<Item = &'a Coordinate>>(iter: T) -> Self {
        iter.into_iter().cloned().collect()
    }
}
