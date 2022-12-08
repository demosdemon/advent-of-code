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

use std::collections::BTreeMap;

use super::line::Line;
use super::Coordinate;

#[derive(Debug)]
pub struct Board(BTreeMap<(isize, isize), usize>);

impl Board {
    pub fn overlaps(&self) -> usize {
        self.0.iter().filter(|&(_, &v)| v >= 2).count()
    }
}

impl FromIterator<Coordinate> for Board {
    fn from_iter<T: IntoIterator<Item = Coordinate>>(iter: T) -> Self {
        Self(iter.into_iter().fold(BTreeMap::new(), |mut map, c| {
            *map.entry(c.cast::<isize>().into()).or_default() += 1;
            map
        }))
    }
}

impl FromIterator<Line> for Board {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        iter.into_iter()
            .flat_map(|l| l.into_iter().map(|l| l.0))
            .collect()
    }
}
