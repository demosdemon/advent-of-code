use std::ops::Add;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, derive_more::Display, derive_more::Constructor,
)]
#[display(fmt = "{},{}", x, y)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl From<Coordinate> for (isize, isize) {
    fn from(c: Coordinate) -> Self {
        (c.x, c.y)
    }
}

::aoc::derive_FromStr_for_nom!(Coordinate, super::parser::coordinate);

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
    pub fn angle(&self, rhs: &Coordinate) -> usize {
        ((rhs.y as f32) - (self.y as f32))
            .atan2((rhs.x as f32) - (self.x as f32))
            .to_degrees()
            .abs() as usize
    }
}
