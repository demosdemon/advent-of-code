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
