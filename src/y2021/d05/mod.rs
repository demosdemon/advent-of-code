pub mod part1;
pub mod part2;

mod parser;

use std::io::BufRead;
use std::ops::Add;
use std::str::FromStr;

use nom::Finish;

use crate::errors::Error;
use crate::problem::Problem;

#[derive(Debug, derive_more::IntoIterator)]
#[into_iterator(owned, ref)]
pub struct SolutionBuilder(Vec<Line>);

impl SolutionBuilder {
    fn max_x(&self) -> i32 {
        self.into_iter().map(Line::max_x).max().unwrap_or(0)
    }

    fn max_y(&self) -> i32 {
        self.into_iter().map(Line::max_y).max().unwrap_or(0)
    }
}

impl<R: BufRead> TryFrom<Problem<R>> for SolutionBuilder {
    type Error = Error;

    fn try_from(value: Problem<R>) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .parse_lines(str::parse::<Line>)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

#[derive(Debug, PartialEq, Clone, derive_more::Display)]
#[display(fmt = "{} -> {}", _0, _1)]
pub struct Line(Coordinate, Coordinate);

impl Line {
    pub fn x(&self) -> (i32, i32) {
        (self.0.x, self.1.x)
    }

    pub fn y(&self) -> (i32, i32) {
        (self.0.y, self.1.y)
    }

    fn max_x(&self) -> i32 {
        std::cmp::max(self.0.x, self.1.x)
    }

    fn max_y(&self) -> i32 {
        std::cmp::max(self.0.y, self.1.y)
    }

    pub fn angle(&self) -> i32 {
        self.0.angle(&self.1)
    }

    pub fn is_diagonal(&self) -> bool {
        (self.angle() % 90) == 45
    }

    fn is_valid(&self) -> bool {
        self.0 != self.1 && (self.angle() % 45) == 0
    }

    pub fn incr(&self) -> Line {
        assert!(self.is_valid());
        let a = &self.0;
        let b = &self.1;
        if a.x == b.x {
            if a.y < b.y {
                Line(a.incr_y(), b.clone())
            } else {
                Line(a.decr_y(), b.clone())
            }
        } else if a.y == b.y {
            if a.x < b.x {
                Line(a.incr_x(), b.clone())
            } else {
                Line(a.decr_x(), b.clone())
            }
        } else {
            Line(a + b, b.clone())
        }
    }
}

impl FromStr for Line {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parser::line(s).finish() {
            Ok((_, v)) => Ok(v),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_owned(),
                code,
            }),
        }
    }
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, derive_more::Display, derive_more::Constructor,
)]
#[display(fmt = "{},{}", x, y)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl<'a> Add<&'a Coordinate> for &'a Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: &'a Coordinate) -> Self::Output {
        let dx = (rhs.x - self.x).signum();
        let dy = (rhs.y - self.y).signum();
        assert!(!(dx == 0 && dy == 0));
        Coordinate::new(self.x + dx, self.y + dy)
    }
}

impl Coordinate {
    pub fn angle(&self, rhs: &Coordinate) -> i32 {
        ((rhs.y as f64) - (self.y as f64))
            .atan2((rhs.x as f64) - (self.x as f64))
            .to_degrees()
            .abs() as i32
    }

    pub fn incr_x(&self) -> Coordinate {
        Coordinate::new(self.x + 1, self.y)
    }

    pub fn incr_y(&self) -> Coordinate {
        Coordinate::new(self.x, self.y + 1)
    }

    pub fn decr_x(&self) -> Coordinate {
        Coordinate::new(self.x - 1, self.y)
    }

    pub fn decr_y(&self) -> Coordinate {
        Coordinate::new(self.x, self.y - 1)
    }
}

impl FromStr for Coordinate {
    type Err = nom::error::Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parser::coordinate(s).finish() {
            Ok((_, v)) => Ok(v),
            Err(nom::error::Error { input, code }) => Err(nom::error::Error {
                input: input.to_owned(),
                code,
            }),
        }
    }
}
