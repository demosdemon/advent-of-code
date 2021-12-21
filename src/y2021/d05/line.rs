use super::coordinate::Coordinate;

#[derive(Debug, PartialEq, Clone, derive_more::Display)]
#[display(fmt = "{} -> {}", _0, _1)]
pub(super) struct Line(pub Coordinate, pub Coordinate);

impl Line {
    pub fn angle(&self) -> usize {
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

pub(super) struct LineIterator(Option<Line>);

::aoc::derive_FromStr_for_nom!(Line, super::parser::line);

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
