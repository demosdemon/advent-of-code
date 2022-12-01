use euclid::point2;

use super::Coordinate;

#[derive(Debug, PartialEq, Clone)]
pub struct Line(pub Coordinate, pub Coordinate);

impl Line {
    pub fn angle(&self) -> usize {
        ((self.1.y as f32) - (self.0.y as f32))
            .atan2((self.1.x as f32) - (self.0.x as f32))
            .to_degrees()
            .abs()
            .trunc() as usize
    }

    pub fn is_diagonal(&self) -> bool {
        (self.angle() % 90) == 45
    }

    fn is_valid(&self) -> bool {
        self.0 != self.1 && (self.angle() % 45) == 0
    }

    fn incr(self) -> Line {
        assert!(self.is_valid());
        let a = self.0.cast::<isize>();
        let b = self.1.cast::<isize>();
        let dx = (b.x - a.x).signum();
        let dy = (b.y - a.y).signum();
        debug_assert!(!(dx == 0 && dy == 0));
        Line(point2(a.x + dx, a.y + dy).cast(), b.cast())
    }
}

impl IntoIterator for Line {
    type IntoIter = LineIterator;
    type Item = Line;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator(Some(self))
    }
}

pub struct LineIterator(Option<Line>);

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
