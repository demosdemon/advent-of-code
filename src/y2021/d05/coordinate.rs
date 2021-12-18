use std::ops::Add;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, derive_more::Display, derive_more::Constructor,
)]
#[display(fmt = "{},{}", x, y)]
pub(super) struct Coordinate {
    pub x: i64,
    pub y: i64,
}

impl From<Coordinate> for (i64, i64) {
    fn from(c: Coordinate) -> Self {
        (c.x, c.y)
    }
}

crate::derive_FromStr_for_nom!(Coordinate, super::parser::coordinate);

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
