use std::ops::Add;
use std::ops::Sub;

pub(super) const MAX_ROTATIONS: usize = 24;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
#[display(fmt = "{x},{y},{z}")]
pub(super) struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

impl Coordinate {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Coordinate { x, y, z }
    }

    pub fn abs(&self) -> usize {
        let Coordinate { x, y, z } = *self;
        (x.abs() + y.abs() + z.abs()) as usize
    }

    pub fn rotate(&self, angle: usize) -> Coordinate {
        let Coordinate { x, y, z } = *self;
        match angle {
            0 => Coordinate::new(x, y, z),
            1 => Coordinate::new(x, z, -y),
            2 => Coordinate::new(x, -y, -z),
            3 => Coordinate::new(x, -z, y),
            4 => Coordinate::new(y, x, -z),
            5 => Coordinate::new(y, z, x),
            6 => Coordinate::new(y, -x, z),
            7 => Coordinate::new(y, -z, -x),
            8 => Coordinate::new(z, x, y),
            9 => Coordinate::new(z, y, -x),
            10 => Coordinate::new(z, -x, -y),
            11 => Coordinate::new(z, -y, x),
            12 => Coordinate::new(-x, y, -z),
            13 => Coordinate::new(-x, z, y),
            14 => Coordinate::new(-x, -y, z),
            15 => Coordinate::new(-x, -z, -y),
            16 => Coordinate::new(-y, x, z),
            17 => Coordinate::new(-y, z, -x),
            18 => Coordinate::new(-y, -x, -z),
            19 => Coordinate::new(-y, -z, x),
            20 => Coordinate::new(-z, x, -y),
            21 => Coordinate::new(-z, y, x),
            22 => Coordinate::new(-z, -x, y),
            23 => Coordinate::new(-z, -y, -x),
            _ => unreachable!(),
        }
    }
}

impl<'lhs, 'rhs> Sub<&'rhs Coordinate> for &'lhs Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: &'rhs Coordinate) -> Self::Output {
        Coordinate::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<'lhs, 'rhs> Add<&'rhs Coordinate> for &'lhs Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: &'rhs Coordinate) -> Self::Output {
        Coordinate::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

::aoc::derive_FromStr_for_nom!(Coordinate, super::parser::beacon);
