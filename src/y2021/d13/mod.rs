pub(crate) mod part1;
pub(crate) mod part2;

use std::collections::BTreeSet;
use std::fmt::{Display, Write};
use std::ops::Add;
use std::str::FromStr;

use anyhow::{anyhow, Context, Error};

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
            .with_context(|| format!("splitting {} on `,`", s))?;
        Ok(Self(
            a.parse()
                .with_context(|| format!("parsing {} into an int", a))?,
            b.parse()
                .with_context(|| format!("parsing {} into an int", b))?,
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
            .with_context(|| format!("splitting {} on `=`", s))?;
        let b = b
            .parse()
            .with_context(|| format!("parsing {} into an int", b))?;
        match a {
            "fold along x" => Ok(Fold::X(b)),
            "fold along y" => Ok(Fold::Y(b)),
            _ => Err(anyhow!("invalid fold instructions: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct Instructions {
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
            let line = lines.next().context("reading coordinates from input")?;
            if line.is_empty() {
                break;
            }
            coordinates.push(line.parse::<Coordinate>()?);
        }
        Ok(Self {
            coordinates,
            folds: lines.map(str::parse).collect::<Result<_, _>>()?,
        })
    }
}

struct Matrix(BTreeSet<Coordinate>);

::aoc::derive_FromIterator!(Matrix, Coordinate);

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

impl<'a> FromIterator<&'a Coordinate> for Matrix {
    fn from_iter<T: IntoIterator<Item = &'a Coordinate>>(iter: T) -> Self {
        iter.into_iter().cloned().collect()
    }
}
