// Copyright (c) 2021-2022 Brandon LeBlanc <brandon@leblanc.codes>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub(crate) mod part1;
pub(crate) mod part2;

use std::collections::BTreeSet;
use std::fmt::Display;
use std::fmt::Write;
use std::ops::Add;
use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Error;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, derive_more::Display)]
#[display(fmt = "{_0},{_1}\n")]
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
            .with_context(|| format!("splitting {s} on `,`"))?;
        Ok(Self(
            a.parse()
                .with_context(|| format!("parsing {a} into an int"))?,
            b.parse()
                .with_context(|| format!("parsing {b} into an int"))?,
        ))
    }
}

#[derive(Debug, derive_more::Display)]
enum Fold {
    #[display(fmt = "fold along x={_0}\n")]
    X(usize),

    #[display(fmt = "fold along y={_0}\n")]
    Y(usize),
}

impl FromStr for Fold {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once('=')
            .with_context(|| format!("splitting {s} on `=`"))?;
        let b = b
            .parse()
            .with_context(|| format!("parsing {b} into an int"))?;
        match a {
            "fold along x" => Ok(Fold::X(b)),
            "fold along y" => Ok(Fold::Y(b)),
            _ => Err(anyhow!("invalid fold instructions: {}", s)),
        }
    }
}

#[derive(Debug, macros::TryFromStr)]
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

#[derive(macros::FromIterator)]
#[from_iterator(Coordinate)]
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
