mod part1;
mod part2;

mod parser;

use std::ops::Add;

#[derive(Debug, derive_more::IntoIterator)]
#[into_iterator(owned, ref)]
struct SolutionBuilder(Vec<Line>);

crate::derive_FromIterator!(SolutionBuilder, Line);
crate::derive_FromStr_for_FromIterator!(SolutionBuilder, Line);

impl SolutionBuilder {
    fn max_x(&self) -> i64 {
        self.into_iter().map(Line::max_x).max().unwrap_or(0)
    }

    fn max_y(&self) -> i64 {
        self.into_iter().map(Line::max_y).max().unwrap_or(0)
    }
}

#[derive(Debug, PartialEq, Clone, derive_more::Display)]
#[display(fmt = "{} -> {}", _0, _1)]
struct Line(Coordinate, Coordinate);

impl Line {
    fn max_x(&self) -> i64 {
        std::cmp::max(self.0.x, self.1.x)
    }

    fn max_y(&self) -> i64 {
        std::cmp::max(self.0.y, self.1.y)
    }

    pub fn angle(&self) -> i64 {
        self.0.angle(&self.1)
    }

    pub fn is_diagonal(&self) -> bool {
        (self.angle() % 90) == 45
    }

    fn is_valid(&self) -> bool {
        self.0 != self.1 && (self.angle() % 45) == 0
    }

    fn incr(self) -> Line {
        assert!(self.is_valid());
        let a = &self.0;
        let b = &self.1;
        Line(a + b, b.clone())
    }
}

impl IntoIterator for Line {
    type Item = Line;

    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator(Some(self))
    }
}

struct LineIterator(Option<Line>);

crate::derive_FromStr_for_nom!(Line, parser::line);

impl Iterator for LineIterator {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.take() {
            Some(line) => {
                if line.is_valid() {
                    self.0.replace(line.clone().incr());
                }
                Some(line)
            }
            None => None,
        }
    }
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, derive_more::Display, derive_more::Constructor,
)]
#[display(fmt = "{},{}", x, y)]
struct Coordinate {
    x: i64,
    y: i64,
}

crate::derive_FromStr_for_nom!(Coordinate, parser::coordinate);

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
    pub fn angle(&self, rhs: &Coordinate) -> i64 {
        ((rhs.y as f64) - (self.y as f64))
            .atan2((rhs.x as f64) - (self.x as f64))
            .to_degrees()
            .abs() as i64
    }
}
