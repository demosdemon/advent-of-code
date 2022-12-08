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

use euclid::point2;

use super::Coordinate;

#[derive(Debug, PartialEq, Clone)]
pub(super) struct Line(pub Coordinate, pub Coordinate);

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
